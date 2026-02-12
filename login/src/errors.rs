use axum::{
    response::{IntoResponse, Response},
    http:: StatusCode,
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
}