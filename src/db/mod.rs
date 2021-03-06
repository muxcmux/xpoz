mod albums;
pub mod assets;
pub mod entities;
pub mod migrate;
pub mod tokens;

use albums::{album, my_albums, Album};
use async_graphql::{
    Context, EmptySubscription, Error, ErrorExtensions, Object, Result, Schema as AGSchema,
};
use entities::Entity;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use tokens::{create_token, delete_token, tokens, update_token, Token, TokenInput};

pub async fn build_pool(options: SqliteConnectOptions) -> SqlitePool {
    log::debug!("Conn settings: {:?}", &options);
    SqlitePoolOptions::new()
        .idle_timeout(std::time::Duration::new(5, 0))
        .max_connections(3)
        .connect_with(options)
        .await
        .expect("Can't open database")
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
    /// Get an album by its id
    async fn album(&self, ctx: &Context<'_>, id: String) -> Result<Option<Album>> {
        album(
            &ctx.data::<Databases>()?.photos,
            ctx.data::<Vec<Entity>>()?,
            &ctx.data::<Token>()?.whitelist(),
            &id,
        )
        .await
        .map_err(Error::from)
    }

    /// "My Albums" which have been xpozed, keeping the original Photos sorting
    async fn my_albums(&self, ctx: &Context<'_>, page: Option<i32>) -> Result<Vec<Album>> {
        my_albums(
            &ctx.data::<Databases>()?.photos,
            ctx.data::<Vec<Entity>>()?,
            &ctx.data::<Token>()?.whitelist(),
            page,
        )
        .await
        .map_err(Error::from)
    }

    /// Returns the current access token
    async fn me(&self, ctx: &Context<'_>) -> Result<Token> {
        let token_ref = ctx.data::<Token>()?;
        Ok(token_ref.clone())
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
    async fn create_token(&self, ctx: &Context<'_>, input: TokenInput) -> Result<Option<Token>> {
        let token = ctx.data::<Token>()?;
        if token.admin {
            create_token(&ctx.data::<Databases>()?.app, input)
                .await
                .map_err(Error::from)
        } else {
            Err(Error::new("Unauthorised").extend_with(|_, e| e.set("code", 401)))
        }
    }

    async fn update_token(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: TokenInput,
    ) -> Result<Option<Token>> {
        let token = ctx.data::<Token>()?;
        if token.admin {
            update_token(&ctx.data::<Databases>()?.app, &id, input)
                .await
                .map_err(Error::from)
        } else {
            Err(Error::new("Unauthorised").extend_with(|_, e| e.set("code", 401)))
        }
    }

    async fn delete_token(&self, ctx: &Context<'_>, id: String) -> Result<Option<Token>> {
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
