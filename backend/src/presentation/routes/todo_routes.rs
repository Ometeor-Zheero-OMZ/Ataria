use axum::{
    routing::{get, post, put, delete},
    Router
};

use crate::presentation::handlers::todo_handler::{
    complete_todo,
    create_todo,
    delete_todo,
    get_todos,
    update_todo
};

pub fn todo_scope() -> Router {
    Router::new()
        .route("/todos", get(get_todos))
        .route("/todo", post(create_todo))
        .route("/todo", put(update_todo))
        .route("/todo", delete(delete_todo))
        .route("/todo/change-status", put(complete_todo))
        .fallback(|| async { "404 Not Found" })
}
