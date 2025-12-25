use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};

mod env;
mod db;
mod state;
mod models;
mod handlers;
mod routes;

use env::Env;
use db::connect_db;
use state::AppState;
use routes::todo_routes::todo_routes;

#[tokio::main]
async fn main() {
    // 1. Load env variables
    let env = Env::load();

    // 2. Connect DB ONCE
    let db = connect_db(&env).await;

    // 3. Create app state
    let state = AppState { db };

    // 4. Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 5. Build router
    let app = Router::new()
        .merge(todo_routes())
        .layer(cors)  // <-- Add CORS layer here
        .with_state(state);

    // 6. Start server
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}