use axum::{extract::State, Json};
use futures::stream::TryStreamExt;

use crate::{
    models::todo::Todo,
    state::AppState,
};

// GET /todos
pub async fn get_todos(
    State(state): State<AppState>, // DB from state
) -> Json<Vec<Todo>> {

    // Get MongoDB collection
    let collection = state.db.collection::<Todo>("todos");

    // Run find query (no filter = get all)
    let cursor = collection
        .find(None, None)
        .await
        .expect("Failed to fetch todos");

    // Convert cursor into Vec<Todo>
    let todos: Vec<Todo> = cursor
        .try_collect()
        .await
        .expect("Failed to read todos");

    Json(todos)
}
