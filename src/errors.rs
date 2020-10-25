use actix_web::{error::ResponseError, HttpResponse, http::StatusCode};
use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum APIError {
    #[error("The requested resource was not found on this server")]
    NotFound,
    #[error("An internal server error has occurred")]
    ServerError {
        #[from]
        source: anyhow::Error
    },
    #[error("You are not authorized to access this resource")]
    Unauthorized
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String
}

impl ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::ServerError {..} => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized => StatusCode::UNAUTHORIZED
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(ErrorResponse {
                error: self.to_string()
            })
    }
}

