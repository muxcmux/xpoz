use async_graphql::{Object, Context, Result as AGResult};
use sqlx::{query_as, sqlite::SqlitePool};
use sql_builder::prelude::*;
use anyhow::Result;
use super::{Entity, assets::{Asset, assets, assets_by_id}};

#[derive(sqlx::FromRow)]
pub struct Album {
    pub id: i32,
    uuid: String,
    title: Option<String>,
    entity_id: i32,
    items_count: i32,
    photos_count: i32,
    videos_count: i32,
    created_at: String,
    key_asset_id: Option<i32>,
    secondary_key_asset_id: Option<i32>,
    tertiery_key_asset_id: Option<i32>,
    custom_key_asset_id: Option<i32>
}

#[Object]
impl Album {
    async fn id(&self) -> &i32 { &self.id }
    async fn uuid(&self) -> &String { &self.uuid }
    async fn title(&self) -> &Option<String> { &self.title }
    async fn items_count(&self) -> &i32 { &self.items_count }
    async fn photos_count(&self) -> &i32 { &self.photos_count }
    async fn videos_count(&self) -> &i32 { &self.videos_count }
    async fn created_at(&self) -> &String { &self.created_at }

    async fn entity<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a Entity> {
        let cache = ctx.data::<Vec<Entity>>().expect("Couldn't load entity cache");
        cache.iter().find(|e| { e.id == self.entity_id })
    }

    async fn assets(&self,  ctx: &Context<'_>, page: i32) -> AGResult<Vec<Asset>> {
        let assets = assets(ctx.data::<SqlitePool>()?,
                            ctx.data::<Vec<Entity>>()?,
                            &self,
                            page * 20,
                            20).await?;
        Ok(assets)
    }

    async fn key_assets(&self, ctx: &Context<'_>) -> AGResult<Vec<Asset>> {
        let mut ids = vec![];
        let ordered_key_asset_ids = [
            self.custom_key_asset_id,
            self.key_asset_id,
            self.secondary_key_asset_id,
            self.tertiery_key_asset_id
        ];

        for id in ordered_key_asset_ids.iter() {
            if let Some(i) = id {
                ids.push(*i);
            }
        }

        let mut assets = assets_by_id(ctx.data::<SqlitePool>()?, &ids).await?;
        assets.sort_by(|a, b| {
            let a_pos = &ids.iter().position(|&s| { s == a.id });
            let b_pos = &ids.iter().position(|&s| { s == b.id });
            a_pos.unwrap().cmp(&b_pos.unwrap())
        });

        Ok(assets)
   }
}

fn base_select(entity: &Entity) -> SqlBuilder {
    let fields = [
        "Z_PK as id",
        "ZUUID as uuid",
        "ZTITLE as title",
        "Z_ENT as entity_id",
        "ZCACHEDCOUNT as items_count",
        "ZCACHEDPHOTOSCOUNT as photos_count",
        "ZCACHEDVIDEOSCOUNT as videos_count",
        "datetime(ZCREATIONDATE,'unixepoch','31 years','localtime') as created_at",
        "ZKEYASSET as key_asset_id",
        "ZSECONDARYKEYASSET as secondary_key_asset_id",
        "ZTERTIARYKEYASSET as tertiery_key_asset_id",
        "ZCUSTOMKEYASSET as custom_key_asset_id"
    ];

    let mut builder = SqlBuilder::select_from("ZGENERICALBUM");
    builder.fields(&fields)
        .and_where_eq("Z_ENT", entity.id)
        .and_where_is_not_null("ZTITLE")
        .and_where_lt("ZTRASHEDSTATE", 1)
        .and_where_gt("ZCACHEDCOUNT", 0)
        .order_asc("Z_FOK_PARENTFOLDER");

    builder
}

pub async fn album(pool: &SqlitePool, cache: &Vec<Entity>, uuid: &String) -> Result<Option<Album>> {
    let entity = cache.iter().find(|e| { e.name == "Album" });
    let mut select = base_select(entity.expect("Couldn't find an Album entity in the entity cache"));
    select.and_where("ZUUID = ?".bind(uuid));

    let result = query_as::<_, Album>(select.sql()?.as_str())
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub async fn my_albums(pool: &SqlitePool, cache: &Vec<Entity>, page: i32) -> Result<Vec<Album>> {
    let entity = cache.iter().find(|e| { e.name == "Album" });
    let mut select = base_select(entity.expect("Couldn't find an Album entity in the entity cache"));
    select.offset(page * 10).limit(10);
    let records = query_as::<_, Album>(select.sql()?.as_str())
        .fetch_all(pool)
        .await?;

    Ok(records)
}

