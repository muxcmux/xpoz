mod expect;
mod settings;
mod db;

use anyhow::Result;
use actix_web::{web, post, get, App, HttpServer, HttpResponse, Result as AWResult};
use actix_web::middleware::{Compress, Logger};
use std::env::args;
use settings::Settings;
use expect::*;
use sqlx::sqlite::SqlitePool;
use async_graphql::{Schema as AGSchema, EmptyMutation, EmptySubscription};
use async_graphql_actix_web::{Request, Response};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use db::{Schema, QueryRoot};


#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let cfg = configure().await;
    run(cfg.0, cfg.1).await?;

    Ok(())
}

async fn configure() -> (Settings, SqlitePool) {
    let config_file = args().nth(1)
        .unwrap_or_else(|| { Settings::default_file().to_string() });
    let settings = Settings::from_file(&config_file).expect_or_exit("Config error");
    log::debug!("{:?}", settings);
    let pool = SqlitePool::new(&format!("sqlite://{}", settings.photos.database))
        .await.expect_or_exit("Can't open photos database");
    (settings, pool)
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

async fn run(settings: Settings, pool: SqlitePool) -> Result<()> {
    let address = settings.server.address.clone();
    let schema = AGSchema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool.clone())
        .finish();
    HttpServer::new(move || {
        App::new()
            .data(settings.clone())
            .data(pool.clone())
            .data(schema.clone())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(api)
            .service(graphiql)
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}

