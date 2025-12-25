use axum::{routing::post, Router};
use crate::{
    handlers::{register::register, login::login},
    state::AppState,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}
