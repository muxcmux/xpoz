use crate::auth::Access;
use crate::{db::Schema, settings::load_settings};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Result as AWResult};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};

#[post("/api")]
async fn api(schema: web::Data<Schema>, req: HttpRequest, gql_req: Request) -> Response {
    // std::thread::sleep_ms(500);
    let access = req
        .head()
        .extensions()
        .get::<Access>()
        .expect("Can't get api access info")
        .clone();
    let mut gql_request = gql_req.into_inner();
    gql_request = gql_request.data(access);
    schema.execute(gql_request).await.into()
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
    cfg.service(api);
    let settings = load_settings();
    if settings.server.graphiql {
        cfg.service(graphiql);
    }
}
