mod albums;
pub mod assets;
pub mod entities;

use async_graphql::{
    Context, EmptyMutation, EmptySubscription, Error, Object, Result, Schema as AGSchema,
};

use albums::{album, my_albums, Album};
use entities::{entities, entity, Entity};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use crate::auth::Access;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Gets a single entity by it's id
    async fn entity(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Entity>> {
        entity(ctx.data::<SqlitePool>()?, id)
            .await
            .map_err(Error::from)
    }

    /// Returns all available entities in the photos app
    async fn entities(&self, ctx: &Context<'_>) -> Result<Vec<Entity>> {
        entities(ctx.data::<SqlitePool>()?)
            .await
            .map_err(Error::from)
    }

    /// Get an album by its uuid
    async fn album(&self, ctx: &Context<'_>, uuid: String) -> Result<Option<Album>> {
        album(ctx.data::<SqlitePool>()?, ctx.data::<Vec<Entity>>()?, &ctx.data::<Access>()?.whitelist(), &uuid)
            .await
            .map_err(Error::from)
    }

    /// "My Albums" which have been xpozed, keeping the original Photos sorting
    async fn my_albums(&self, ctx: &Context<'_>, page: i32) -> Result<Vec<Album>> {
        my_albums(ctx.data::<SqlitePool>()?, ctx.data::<Vec<Entity>>()?, &ctx.data::<Access>()?.whitelist(), page)
            .await
            .map_err(Error::from)
    }
}

pub type Schema = AGSchema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn build_pool(url: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .idle_timeout(std::time::Duration::new(5, 0))
        .max_connections(3)
        .connect(url)
        .await
        .expect(&format!("Can't open database {}", url))
}

#[derive(Clone)]
pub struct Databases {
    pub photos: SqlitePool,
    pub auth: SqlitePool
}
