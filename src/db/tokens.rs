use crate::db::bool_to_insert_string;
use anyhow::Result;
use async_graphql::{Context, Object, Result as AGResult};
use nanoid::nanoid;
use sql_builder::prelude::*;
use sqlx::{query, query_as, sqlite::SqlitePool};

const WHITELIST_SEPARATOR: &str = ",";

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

    pub fn whitelist(&self) -> Option<Vec<&str>> {
        match &self.whitelist {
            None => None,
            Some(v) => {
                let split = v.split(WHITELIST_SEPARATOR);
                Some(split.collect::<Vec<&str>>())
            }
        }
    }
}

#[Object]
impl Token {
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
    async fn token(&self) -> &str {
        &self.token
    }
    async fn created_at(&self) -> &str {
        &self.created_at
    }
}

pub async fn create_token(
    pool: &SqlitePool,
    name: Option<String>,
    session_bound: bool,
    admin: bool,
    whitelist: Option<String>,
) -> Result<Option<Token>> {
    let token = nanoid!();

    let mut builder = SqlBuilder::insert_into("tokens");
    builder.field("session_bound").field("admin").field("token");

    let mut values = vec![
        bool_to_insert_string(session_bound),
        bool_to_insert_string(admin),
        quote(&token),
    ];

    if let Some(w) = whitelist {
        builder.field("whitelist");
        values.push(quote(&w));
    }

    if let Some(n) = name {
        builder.field("name");
        values.push(quote(n));
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
