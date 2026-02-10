use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    NotFound,
    BadRequest,
    Database,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Database => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(_: mongodb::error::Error) -> Self {
        AppError::Database
    }
}
