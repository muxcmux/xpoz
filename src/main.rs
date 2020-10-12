mod expect;
mod settings;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, middleware::Logger };
use std::env::args;
use settings::Settings;
use expect::*;

const DEFAULT_SETTINGS_FILE: &str = "settings.yml";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_file = args().nth(1).unwrap_or_else(|| { DEFAULT_SETTINGS_FILE.to_string() });

    pretty_env_logger::init();

    run(&config_file).await
}

async fn run(config_file: &str) -> std::io::Result<()> {
    let settings = Settings::from_file(config_file).expect_or_exit("Config error");
    log::debug!("{:?}", settings);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(hello)
    })
    .bind(settings.server.bind_address)?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    let environment = std::env::vars_os()
        .map(|s| format!("{}={}", s.0.into_string().unwrap(), s.1.into_string().unwrap()))
        .collect::<Vec<String>>()
        .join("\n");

    HttpResponse::Ok().body(environment)
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey, whas up?")
}
