use axum::Router;
use tower_http::trace::TraceLayer;

use crate::{routes::note_routes, state::AppState};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .nest("/api/notes", note_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
