use std::sync::Arc;

use axum::{
    extract::Json,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

use crate::{
    application::{
        jwt::jwt,
        types::custom_types::{TodoRepositoryArc, TodoServiceArc}},
    domain::{entities::todo::{
            RequestCompleteTodoItem,
            RequestCreateTodoItem,
            RequestDeleteTodoItem,
            RequestUpdateTodoItem
        },
        repositories::todo_repository::TodoRepositoryImpl,
        services::todo_service::TodoServiceImpl
    },
    infrastructure::db::connection::get_db_pool
};

pub async fn get_todos(
    headers: HeaderMap,
) -> impl IntoResponse {
    let pool = get_db_pool().await;
    let todo_repository: TodoRepositoryArc = Arc::new(TodoRepositoryImpl::new(pool.clone()));
    let todo_service: TodoServiceArc = Arc::new(TodoServiceImpl::new(todo_repository.clone(), pool.clone()));

    let user = jwt::verify(&headers);

    match user {
        Ok(user_data) => match todo_service.get_todos(user_data).await {
            Ok(todos) => {
                Json(json!({ "status": "success", "data": todos }))
            },
            Err(todo_error) => {
                println!("[todo_controller] - [get_todos] - [message: todo_error = {}]", todo_error);
                Json(json!({
                    "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    "message": todo_error.to_string()
                }))
            }
        },
        Err(error) => {
            println!("[todo_controller] - [get_todos] - [message: error = {}]", error);
            Json(json!({
                "status": StatusCode::UNAUTHORIZED.to_string(),
                "message": error.to_string()
            }))
        }
    }
}

pub async fn create_todo(
    headers: HeaderMap,
    Json(todo_req): Json<RequestCreateTodoItem>,
) -> impl IntoResponse {
    let pool = get_db_pool().await;
    let todo_repository: TodoRepositoryArc = Arc::new(TodoRepositoryImpl::new(pool.clone()));
    let todo_service: TodoServiceArc = Arc::new(TodoServiceImpl::new(todo_repository.clone(), pool.clone()));

    let user = jwt::verify(&headers);

    match user {
        Ok(user_data) => match todo_service.create_todo(user_data, &todo_req).await {
            Ok(response) => Json(json!({ "status": "success", "data": response })),
            Err(todo_error) => {
                println!("[todo_controller] - [create_todo] - [message: todo_error = {}]", todo_error);
                Json(json!({
                    "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    "message": todo_error.to_string()
                }))
            }
        },
        Err(error) => {
            println!("[todo_controller] - [create_todo] - [message: error = {}]", error);
            Json(json!({
                "status": StatusCode::UNAUTHORIZED.to_string(),
                "message": "登録されていないユーザーです。"
            }))
        }
    }
}

pub async fn update_todo(
    headers: HeaderMap,
    Json(todo_req): Json<RequestUpdateTodoItem>,
) -> impl IntoResponse {
    let pool = get_db_pool().await;
    let todo_repository: TodoRepositoryArc = Arc::new(TodoRepositoryImpl::new(pool.clone()));
    let todo_service: TodoServiceArc = Arc::new(TodoServiceImpl::new(todo_repository.clone(), pool.clone()));

    let user = jwt::verify(&headers);

    match user {
        Ok(user_data) => match todo_service.update_todo(user_data, &todo_req).await {
            Ok(()) => Json(json!({ "status": "success", "message": "タスクを更新しました。" })),
            Err(todo_error) => {
                println!("[todo_controller] - [update_todo] - [message: todo_error = {}]", todo_error);
                Json(json!({
                    "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    "message": todo_error.to_string()
                }))
            }
        },
        Err(error) => {
            println!("[todo_controller] - [update_todo] - [message: error = {}]", error);
            Json(json!({
                "status": StatusCode::UNAUTHORIZED.to_string(),
                "message": "登録されていないユーザーです。"
            }))
        }
    }
}

pub async fn delete_todo(
    headers: HeaderMap,
    Json(todo_req): Json<RequestDeleteTodoItem>,
) -> impl IntoResponse {
    let pool = get_db_pool().await;
    let todo_repository: TodoRepositoryArc = Arc::new(TodoRepositoryImpl::new(pool.clone()));
    let todo_service: TodoServiceArc = Arc::new(TodoServiceImpl::new(todo_repository.clone(), pool.clone()));

    let user = jwt::verify(&headers);

    match user {
        Ok(user_data) => match todo_service.delete_todo(user_data, &todo_req).await {
            Ok(()) => Json(json!({ "status": "success", "message": "タスクを削除しました。" })),
            Err(todo_error) => {
                println!("[todo_controller] - [delete_todo] - [message: todo_error = {}]", todo_error);
                Json(json!({
                    "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    "message": todo_error.to_string()
                }))
            }
        },
        Err(error) => {
            println!("[todo_controller] - [delete_todo] - [message: error = {}]", error);
            Json(json!({
                "status": StatusCode::UNAUTHORIZED.to_string(),
                "message": "登録されていないユーザーです。"
            }))
        }
    }
}

pub async fn complete_todo(
    headers: HeaderMap,
    Json(todo_req): Json<RequestCompleteTodoItem>,
) -> impl IntoResponse {
    let pool = get_db_pool().await;
    let todo_repository: TodoRepositoryArc = Arc::new(TodoRepositoryImpl::new(pool.clone()));
    let todo_service: TodoServiceArc = Arc::new(TodoServiceImpl::new(todo_repository.clone(), pool.clone()));

    let user = jwt::verify(&headers);

    match user {
        Ok(user_data) => match todo_service.complete_todo(user_data, &todo_req).await {
            Ok(()) => Json(json!({ "status": "success", "message": "タスクを完了しました。" })),
            Err(todo_error) => {
                println!("[todo_controller] - [complete_todo] - [message: todo_error = {}]", todo_error);
                Json(json!({
                    "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    "message": todo_error.to_string()
                }))
            }
        },
        Err(error) => {
            println!("[todo_controller] - [complete_todo] - [message: error = {}]", error);
            Json(json!({
                "status": StatusCode::UNAUTHORIZED.to_string(),
                "message": "登録されていないユーザーです。"
            }))
        }
    }
}