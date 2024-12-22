use actix_web::{
    error, http::StatusCode,
    HttpResponse,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("{0}")]
    InternalError(String),

    #[error("{0}")]
    NotFoundError(String)
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(json!({
                "status": "failed",
                "message": self.to_string()
            }))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::NotFoundError(_) => StatusCode::NOT_FOUND
        }
    }
}