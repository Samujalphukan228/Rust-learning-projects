mod app;
mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod repository;
mod routes;
mod state;

use tokio::net::TcpListener;
use tracing_subscriber::fmt::Subscriber;

use config::AppConfig;
use db::init_db;
use state::AppState;
use app::create_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Subscriber::builder().init();

    let config = AppConfig::from_env()?;
    let db = init_db(&config).await?;
    let state = AppState::new(db);

    let app = create_app(state);

    let addr: String = format!("0.0.0.0:{}", config.port);
    let listener: TcpListener = TcpListener::bind(&addr).await?;

    tracing::info!("Server running on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
