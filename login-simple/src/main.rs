use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod env;
mod db;
mod state;
mod models;
mod handlers;
mod routes;

use env::Env;
use db::connect_db;
use state::AppState;
use routes::auth_routes::auth_routes;

#[tokio::main]
async fn main() {
    let env = Env::load();
    let db = connect_db(&env).await;

    let state = AppState {
        db,
        jwt_secret: env.jwt_secret,
    };

    let app = Router::new()
        .merge(auth_routes())
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Auth server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
