use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("User already exists")]
    UserExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Database error")]
    DatabaseError,

    #[error("Internal error")]
    InternalError,

    #[error("Hashing error")]
    HashingError,

    #[error("Token generation error")]
    TokenGenerationError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::UserExists => StatusCode::CONFLICT,
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::HashingError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TokenGenerationError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(_: mongodb::error::Error) -> Self {
        AppError::DatabaseError
    }
}