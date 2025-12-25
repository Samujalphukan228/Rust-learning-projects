use axum::{extract::State, Json};

use crate::{
    models::todo::Todo,
    state::AppState,
};

#[derive(serde::Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

// POST /todos
pub async fn create_todo(
    State(state): State<AppState>,     // DB comes from state
    Json(payload): Json<CreateTodoRequest>,
) -> Json<Todo> {

    // Get collection
    let collection = state.db.collection::<Todo>("todos");

    // Create todo
    let todo = Todo::new(payload.title);

    // Insert into MongoDB
    collection
        .insert_one(&todo, None)
        .await
        .expect("Failed to insert todo");

    Json(todo)
}
