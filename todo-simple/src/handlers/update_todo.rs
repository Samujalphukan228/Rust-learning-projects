use axum::{extract::{Path, State}, Json};
use mongodb::bson::{doc, Document};

use crate::state::AppState;

#[derive(serde::Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub done: Option<bool>,
}

// PUT /todos/:id
pub async fn update_todo(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Json<bool> {

    let collection = state.db.collection::<Document>("todos");

    let mut update_doc = Document::new();

    if let Some(title) = payload.title {
        update_doc.insert("title", title);
    }

    if let Some(done) = payload.done {
        update_doc.insert("done", done);
    }

    let result = collection
        .update_one(
            doc! { "_id": id },
            doc! { "$set": update_doc },
            None,
        )
        .await
        .expect("Failed to update todo");

    Json(result.matched_count == 1)
}
