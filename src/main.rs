mod db;
mod services;
mod settings;

use actix_cors::Cors;
use actix_web::middleware::{Compress, DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use async_graphql::{EmptyMutation, EmptySubscription, Schema as AGSchema};
use db::{
    entities::{entities, Entity},
    QueryRoot,
};
use settings::Settings;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env::args;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let cfg = configure().await;
    run(cfg.0, cfg.1, cfg.2).await?;

    Ok(())
}

async fn configure() -> (Settings, SqlitePool, Vec<Entity>) {
    let config_file = args()
        .nth(1)
        .unwrap_or_else(|| Settings::default_file().to_string());
    let settings = Settings::from_file(&config_file).expect("Config error");
    log::debug!("{:?}", settings);
    let pool = SqlitePoolOptions::new()
        .idle_timeout(std::time::Duration::new(5, 0))
        .max_connections(3)
        .connect(&settings.photos.database_url())
        .await
        .expect("Can't open photos database");
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
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
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
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
