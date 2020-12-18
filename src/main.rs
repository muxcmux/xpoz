mod db;
mod services;
mod settings;

use actix_session::{CookieSession, Session};
use actix_web::middleware::{Compress, DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use async_graphql::{EmptyMutation, EmptySubscription, Schema as AGSchema};
use db::{
    entities::{entities, Entity},
    QueryRoot,
    build_pool
};
use settings::{Settings, load_settings};
use sqlx::sqlite::SqlitePool;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let cfg = configure().await;
    run(cfg.0, cfg.1, cfg.2).await?;

    Ok(())
}

async fn configure() -> (Settings, SqlitePool, Vec<Entity>) {
    let settings = load_settings();
    log::debug!("{:?}", settings);
    let pool = build_pool(&settings).await;
    let entities = entities(&pool).await.expect("Can't load entities from db");
    (settings, pool, entities)
}

async fn run(settings: Settings, pool: SqlitePool, entity_cache: Vec<Entity>) -> Result<()> {
    let address = settings.server.address.clone();
    let schema = AGSchema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool.clone())
        .data(entity_cache)
        .finish();
    HttpServer::new(move || {
        let session = CookieSession::signed(&[0; 32])
            .secure(false)
            .expires_in(365 * 24 * 60 * 60);
        App::new()
            .wrap(session)
            .data(settings.clone())
            .data(pool.clone())
            .data(schema.clone())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(
                web::scope("/asset")
                    .wrap(DefaultHeaders::new().header("cache-control", "max-age=86400"))
                    .configure(services::files::config),
            )
            .configure(services::graphql::config)
            .service(actix_files::Files::new("/", &settings.server.public_dir).index_file("index.html"))
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
