use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use mongodb::error;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("Invalid ID")]
    InvalidId(#[from] bson::oid::Error),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("Note found")]
    NotFound,

    #[error("Configuration error: {0}")]
    Config(String),
}