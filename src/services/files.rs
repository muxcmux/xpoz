use crate::db::{assets::asset, Databases};
use crate::settings::Settings;
use actix_files as fs;
use actix_web::{get, web, Either, HttpResponse};

#[get("/{variant}/{uuid}")]
async fn get_asset(
    web::Path((variant, uuid)): web::Path<(String, String)>,
    settings: web::Data<Settings>,
    dbs: web::Data<Databases>,
) -> Either<fs::NamedFile, HttpResponse> {
    let settings = settings.into_inner();
    let dbs = dbs.into_inner();
    if let Ok(Some(asset)) = asset(&dbs.photos, &uuid).await {
        let file = match variant.as_str() {
            "thumb" => asset.thumb(&settings),
            "render" => asset.render(&settings),
            "resized" => asset.resized(&settings),
            "video" => asset.video(&settings),
            _ => asset.original(&settings),
        };

        if let Ok(f) = file {
            return Either::A(f);
        }
    }

    Either::B(
        HttpResponse::NotFound()
            .header("cache-control", "no-cache, must-revalidate")
            .body("The requested asset was not found"),
    )
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_asset);
}
