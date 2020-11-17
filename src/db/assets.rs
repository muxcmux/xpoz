use async_graphql::{Object, Context};
use actix_files as fs;
use sqlx::{query_as, sqlite::{SqlitePool, SqliteQueryAs}};
use sql_builder::prelude::*;
use crate::ext::SqlBuilderExt;
use anyhow::Result;
use crate::settings::Settings;
use super::{Entity, Album};
use glob::glob;

#[derive(sqlx::FromRow)]
pub struct Asset {
    id: i32,
    uuid: String,
    entity_id: i32,
    created_at: String,
    height: i32,
    width: i32,
    latitude: f32,
    longitude: f32,
    directory: String,
    filename: String,
    duration: f32
}

#[Object]
impl Asset {
    async fn id(&self) -> &i32 { &self.id }
    async fn uuid(&self) -> &String { &self.uuid }
    async fn created_at(&self) -> &String { &self.created_at }
    async fn height(&self) -> &i32 { &self.height }
    async fn width(&self) -> &i32 { &self.width }
    async fn latitude(&self) -> &f32 { &self.latitude }
    async fn longitude(&self) -> &f32 { &self.longitude }
    async fn duration(&self) -> &f32 { &self.duration }
    async fn entity<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a Entity> {
        let cache = ctx.data::<Vec<Entity>>().expect("Failed getting entity cache");
        cache.iter().find(|e| { e.id == self.entity_id })
    }
}

impl Asset {
    pub fn original(&self, settings: &Settings) -> Result<fs::NamedFile> {
        let mut dir = settings.photos.originals_dir();
        dir.push(self.directory.clone());
        dir.push(self.filename.clone());
        let file = fs::NamedFile::open(dir)?;
        Ok(file)
    }

    // pub fn resized(&self, settings: &Settings) -> Result<fs::NamedFile> {
    //     let mut dir = settings.photos.resized_dir();
    //     dir.push(self.directory.clone().expect("Tried to get resized file for an asset without a directory"));
    //     dir.push(format!("{}*", self.uuid.clone().expect("Tried to get resized file for an asset without a uuid")));
    //     let glb = glob(dir.to_str().unwrap()).expect("Failed to read resized glob patter");

    // }
}

fn album_join_tables(cache: &Vec<Entity>) -> (String, String, String) {
    let album = cache.iter().find(|e| { e.name == "Album" })
        .expect("Couldn't find an 'Album' entity in the entity cache");
    let generic_asset = cache.iter().find(|e| { e.name == "GenericAsset" })
        .expect("Couldn't find a 'GenericAsset' entity in the entity cache");
    let join_table = format!("Z_{}ASSETS as joins", album.id);
    let album_fk = format!("Z_{}ALBUMS", album.id);
    let asset_fk = format!("Z_{}ASSETS", generic_asset.id);
    (join_table, album_fk, asset_fk)
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
        "ZDURATION as duration"
    ];

    let mut builder = SqlBuilder::select_from("ZGENERICASSET as assets");

    builder.fields(&fields)
        .and_where_lt("assets.ZTRASHEDSTATE", 1);

    builder
}

pub async fn asset(pool: &SqlitePool, uuid: &String) -> Result<Option<Asset>> {
    let mut select = base_select();
    select.and_where("ZUUID = ?".bind(uuid));

    let record = query_as::<_, Asset>(select.sqld()?.as_str())
        .fetch_optional(pool)
        .await?;

    Ok(record)
}

pub async fn assets(pool: &SqlitePool, cache: &Vec<Entity>, album: &Album) -> Result<Vec<Asset>> {
    let joins = album_join_tables(cache);

    let mut select = base_select();

    select.join(joins.0)
        .on(format!("joins.{} = assets.Z_PK", joins.2))
        .and_where_eq(format!("joins.{}", joins.1), album.id);

    let records = query_as::<_, Asset>(select.sqld()?.as_str())
        .fetch_all(pool)
        .await?;

    Ok(records)
}