use anyhow::Result;
use async_graphql::{Context, Object};
use sql_builder::prelude::*;
use sqlx::{query_as, sqlite::SqlitePool};

#[derive(sqlx::FromRow)]
pub struct Entity {
    pub id: i32,
    pub name: String,
    parent_id: Option<i32>,
}

#[Object]
impl Entity {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn parent_id(&self) -> &Option<i32> {
        &self.parent_id
    }
    async fn parent<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a Self> {
        let cache = ctx
            .data::<Vec<Entity>>()
            .expect("Couldn't get entity cache");
        cache.iter().find(|e| Some(e.id) == self.parent_id)
    }
}

fn base_select() -> SqlBuilder {
    let fields = ["Z_ENT as id", "Z_NAME as name", "Z_SUPER as parent_id"];

    let mut builder = SqlBuilder::select_from("Z_PRIMARYKEY");
    builder.fields(&fields);

    builder
}

pub async fn entities(pool: &SqlitePool) -> Result<Vec<Entity>> {
    let select = base_select();
    let records = query_as::<_, Entity>(select.sql()?.as_str())
        .fetch_all(pool)
        .await?;

    Ok(records)
}

pub async fn entity(pool: &SqlitePool, id: i32) -> Result<Option<Entity>> {
    let mut select = base_select();
    select.and_where_eq("Z_ENT", id);
    let result = query_as::<_, Entity>(select.sql()?.as_str())
        .fetch_optional(pool)
        .await?;

    Ok(result)
}
