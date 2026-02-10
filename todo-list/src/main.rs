use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use std::sync::Arc;

mod env;
mod db;
mod error;
mod models;
mod routes;
mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Database,
    pub collection_name: Arc<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = env::AppConfig::load();
    let db = db::connect_db(&config).await;

    let state = AppState {
        db,
        collection_name: Arc::new(config.todo_collection), // ✅ Now being used!
    };

    let app = Router::new()
        .merge(routes::todo_routes())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("🚀 Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}