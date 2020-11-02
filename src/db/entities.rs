use async_graphql::{Object, Result, Context};
use sqlx::{query, sqlite::SqlitePool};

pub struct Entity {
    id: Option<i32>,
    name: Option<String>,
    parent_id: Option<i32>
}

#[Object]
impl Entity {
    async fn id(&self) -> &Option<i32> { &self.id }
    async fn name(&self) -> &Option<String> { &self.name }
    async fn parent_id(&self) -> &Option<i32> { &self.parent_id }
    // async fn parent(&self, ctx: &Context<'_>) -> &Option<Self> {
    //     &self.parent_id.map(|v| {
    //         if v == 0 { return None }
    //     })
    // }
}

pub async fn entities(ctx: &Context<'_>) -> Result<Vec<Entity>> {
    let mut entities = vec![];
    let records = query!("SELECT * FROM Z_PRIMARYKEY")
        .fetch_all(ctx.data::<SqlitePool>()?)
        .await?;

    for r in records {
        entities.push(Entity {
            id: r.Z_ENT,
            name: r.Z_NAME,
            parent_id: r.Z_SUPER
        });
    }

    Ok(entities)
}

pub async fn entity(ctx: &Context<'_>, id: i32) -> Result<Option<Entity>> {
    let result = query!("SELECT * FROM Z_PRIMARYKEY WHERE Z_ENT = ?", id)
        .fetch_optional(ctx.data::<SqlitePool>()?)
        .await?;

    let any = match result {
        Some(r) => Some(Entity {
            id: r.Z_ENT,
            name: r.Z_NAME,
            parent_id: r.Z_SUPER
        }),
        None => None
    };

    Ok(any)
}
