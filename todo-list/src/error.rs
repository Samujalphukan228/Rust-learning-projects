use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound,
    Database,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{:?",self);

        match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::NotFound => StatusCode::NOT_FOUND.into_response(),
            AppError::Database => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(_: mongodb::error::Error) -> Self {
        AppError::Database
    }
}