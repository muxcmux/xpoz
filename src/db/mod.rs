mod albums;
pub mod assets;
pub mod entities;
pub mod migrate;
pub mod tokens;

use albums::{album, my_albums, Album};
use async_graphql::{
    Context, EmptySubscription, Error, ErrorExtensions, Object, Result, Schema as AGSchema,
};
use entities::{entities, entity, Entity};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tokens::{create_token, delete_token, tokens, Token};

pub async fn build_pool(url: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .idle_timeout(std::time::Duration::new(5, 0))
        .max_connections(3)
        .connect(url)
        .await
        .expect(&format!("Can't open database {}", url))
}

pub fn bool_to_insert_string(value: bool) -> String {
    if value { "1" } else { "0" }.to_string()
}

#[derive(Clone)]
pub struct Databases {
    pub photos: SqlitePool,
    pub app: SqlitePool,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Gets a single entity by it's id
    async fn entity(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Entity>> {
        entity(&ctx.data::<Databases>()?.photos, id)
            .await
            .map_err(Error::from)
    }

    /// Returns all available entities in the photos app
    async fn entities(&self, ctx: &Context<'_>) -> Result<Vec<Entity>> {
        entities(&ctx.data::<Databases>()?.photos)
            .await
            .map_err(Error::from)
    }

    /// Get an album by its uuid
    async fn album(&self, ctx: &Context<'_>, uuid: String) -> Result<Option<Album>> {
        album(
            &ctx.data::<Databases>()?.photos,
            ctx.data::<Vec<Entity>>()?,
            &ctx.data::<Token>()?.whitelist(),
            &uuid,
        )
        .await
        .map_err(Error::from)
    }

    /// "My Albums" which have been xpozed, keeping the original Photos sorting
    async fn my_albums(&self, ctx: &Context<'_>, page: i32) -> Result<Vec<Album>> {
        my_albums(
            &ctx.data::<Databases>()?.photos,
            ctx.data::<Vec<Entity>>()?,
            &ctx.data::<Token>()?.whitelist(),
            page,
        )
        .await
        .map_err(Error::from)
    }

    // Admin resources

    /// Returns the available access tokens
    async fn tokens(&self, ctx: &Context<'_>) -> Result<Vec<Token>> {
        let token = ctx.data::<Token>()?;
        if token.admin {
            tokens(&ctx.data::<Databases>()?.app)
                .await
                .map_err(Error::from)
        } else {
            Err(Error::new("Unauthorised").extend_with(|_, e| e.set("code", 401)))
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_token(
        &self,
        ctx: &Context<'_>,
        name: Option<String>,
        session_bound: bool,
        admin: bool,
        whitelist: Option<String>,
    ) -> Result<Option<Token>> {
        let token = ctx.data::<Token>()?;
        if token.admin {
            create_token(
                &ctx.data::<Databases>()?.app,
                name,
                session_bound,
                admin,
                whitelist,
            )
            .await
            .map_err(Error::from)
        } else {
            Err(Error::new("Unauthorised").extend_with(|_, e| e.set("code", 401)))
        }
    }

    async fn delete_token(&self, ctx: &Context<'_>, id: String) -> Result<u64> {
        let token = ctx.data::<Token>()?;
        if token.admin {
            delete_token(&ctx.data::<Databases>()?.app, id)
                .await
                .map_err(Error::from)
        } else {
            Err(Error::new("Unauthorised").extend_with(|_, e| e.set("code", 401)))
        }
    }
}

pub type Schema = AGSchema<QueryRoot, MutationRoot, EmptySubscription>;
