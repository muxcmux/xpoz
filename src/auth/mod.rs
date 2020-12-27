use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_session::{Session, UserSession};
use actix_web::{
    dev::ServiceRequest, dev::ServiceResponse, get, http::header, Error, HttpRequest, HttpResponse,
    Result as AWResult,
};
use futures::future::{ok, Ready};
use futures::Future;
use nanoid::nanoid;

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
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
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
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
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
        let session = req.get_session();

        if let Ok(None) = session.get::<String>("id") {
            let id = nanoid!();
            session.set("id", id);
        }

        if req.path() == "/auth" || access_allowed(&session) {
            let fut = self.service.call(req);
            return Box::pin(async move { Ok(fut.await?) });
        }

        Box::pin(
            async move { Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body())) },
        )
    }
}

fn access_allowed(session: &Session) -> bool {
    let session_token = session.get::<String>("token");

    if let Ok(Some(v)) = session_token {
        return v == "asd";
    }

    false
}

#[get("/auth")]
pub async fn auth(session: Session, req: HttpRequest) -> AWResult<HttpResponse> {
    let token = req.query_string();

    if token != "" {
        session.set("token", token);
    }

    Ok(HttpResponse::Found()
        .set_header(header::LOCATION, "/")
        .finish())
}
