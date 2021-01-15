use super::albums::{my_albums, Album, AllowedAlbumIds};
use super::Databases;
use super::entities::Entity;
use crate::db::bool_to_insert_string;
use anyhow::Result;
use async_graphql::{Context, InputObject, Object, Result as AGResult};
use nanoid::nanoid;
use sql_builder::prelude::*;
use sqlx::{query, query_as, sqlite::SqlitePool, Done};

#[derive(InputObject)]
pub struct TokenInput {
    name: String,
    session_bound: bool,
    admin: bool,
    album_ids: Option<Vec<String>>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct Token {
    name: String,
    session_bound: bool,
    pub admin: bool,
    session_id: Option<String>,
    token: String,
    pub whitelist: Option<String>,
    created_at: String,
}

impl Token {
    pub fn anonymous() -> Self {
        Self {
            name: "Anonymous".to_string(),
            session_bound: true,
            admin: false,
            session_id: None,
            token: nanoid!(),
            whitelist: None,
            created_at: "".to_string(),
        }
    }

    pub fn whitelist(&self) -> AllowedAlbumIds {
        match &self.whitelist {
            None => None,
            Some(v) => {
                let json: Result<AllowedAlbumIds> =
                    serde_json::from_str(&v).map_err(anyhow::Error::from);
                json.map_or_else(|_| None, |v| v)
            }
        }
    }
}

#[Object]
impl Token {
    async fn id(&self) -> &str {
        &self.token
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn session_bound(&self) -> &bool {
        &self.session_bound
    }
    async fn admin(&self) -> &bool {
        &self.admin
    }
    async fn session_id(&self) -> &Option<String> {
        &self.session_id
    }
    async fn created_at(&self) -> &str {
        &self.created_at
    }
    async fn whitelisted_albums(&self, ctx: &Context<'_>) -> AGResult<Option<Vec<Album>>> {
        if let None = &self.whitelist {
            return Ok(None);
        }
        Ok(Some(my_albums(
            &ctx.data::<Databases>()?.photos,
            ctx.data::<Vec<Entity>>()?,
            &self.whitelist(),
            None,
        ).await?))
    }
}

pub async fn create_token(pool: &SqlitePool, input: TokenInput) -> Result<Option<Token>> {
    let token = nanoid!();

    let mut builder = SqlBuilder::insert_into("tokens");
    builder.field("session_bound").field("admin").field("token");

    let mut values = vec![
        bool_to_insert_string(input.session_bound),
        bool_to_insert_string(input.admin),
        quote(&token),
    ];

    builder.field("name");
    values.push(quote(input.name));

    if let Some(w) = input.album_ids {
        let album_ids = serde_json::to_string(&w)?;
        builder.field("whitelist");
        values.push(quote(&album_ids));
    }

    builder.values(&values);

    query(builder.sql()?.as_str()).execute(pool).await?;

    let mut finder = SqlBuilder::select_from("tokens");
    finder.and_where("token = ?".bind(&token));

    let result = query_as::<_, Token>(finder.sql()?.as_str())
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub async fn update_token(pool: &SqlitePool, token: &str, input: TokenInput) -> Result<Option<Token>> {
    let mut builder = SqlBuilder::update_table("tokens");

    builder.set("name", quote(&input.name))
        .set("session_bound", bool_to_insert_string(input.session_bound))
        .set("admin", bool_to_insert_string(input.admin));

    if let Some(w) = input.album_ids {
        let album_ids = serde_json::to_string(&w)?;
        builder.set("whitelist", quote(&album_ids));
    } else {
        builder.set("whitelist", "NULL");
    }

    builder.and_where("token = ?".bind(&token));

    query(builder.sql()?.as_str()).execute(pool).await?;

    let mut finder = SqlBuilder::select_from("tokens");
    finder.and_where("token = ?".bind(&token));

    let result = query_as::<_, Token>(finder.sql()?.as_str())
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub async fn delete_token(pool: &SqlitePool, token: String) -> Result<Option<Token>> {
    let existing = get_token(pool, &token).await?;

    if let None = existing {
        return Ok(None);
    }

    let mut builder = SqlBuilder::delete_from("tokens");
    builder.and_where("token = ?".bind(&token));

    let result = query(builder.sql()?.as_str()).execute(pool).await?;

    if result.rows_affected() < 1 {
        Ok(None)
    } else {
        Ok(existing)
    }
}

async fn get_token(pool: &SqlitePool, token: &str) -> Result<Option<Token>> {
    let mut builder = SqlBuilder::select_from("tokens");
    builder.and_where("token = ?".bind(&token));

    let result = query_as::<_, Token>(builder.sql()?.as_str())
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub async fn tokens(pool: &SqlitePool) -> Result<Vec<Token>> {
    let mut builder = SqlBuilder::select_from("tokens");
    builder.order_by("created_at", true);

    let records = query_as::<_, Token>(builder.sql()?.as_str())
        .fetch_all(pool)
        .await?;

    Ok(records)
}

pub async fn find_valid_token(
    pool: &SqlitePool,
    session_id: &str,
    token: &str,
    admin: bool,
) -> Option<Token> {
    let mut builder = SqlBuilder::select_from("tokens");
    builder.and_where("token = ?".bind(&token));

    if admin {
        builder.and_where_eq("admin", 1);
    }

    builder.and_where(
        "session_bound = 0 OR (session_bound = 1 AND (session_id IS NULL OR session_id = ?))"
            .bind(&session_id),
    );

    builder.limit(1);

    let result = query_as::<_, Token>(builder.sql().expect("Failed to build token query").as_str())
        .fetch_optional(pool)
        .await;

    match result {
        Ok(result) => {
            if let Some(record) = result {
                if record.session_bound {
                    consume_token(pool, &record.token, session_id).await;
                }
                return Some(record);
            }
            None
        }
        _ => None,
    }
}

async fn consume_token(pool: &SqlitePool, token: &str, session_id: &str) {
    let update = SqlBuilder::update_table("tokens")
        .set("session_id", &quote(session_id))
        .and_where("token = ?".bind(&token))
        .sql()
        .expect("Failed SQL query when consuming auth token");

    let _ = query(&update).execute(pool).await;
}
