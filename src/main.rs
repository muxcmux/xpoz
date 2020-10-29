mod expect;
mod settings;
mod errors;

use anyhow::Result;
use actix_web::{dev, web, post, get, http, App, Error, HttpServer, HttpResponse, Result as AWResult};
use actix_web::middleware::{Compress, Logger, errhandlers::{ErrorHandlers, ErrorHandlerResponse}};
use std::env::args;
use settings::Settings;
use expect::*;
use errors::*;
use sqlx::{query, sqlite::SqlitePool};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let cfg = configure().await;
    run(cfg.0, cfg.1).await?;

    Ok(())
}

fn handle_404s<T>(mut _res: dev::ServiceResponse<T>) -> AWResult<ErrorHandlerResponse<T>> {
    Err(Error::from(APIError::NotFound))
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







use async_graphql::{EmptyMutation, EmptySubscription, Schema, Object};
use async_graphql_actix_web::{Request, Response};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[post("/graphql")]
async fn graphql(schema: web::Data<MySchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

#[get("/graphql")]
async fn index_playground() -> AWResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}



async fn run(settings: Settings, pool: SqlitePool) -> Result<()> {
    let address = settings.server.address.clone();
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    HttpServer::new(move || {
        App::new()
            .data(settings.clone())
            .data(pool.clone())
            .data(schema.clone())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, handle_404s)) .service(index)
            .service(show)
            .service(graphql)
            .service(index_playground)
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}











use serde::Serialize;

#[derive(Debug, Serialize)]
struct Entity {
    id: Option<i32>,
    name: Option<String>,
    parent_id: Option<i32>
}

#[get("/")]
async fn index(pool: web::Data<SqlitePool>) -> Result<HttpResponse, APIError> {
    let entities = get_entities(pool.get_ref()).await?;
    if entities.is_empty() {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::Ok().json(entities))
    }
}

#[get("/entities/{id}")]
async fn show(_pool: web::Data<SqlitePool>, id: web::Path<i32>) -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::Ok().json(Entity {
        id: Some(id.into_inner()),
        name: Some("Hello".into()),
        parent_id: None
    }))
}

async fn create_entity(pool: web::Data<SqlitePool>) -> Result<HttpResponse> {
    Ok(HttpResponse::Created().json(Entity {
        id: Some(123),
        name: Some("This is an entity that was just created".into()),
        parent_id: None
    }))
}

// async fn get_entity(pool: &SqlitePool, id: usize) -> Result<Entity> {
//     let records = query!
// }

async fn get_entities(pool: &SqlitePool) -> Result<Vec<Entity>> {
    let mut entities = vec![];

    let records = query!("SELECT * FROM Z_PRIMARYKEY")
        .fetch_all(pool)
        .await?;

    for r in records {
        entities.push(Entity {
            id: r.Z_ENT,
            name: r.Z_NAME,
            parent_id: r.Z_SUPER
        });
    }

    Ok(entities)
}
