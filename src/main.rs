mod auth;
mod db;
mod services;
mod settings;
mod transcoder;

use actix_session::CookieSession;
use actix_web::middleware::{Compress, DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use async_graphql::{EmptySubscription, Schema as AGSchema};
use auth::Auth;
use db::{
    build_pool,
    entities::{entities, Entity},
    migrate::migrate_database,
    Databases, MutationRoot, QueryRoot,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use settings::{load_settings, Settings};
use std::sync::Arc;
use transcoder::Transcoder;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let cfg = configure().await;

    let config = Arc::new(cfg.0.clone());

    if cfg.0.media.convert_videos {
        std::thread::spawn(move || Transcoder::new(config));
    }

    // This is blocking the main thread and unblocks on SIGINT
    // so there's no need to join the transcoder thread, since we
    // want it to stop when the server is stopped
    run(cfg.0, cfg.1, cfg.2).await?;

    Ok(())
}

async fn configure() -> (Settings, Databases, Vec<Entity>) {
    let settings = load_settings();
    log::debug!("{:?}", settings);
    migrate_database(&settings.app.database);
    let photos_pool = build_pool(&settings.photos.database_url()).await;
    let app_pool = build_pool(&settings.app.database_url()).await;
    let entities = entities(&photos_pool)
        .await
        .expect("Can't load entities from db");
    let dbs = Databases {
        photos: photos_pool,
        app: app_pool,
    };
    (settings, dbs, entities)
}

async fn run(settings: Settings, dbs: Databases, entity_cache: Vec<Entity>) -> Result<()> {
    let server_settings = settings.server.clone();
    let schema = AGSchema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(dbs.clone())
        .data(entity_cache)
        .finish();
    let server = HttpServer::new(move || {
        let session = CookieSession::signed(&[0; 32])
            .secure(settings.server.ssl)
            .expires_in(365 * 24 * 60 * 60);
        App::new()
            .data(settings.clone())
            .data(dbs.clone())
            .data(schema.clone())
            .wrap(Auth {})
            .wrap(session)
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(auth::auth)
            .service(
                web::scope("/asset")
                    .wrap(DefaultHeaders::new().header("cache-control", "max-age=86400"))
                    .configure(services::files::config),
            )
            .configure(services::graphql::config)
            .service(
                actix_files::Files::new("/", &settings.server.public_dir).index_file(&settings.server.index_file),
            )
    });

    if server_settings.ssl {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
            .expect("Failed initialising ssl builder");
        builder
            .set_private_key_file(&server_settings.key, SslFiletype::PEM)
            .expect("Can't set up private key file. Is your key file configured correctly?");
        builder
            .set_certificate_chain_file(&server_settings.cert)
            .expect("Can't set up certificate chain file. Is your cert file configured correctly?");
        let _ = server
            .bind_openssl(&server_settings.address, builder)?
            .run()
            .await;
    } else {
        let _ = server.bind(&server_settings.address)?.run().await;
    }

    Ok(())
}
