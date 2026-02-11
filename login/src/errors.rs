use axum::{
    response::{IntoResponse, Response},
    http:: StatusCode,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("User already exists")
    User]
}