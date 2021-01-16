use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::db::tokens::{find_valid_token, Token};
use crate::db::Databases;
use actix_service::{Service, Transform};
use actix_session::{Session, UserSession};
use actix_web::{
    dev::ServiceRequest, dev::ServiceResponse, error::ErrorUnauthorized, get, http::header, web,
    Error, HttpRequest, HttpResponse, Result as AWResult,
};
use futures::future::{ok, Ready};
use futures::Future;
use nanoid::nanoid;
use sqlx::sqlite::SqlitePool;

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
        ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
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

        Box::pin(async move {
            let session = req.get_session();
            let dbs = req
                .app_data::<web::Data<Databases>>()
                .expect("Can't get db pool from auth middleware")
                .get_ref();

            if let Ok(None) = session.get::<String>("id") {
                let id = nanoid!();
                let _ = session.set("id", id);
            }

            let token = authenticate(Some(&req.path()), &session, &dbs.app).await;

            if let Some(a) = token {
                req.head().extensions_mut().insert(a);
                let fut = srv.call(req);
                Ok(fut.await?)
            } else {
                Err(ErrorUnauthorized("unauthorized"))
            }
        })
    }
}

const NO_AUTH_PATHS: [&str; 3] = [
    "/auth",
    "/robots.txt",
    "/share.html"
];

async fn authenticate(path: Option<&str>, session: &Session, pool: &SqlitePool) -> Option<Token> {
    match path {
        Some(p) => {
            if NO_AUTH_PATHS.contains(&p) {
                return Some(Token::anonymous());
            }
            authenticate_user(session, pool).await
        }
        None => authenticate_user(session, pool).await,
    }
}

async fn authenticate_user(session: &Session, pool: &SqlitePool) -> Option<Token> {
    let session_token = session.get::<String>("token");
    let session_id = session.get::<String>("id");

    if let Ok(Some(token)) = session_token {
        if let Ok(Some(id)) = session_id {
            return find_valid_token(pool, &id, &token, false).await;
        }
        return None;
    }
    None
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
