use std::pin::Pin;
use std::cell::RefCell;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::db::Databases;
use actix_service::{Service, Transform};
use actix_session::{Session, UserSession};
use actix_web::{
    error::ErrorUnauthorized,
    dev::ServiceRequest, dev::ServiceResponse, get, http::header, Error, HttpRequest, HttpResponse,
    web,
    Result as AWResult,
};
use futures::future::{ok, Ready};
use futures::Future;
use nanoid::nanoid;
use sqlx::{query_as, query, sqlite::SqlitePool};
use sql_builder::prelude::*;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service: Rc::new(RefCell::new(service)) })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut srv = self.service.clone();

        Box::pin(
            async move {
                let session = req.get_session();
                let dbs = req
                    .app_data::<web::Data<Databases>>()
                    .expect("Can't get db pool from auth middleware")
                    .get_ref();

                if let Ok(None) = session.get::<String>("id") {
                    let id = nanoid!();
                    let _ = session.set("id", id);
                }

                let access = access_allowed(Some(&req.path()), &session, &dbs.app).await;


                if let Some(a) = access {
                    req.head().extensions_mut().insert(a);
                    let fut = srv.call(req);
                    Ok(fut.await?)
                } else {
                    Err(ErrorUnauthorized("unauthorized"))
                }
            },
        )
    }
}

const WHITELIST_SEPARATOR: &str = ",";

#[derive(sqlx::FromRow, Clone)]
pub struct Access {
    name: String,
    single_use: bool,
    admin: bool,
    session_id: Option<String>,
    token: String,
    pub whitelist: Option<String>,
}

impl Access {
    fn anonymous() -> Self {
        Self {
            name: "Anonymous".to_string(),
            single_use: true,
            admin: false,
            session_id: None,
            token: nanoid!(),
            whitelist: None,
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

async fn access_allowed(path: Option<&str>, session: &Session, pool: &SqlitePool) -> Option<Access> {
    match path {
        Some(p) => {
            if p == "/auth" {
                return Some(Access::anonymous());
            }
            if p == "/admin" {
                return admin_access_allowed(session, pool).await;
            }
            user_access_allowed(session, pool).await
        },
        None => user_access_allowed(session, pool).await
    }
}

async fn admin_access_allowed(session: &Session, pool: &SqlitePool) -> Option<Access> {
    let session_token = session.get::<String>("token");
    let session_id = session.get::<String>("id");

    if let Ok(Some(token)) = session_token {
        if let Ok(Some(id)) = session_id {
            return access(pool, &id, &token, true).await;
        }
        return None;
    }
    None
}

async fn user_access_allowed(session: &Session, pool: &SqlitePool) -> Option<Access> {
    let session_token = session.get::<String>("token");
    let session_id = session.get::<String>("id");

    if let Ok(Some(token)) = session_token {
        if let Ok(Some(id)) = session_id {
            return access(pool, &id, &token, false).await;
        }
        return None;
    }
    None
}

async fn access(pool: &SqlitePool, session_id: &str, token: &str, admin: bool) -> Option<Access> {
    let mut builder = SqlBuilder::select_from("access");
    builder.and_where("token = ?".bind(&token));

    if admin {
        builder.and_where_eq("admin", 1);
    }

    builder.and_where("single_use = 0 OR (single_use = 1 AND (session_id IS NULL OR session_id = ?))".bind(&session_id));

    builder.limit(1);

    let result = query_as::<_, Access>(builder.sql().expect("Failed to build access query").as_str())
        .fetch_optional(pool)
        .await;

    match result {
        Ok(result) => {
            if let Some(record) = result {
                if record.single_use {
                    consume_token(pool, &record.token, session_id).await;
                }
                return Some(record)
            }
            None
        },
        _ => None
    }
}

async fn consume_token(pool: &SqlitePool, token: &str, session_id: &str) {
    let update = SqlBuilder::update_table("access")
        .set("session_id", &quote(session_id))
        .and_where("token = ?".bind(&token))
        .sql()
        .expect("Failed SQL query when consuming auth token");

    let _ = query(&update)
        .execute(pool)
        .await;
}

#[get("/auth")]
pub async fn auth(session: Session, req: HttpRequest) -> AWResult<HttpResponse> {
    let token = req.query_string();

    if token != "" {
        let _ = session.set("token", token);
    }

    Ok(HttpResponse::Found()
        .set_header(header::LOCATION, "/")
        .finish())
}
