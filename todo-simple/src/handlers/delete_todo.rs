use axum::{extract::{Path, State}, Json};
use mongodb::bson::doc;

use crate::state::AppState;

// DELETE /todos/:id
pub async fn delete_todo(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Json<bool> {

    let collection = state.db.collection::<mongodb::bson::Document>("todos");

    let result = collection
        .delete_one(doc! { "_id": id }, None)
        .await
        .expect("Failed to delete todo");

    Json(result.deleted_count == 1)
}
