mod entities;

use async_graphql::{
    Object,
    Result,
    Context,
    Schema as AGSchema,
    EmptyMutation,
    EmptySubscription
};

use entities::{Entity, entities, entity};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns all available entities in the photos app
    async fn entities(&self, ctx: &Context<'_>) -> Result<Vec<Entity>> { entities(ctx).await }
    /// Gets a single entity by it's id
    async fn entity(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Entity>> { entity(ctx, id).await }
}

pub type Schema = AGSchema<QueryRoot, EmptyMutation, EmptySubscription>;

