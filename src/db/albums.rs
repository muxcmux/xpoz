use async_graphql::{Object, Context};
use sqlx::{query_as, sqlite::{SqlitePool, SqliteQueryAs}};
use sql_builder::SqlBuilder;
use crate::ext::SqlBuilderExt;
use anyhow::Result;
use super::Entity;

#[derive(sqlx::FromRow)]
pub struct Album {
    id: Option<i32>,
    uuid: Option<String>,
    cloud_uuid: Option<String>,
    title: Option<String>,
    entity_id: Option<i32>,
    items_count: Option<i32>,
    photos_count: Option<i32>,
    videos_count: Option<i32>,
    created_at: Option<String>
}

#[Object]
impl Album {
    async fn id(&self) -> &Option<i32> { &self.id }
    async fn uuid(&self) -> &Option<String> { &self.uuid }
    async fn cloud_uuid(&self) -> &Option<String> { &self.cloud_uuid }
    async fn title(&self) -> &Option<String> { &self.title }
    async fn items_count(&self) -> &Option<i32> { &self.items_count }
    async fn photos_count(&self) -> &Option<i32> { &self.photos_count }
    async fn videos_count(&self) -> &Option<i32> { &self.videos_count }
    async fn created_at(&self) -> &Option<String> { &self.created_at }
    async fn entity<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a Entity> {
        let cache = ctx.data::<Vec<Entity>>().unwrap();
        cache.iter().find(|e| { e.id == self.entity_id })
    }
}

fn base_select(entity: &Entity) -> SqlBuilder {
    let fields = [
        "Z_PK as id",
        "ZUUID as uuid",
        "ZCLOUDGUID as cloud_uuid",
        "ZTITLE as title",
        "Z_ENT as entity_id",
        "ZCACHEDCOUNT as items_count",
        "ZCACHEDPHOTOSCOUNT as photos_count",
        "ZCACHEDVIDEOSCOUNT as videos_count",
        "datetime(ZCREATIONDATE,'unixepoch','31 years','localtime') as created_at"
    ];

    let mut builder = SqlBuilder::select_from("ZGENERICALBUM");
    builder.fields(&fields)
        .and_where_eq("Z_ENT", entity.id.unwrap())
        .and_where_is_not_null("ZTITLE")
        .and_where_lt("ZTRASHEDSTATE", 1)
        .and_where_gt("ZCACHEDCOUNT", 0)
        .order_asc("Z_FOK_PARENTFOLDER");

    builder
}

pub async fn album(pool: &SqlitePool, cache: &Vec<Entity>, id: i32) -> Result<Option<Album>> {
    let entity = cache.iter().find(|e| { e.name == Some("Album".into()) });
    let mut select = base_select(entity.unwrap());
    select.and_where_eq("Z_PK", id);

    let result = query_as::<_, Album>(select.sqld()?.as_str())
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub async fn my_albums(pool: &SqlitePool, cache: &Vec<Entity>) -> Result<Vec<Album>> {
    let entity = cache.iter().find(|e| { e.name == Some("Album".into()) });
    let select = base_select(entity.unwrap());
    let records = query_as::<_, Album>(select.sqld()?.as_str())
        .fetch_all(pool)
        .await?;

    Ok(records)
}

