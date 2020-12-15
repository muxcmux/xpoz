use crate::db::Schema;
use actix_web::{get, post, web, HttpResponse, Result as AWResult};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};

#[post("/api")]
async fn api(schema: web::Data<Schema>, req: Request) -> Response {
    // std::thread::sleep_ms(500);
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(api).service(graphiql);
}
