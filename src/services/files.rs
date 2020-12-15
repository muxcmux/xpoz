use crate::db::assets::asset;
use crate::settings::Settings;
use actix_files as fs;
use actix_web::{error::ErrorNotFound, get, web, Result as AWResult};
use anyhow::anyhow;
use sqlx::sqlite::SqlitePool;

#[get("/{variant}/{uuid}")]
async fn get_asset(
    web::Path((variant, uuid)): web::Path<(String, String)>,
    settings: web::Data<Settings>,
    pool: web::Data<SqlitePool>,
) -> AWResult<fs::NamedFile> {
    // std::thread::sleep_ms(500);
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
        }
        None => Err(ErrorNotFound(anyhow!("The requested asset was not found"))),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_asset);
}
