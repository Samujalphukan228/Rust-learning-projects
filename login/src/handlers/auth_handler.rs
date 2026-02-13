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
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<String>, AppError> {
    let token = auth_service::register(
        &state.db,
        payload.email,
        payload.password,
        &state.jwt_secret,
    ).await?;

    Ok(Json(token))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<String>, AppError> {

    let token = auth_service::login(
        &state.db,
        payload.email,
        payload.password,
        &state.jwt_secret,
    ).await?;

    Ok(Json(token))
}