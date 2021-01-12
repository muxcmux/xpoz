use super::{Album, Entity};
use crate::settings::Settings;
use actix_files as fs;
use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};
use glob::{glob_with, MatchOptions};
use sql_builder::prelude::*;
use sqlx::{query_as, sqlite::SqlitePool};
use std::path::PathBuf;

#[derive(sqlx::FromRow)]
pub struct Asset {
    pub id: i32,
    uuid: String,
    entity_id: i32,
    created_at: String,
    height: i32,
    width: i32,
    latitude: f32,
    longitude: f32,
    directory: String,
    filename: String,
    duration: f32,
}

#[Object]
impl Asset {
    async fn id(&self) -> &String {
        &self.uuid
    }
    async fn created_at(&self) -> &String {
        &self.created_at
    }
    async fn height(&self) -> &i32 {
        &self.height
    }
    async fn width(&self) -> &i32 {
        &self.width
    }
    async fn latitude(&self) -> &f32 {
        &self.latitude
    }
    async fn longitude(&self) -> &f32 {
        &self.longitude
    }
    async fn duration(&self) -> &f32 {
        &self.duration
    }
    async fn is_video(&self) -> bool {
        &self.duration > &0f32
    }
    async fn entity<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a Entity> {
        let cache = ctx
            .data::<Vec<Entity>>()
            .expect("Failed getting entity cache");
        cache.iter().find(|e| e.id == self.entity_id)
    }
}

impl Asset {
    /// Returns the original file for the asset
    pub fn original(&self, settings: &Settings) -> Result<fs::NamedFile> {
        let mut dir = settings.photos.originals_dir();
        dir.push(self.directory.clone());
        dir.push(self.filename.clone());
        let file = fs::NamedFile::open(dir)?;
        Ok(file)
    }

    /// Returns the first found resized variant of the asset with latest edits
    pub fn resized(&self, settings: &Settings) -> Result<fs::NamedFile> {
        let mut path = settings.photos.resized_dir();
        self.first_in_path(&mut path)
    }

    /// Returns the first found original size asset with latest edits applied
    pub fn render(&self, settings: &Settings) -> Result<fs::NamedFile> {
        let mut path = settings.photos.renders_dir();
        self.first_in_path(&mut path)
    }

    /// Returns the first found thumb of the asset with latest edits applied
    pub fn thumb(&self, settings: &Settings) -> Result<fs::NamedFile> {
        let mut path = settings.photos.thumbs_dir();
        self.first_in_path(&mut path)
    }

    fn first_in_path(&self, path: &mut PathBuf) -> Result<fs::NamedFile> {
        let extensions = ["jpeg", "mov", "mp4", "jpg", "png", "gif"];

        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        path.push(self.directory.clone());

        let dir = path.to_str().expect("Failed converting a path to a string");

        for ext in extensions.iter() {
            let pattern = format!("{}/{}*.{}", dir, self.uuid, ext);
            for entry in glob_with(pattern.as_str(), options).expect("Failed to read glob pattern")
            {
                return Ok(fs::NamedFile::open(entry?)?);
            }
        }

        Err(anyhow!("Requested variant for this asset is not available"))
    }
}

fn album_join_tables(cache: &Vec<Entity>) -> (String, String, String, String) {
    let album = cache
        .iter()
        .find(|e| e.name == "Album")
        .expect("Couldn't find an 'Album' entity in the entity cache");
    let asset = cache
        .iter()
        .find(|e| e.name == "Asset")
        .expect("Couldn't find a 'Asset' entity in the entity cache");
    let join_table = format!("Z_{}ASSETS as joins", album.id);
    let album_fk = format!("Z_{}ALBUMS", album.id);
    let asset_fk = format!("Z_{}ASSETS", asset.id);
    let order_key = format!("Z_FOK_{}ASSETS", asset.id);
    (join_table, album_fk, asset_fk, order_key)
}

fn base_select() -> SqlBuilder {
    let fields = [
        "Z_PK as id",
        "ZUUID as uuid",
        "Z_ENT as entity_id",
        "datetime(ZDATECREATED,'unixepoch','31 years','localtime') as created_at",
        "ZHEIGHT as height",
        "ZWIDTH as width",
        "ZLATITUDE as latitude",
        "ZLONGITUDE as longitude",
        "ZDIRECTORY as directory",
        "ZFILENAME as filename",
        "ZDURATION as duration",
    ];

    let mut builder = SqlBuilder::select_from("ZASSET as assets");

    builder
        .fields(&fields)
        .and_where_lt("assets.ZTRASHEDSTATE", 1);

    builder
}

pub async fn asset(pool: &SqlitePool, uuid: &String) -> Result<Option<Asset>> {
    let mut select = base_select();
    select.and_where("ZUUID = ?".bind(uuid));

    let record = query_as::<_, Asset>(select.sql()?.as_str())
        .fetch_optional(pool)
        .await?;

    Ok(record)
}

pub async fn assets(
    pool: &SqlitePool,
    cache: &Vec<Entity>,
    album: &Album,
    offset: i32,
    limit: i32,
) -> Result<Vec<Asset>> {
    let joins = album_join_tables(cache);

    let mut select = base_select();

    select
        .join(joins.0)
        .on(format!("joins.{} = assets.Z_PK", joins.2))
        .and_where_eq(format!("joins.{}", joins.1), album.id)
        .offset(offset)
        .limit(limit)
        .order_asc(joins.3);

    let records = query_as::<_, Asset>(select.sql()?.as_str())
        .fetch_all(pool)
        .await?;

    Ok(records)
}

pub async fn assets_by_id(pool: &SqlitePool, ids: &Vec<i32>) -> Result<Vec<Asset>> {
    let mut select = base_select();
    select.and_where_in("Z_PK", ids);

    let records = query_as::<_, Asset>(select.sql()?.as_str())
        .fetch_all(pool)
        .await?;

    Ok(records)
}
