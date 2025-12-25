use axum::{extract::{Path, State}, Json};
use mongodb::bson::doc;

use crate::{models::todo::Todo, state::AppState};

// GET /todos/:id
pub async fn get_todo_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Json<Option<Todo>> {

    let collection = state.db.collection::<Todo>("todos");

    let todo = collection
        .find_one(doc! { "_id": id }, None)
        .await
        .expect("Failed to get todo");

    Json(todo)
}
