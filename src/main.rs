mod auth;
mod db;
mod services;
mod settings;

use actix_session::CookieSession;
use actix_web::middleware::{Compress, DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use async_graphql::{EmptyMutation, EmptySubscription, Schema as AGSchema};
use auth::Auth;
use db::{
    build_pool,
    Databases,
    entities::{entities, Entity},
    QueryRoot,
};
use settings::{load_settings, Settings};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let cfg = configure().await;
    run(cfg.0, cfg.1, cfg.2).await?;

    Ok(())
}

async fn configure() -> (Settings, Databases, Vec<Entity>) {
    let settings = load_settings();
    log::debug!("{:?}", settings);
    let photos_pool = build_pool(&settings.photos.database_url()).await;
    let auth_pool = build_pool(&settings.auth.database_url()).await;
    let entities = entities(&photos_pool).await.expect("Can't load entities from db");
    let dbs = Databases {
        photos: photos_pool,
        auth: auth_pool,
    };
    (settings, dbs, entities)
}

async fn run(settings: Settings, dbs: Databases, entity_cache: Vec<Entity>) -> Result<()> {
    let address = settings.server.address.clone();
    let schema = AGSchema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(dbs.photos.clone())
        .data(entity_cache)
        .finish();
    HttpServer::new(move || {
        let session = CookieSession::signed(&[0; 32])
            .secure(false)
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
                actix_files::Files::new("/", &settings.server.public_dir).index_file("index.html"),
            )
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
