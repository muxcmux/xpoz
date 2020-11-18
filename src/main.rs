mod settings;
mod db;
mod ext;

use anyhow::{Result, anyhow};
use actix_files as fs;
use actix_web::{web, post, get, App, HttpServer, HttpRequest, HttpResponse, Result as AWResult, error::ErrorNotFound};
use actix_web::middleware::{Compress, Logger};
use actix_cors::Cors;
use std::env::args;
use settings::Settings;
use ext::ExpectExt;
use sqlx::sqlite::SqlitePool;
use async_graphql::{Schema as AGSchema, EmptyMutation, EmptySubscription};
use async_graphql_actix_web::{Request, Response};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use db::{Schema, QueryRoot, assets::asset, entities::{entities, Entity}};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let cfg = configure().await;
    run(cfg.0, cfg.1, cfg.2).await?;

    Ok(())
}

async fn configure() -> (Settings, SqlitePool, Vec<Entity>) {
    let config_file = args().nth(1)
        .unwrap_or_else(|| { Settings::default_file().to_string() });
    let settings = Settings::from_file(&config_file).expect_or_exit("Config error");
    log::debug!("{:?}", settings);
    let pool = SqlitePool::new(&format!("sqlite://{}", settings.photos.database))
        .await.expect_or_exit("Can't open photos database");
    let entities = entities(&pool).await.expect_or_exit("Can't load entities from db");
    (settings, pool, entities)
}


#[post("/api")]
async fn api(schema: web::Data<Schema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

#[get("/api")]
async fn graphiql() -> AWResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/api").subscription_endpoint("/api"),
        )))
}

#[get("/asset/{variant}/{uuid}")]
async fn get_asset(
    web::Path((variant, uuid)): web::Path<(String, String)>,
    settings: web::Data<Settings>,
    pool: web::Data<SqlitePool>
) -> AWResult<fs::NamedFile> {
    let settings = settings.into_inner();
    let pool = pool.into_inner();
    match asset(&pool, &uuid).await.map_err(|e| ErrorNotFound(e))? {
        Some(asset) => {
            let file = match variant.as_str() {
                "thumb" => asset.thumb(&settings),
                "render" => asset.render(&settings),
                "resized" => asset.resized(&settings),
                _ => asset.original(&settings),
            };

            Ok(file.map_err(|e| ErrorNotFound(e))?)
        },
        None => Err(ErrorNotFound(anyhow!("The requested asset was not found")))
    }
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
            .service(api)
            .service(graphiql)
            .service(get_asset)
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}

