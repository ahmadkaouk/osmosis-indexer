use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Config file not found")]
    ConfigFileNotFound,
    #[error("Invalid height {0}")]
    InvalidHeight(i64),
    #[error("Sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for IndexerError {
    fn into_response(self) -> Response {
        match self {
            IndexerError::Sqlx(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response(),
        }
    }
}
