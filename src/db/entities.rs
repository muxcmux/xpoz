use async_graphql::{Object, Context};
use anyhow::Result;
use sqlx::{query_as, sqlite::SqlitePool};

pub struct Entity {
    pub id: Option<i32>,
    pub name: Option<String>,
    parent_id: Option<i32>
}

#[Object]
impl Entity {
    async fn id(&self) -> &Option<i32> { &self.id }
    async fn name(&self) -> &Option<String> { &self.name }
    async fn parent_id(&self) -> &Option<i32> { &self.parent_id }
    async fn parent<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a Self> {
        let cache = ctx.data::<Vec<Entity>>().unwrap();
        cache.iter().find(|e| { e.id == self.parent_id })
    }
}

pub async fn entities(pool: &SqlitePool) -> Result<Vec<Entity>> {
    let records = query_as!(Entity, "SELECT Z_ENT as id, Z_NAME as name, Z_SUPER as parent_id FROM Z_PRIMARYKEY")
        .fetch_all(pool)
        .await?;

    Ok(records)
}

pub async fn entity(pool: &SqlitePool, id: i32) -> Result<Option<Entity>> {
    let result = query_as!(Entity, "SELECT Z_ENT as id, Z_NAME as name, Z_SUPER as parent_id FROM Z_PRIMARYKEY WHERE Z_ENT = ?", id)
        .fetch_optional(pool)
        .await?;

    Ok(result)
}
