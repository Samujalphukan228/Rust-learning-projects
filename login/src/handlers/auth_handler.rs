use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    services::auth_service,
    errors::AppError,
};

#[derive(Deserialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

pub async fn register(
    State
)