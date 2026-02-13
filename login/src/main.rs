use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

mod config;
mod app_state;
mod routes;
mod handlers;
mod services;
mod models;
mod db;
mod utils;
mod errors;

use app_state::AppState;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let database = db::connect().await;
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET missing");

    let state = AppState {
        db: database,
        jwt_secret,
    };

    let app = Router::new()
        .merge(routes::auth::auth_routes())
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
