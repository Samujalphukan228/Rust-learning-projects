use axum::{
    Router,
    routing::{post, patch},
};

use crate::handlers::todos::*;
use crate::AppState;

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todos", post(create_todo).get(list_todos))
        .route("/todos/:id/toggle", patch(toggle_todo))
}