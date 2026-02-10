use axum::Router;
use tokio::net::TcpListener;
use termcolor::Color;

mod env;
mod db;
mod error;
mod models;
mod routes;
mod handlers;
mod utils;

use crate::utils::console::print_color;

#[tokio::main]
async fn main() {
    // Load config
    let config = env::AppConfig::load();

    // Connect to database
    print_color(" Connecting to MongoDB...", Color::Cyan, true);
    let db = db::connect_db(&config).await;
    print_color(" Database connected successfully!", Color::Green, true);

    // Build app
    let app = Router::new()
        .merge(routes::note_routes())
        .with_state(db);

    // Start server
    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    print_color(
        &format!(" Server running on http://{addr}"),
        Color::Magenta,
        true,
    );

    axum::serve(listener, app)
        .await
        .expect("Server crashed");
}
