use axum::{
    extract::{Path, Query, State},
    Json,
};
use mongodb::bson::{doc, oid::ObjectId};
use serde::Deserialize;
use validator::Validate;
use futures_util::TryStreamExt;

use crate::{models::Todo, error::AppError, AppState};

/* ---------- INPUT ---------- */

#[derive(Deserialize, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "title cannot be empty"))]
    pub title: String,
}

/* ---------- CREATE ---------- */
pub async fn create_todo(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    input.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let todo = Todo {
        id: None,
        title: input.title,
        done: false,
    };

    let col = state.db.collection::<Todo>(&state.collection_name); // ✅ Using config!
    col.insert_one(&todo, None).await?;

    tracing::info!("Todo created");

    Ok(Json(todo))
}

/* ---------- TOGGLE ---------- */
pub async fn toggle_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    let id = ObjectId::parse_str(&id)
        .map_err(|_| AppError::BadRequest("invalid id".into()))?;

    let col = state.db.collection::<Todo>(&state.collection_name); // ✅ Using config!

    let todo = col
        .find_one(doc! { "_id": &id }, None)
        .await?
        .ok_or(AppError::NotFound)?;

    col.update_one(
        doc! { "_id": id },
        doc! { "$set": { "done": !todo.done } },
        None,
    )
    .await?;

    tracing::info!("Todo toggled");

    Ok(())
}

/* ---------- FILTER ---------- */

#[derive(Deserialize)]
pub struct Filter {
    pub status: Option<String>, // all | done | ongoing
}

pub async fn list_todos(
    State(state): State<AppState>,
    Query(filter): Query<Filter>,
) -> Result<Json<Vec<Todo>>, AppError> {
    let query = match filter.status.as_deref() {
        Some("done") => doc! { "done": true },
        Some("ongoing") => doc! { "done": false },
        _ => doc! {},
    };

    let col = state.db.collection::<Todo>(&state.collection_name); // ✅ Using config!
    let mut cursor = col.find(query, None).await?;

    let mut todos = Vec::new();
    while let Some(todo) = cursor.try_next().await? {
        todos.push(todo);
    }

    Ok(Json(todos))
}