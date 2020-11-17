pub mod entities;
pub mod assets;
mod albums;

use async_graphql::{
    Object,
    Result,
    Context,
    Error,
    Schema as AGSchema,
    EmptyMutation,
    EmptySubscription
};

use entities::{Entity, entities, entity};
use albums::{Album, album, my_albums};
use sqlx::SqlitePool;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Gets a single entity by it's id
    async fn entity(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Entity>> {
        entity(ctx.data::<SqlitePool>()?, id).await.map_err(Error::from)
    }

    /// Returns all available entities in the photos app
    async fn entities(&self, ctx: &Context<'_>) -> Result<Vec<Entity>> {
        entities(ctx.data::<SqlitePool>()?).await.map_err(Error::from)
    }

    /// Get an album by it's id
    async fn album(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Album>> {
        album(ctx.data::<SqlitePool>()?, ctx.data::<Vec<Entity>>()?, id).await.map_err(Error::from)
    }

    /// "My Albums" which have been xpozed, keeping the original Photos sorting
    async fn my_albums(&self, ctx: &Context<'_>) -> Result<Vec<Album>> {
        my_albums(ctx.data::<SqlitePool>()?, ctx.data::<Vec<Entity>>()?).await.map_err(Error::from)
    }
}

pub type Schema = AGSchema<QueryRoot, EmptyMutation, EmptySubscription>;

