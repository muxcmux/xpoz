use async_graphql::{Object, Context, Result as AGResult};
use sqlx::{query_as, sqlite::{SqlitePool, SqliteQueryAs}};
use sql_builder::SqlBuilder;
use crate::ext::SqlBuilderExt;
use anyhow::Result;
use super::{Entity, assets::{Asset, assets}};

#[derive(sqlx::FromRow)]
pub struct Album {
    pub id: i32,
    uuid: String,
    title: Option<String>,
    entity_id: i32,
    items_count: i32,
    photos_count: i32,
    videos_count: i32,
    created_at: String
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
    async fn assets(&self,  ctx: &Context<'_>) -> AGResult<Vec<Asset>> {
        let assets = assets(ctx.data::<SqlitePool>()?,
                            ctx.data::<Vec<Entity>>().expect("Couldn't load entity cache"),
                            &self).await?;
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
        "datetime(ZCREATIONDATE,'unixepoch','31 years','localtime') as created_at"
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

pub async fn album(pool: &SqlitePool, cache: &Vec<Entity>, id: i32) -> Result<Option<Album>> {
    let entity = cache.iter().find(|e| { e.name == "Album" });
    let mut select = base_select(entity.expect("Couldn't find an Album entity in the entity cache"));
    select.and_where_eq("Z_PK", id);

    let result = query_as::<_, Album>(select.sqld()?.as_str())
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub async fn my_albums(pool: &SqlitePool, cache: &Vec<Entity>) -> Result<Vec<Album>> {
    let entity = cache.iter().find(|e| { e.name == "Album" });
    let select = base_select(entity.expect("Couldn't find an Album entity in the entity cache"));
    let records = query_as::<_, Album>(select.sqld()?.as_str())
        .fetch_all(pool)
        .await?;

    Ok(records)
}

