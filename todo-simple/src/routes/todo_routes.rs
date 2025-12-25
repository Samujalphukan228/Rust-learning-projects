use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::{
    handlers::{
        create_todo::create_todo,
        get_todos::get_todos,
        get_todo_by_id::get_todo_by_id,
        delete_todo::delete_todo,
        update_todo::update_todo,
    },
    state::AppState,
};

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route(
            "/todos/:id",
            get(get_todo_by_id)
                .put(update_todo)
                .delete(delete_todo),
        )
}
