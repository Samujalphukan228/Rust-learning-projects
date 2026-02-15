use axum::{
    Router,
    routing::{get, post},
};
use mongodb::Database;

use crate::handlers::notes::*;

pub fn note_routes() -> Router<Database> {
    Router::new()
        .route("/notes", post(create_note).get(get_notes))
        .route(
            "/notes/:id",
            get(get_note).put(update_note).delete(delete_note),
        )
}