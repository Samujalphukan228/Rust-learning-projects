use axum::{Router, routing::post};
use crate::handlers::auth_handler;

pub fn auth_routes() -> Router<crate::app_state::AppState> {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
}
