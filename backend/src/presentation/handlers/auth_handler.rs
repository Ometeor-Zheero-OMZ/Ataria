use std::sync::Arc;

use axum::{
    extract::Json,
    http::{HeaderMap, StatusCode}, response::IntoResponse,
};
use serde_json::{json, Value};
use postgres::error::SqlState;
use validator::Validate;
use lambda_http::tracing::{error, info};

use crate::{
    application::{
        errors::auth_error::AuthError,
        jwt::jwt,types::custom_types::{AuthRepositoryArc, AuthServiceArc}
    },
    domain::{
        entities::auth::{LoginRequest, SignupRequest},
        repositories::auth_repository::AuthRepositoryImpl,
        services::auth_service::AuthServiceImpl
    }
};

use crate::infrastructure::db::connection::get_db_pool;

/// ゲストログイン
pub async fn guest_login(
    Json(req): Json<LoginRequest>,
) -> Json<Value> {
    let pool = get_db_pool().await;
    let auth_repository: AuthRepositoryArc = Arc::new(AuthRepositoryImpl::new(pool.clone()));
    let auth_service: AuthServiceArc = Arc::new(AuthServiceImpl::new(auth_repository.clone(), pool.clone()));

    if let Err(validation_errors) = req.validate() {
        return Json(json!({
            "status": StatusCode::BAD_REQUEST.to_string(),
            "data": validation_errors
        }));
    }

    match auth_service.guest_login(&req).await {
        Ok(Some(user_data)) => Json(json!({ "status": "success", "data": user_data })),
        Ok(None) => {
            Json(json!({
                "status": StatusCode::UNAUTHORIZED.to_string(),
                "message": "登録されていないユーザーです。"
            }))
        }
        Err(auth_error) => {
            println!("AuthError");
            Json(json!({
                "status": StatusCode::UNAUTHORIZED.to_string(),
                "message": auth_error.to_string()
            }))
        }
    }
}

/// 新規登録
pub async fn signup(
    Json(req): Json<SignupRequest>,
    // State(app_state): State<AppState>
) -> Json<Value> {
    println!("signup 関数を呼び出しました。");

    if let Err(validation_errors) = req.validate() {
        return Json(json!({
            "status": StatusCode::BAD_REQUEST.to_string(),
            "data": validation_errors
        }));
    }

    let pool = get_db_pool().await;
    let auth_repository: AuthRepositoryArc = Arc::new(AuthRepositoryImpl::new(pool.clone()));
    let auth_service: AuthServiceArc = Arc::new(AuthServiceImpl::new(auth_repository.clone(), pool.clone()));

    // let auth_service = &app_state.auth_service;

    match auth_service.signup(&req).await {
        Ok(()) => {
            info!("[auth_controller] - [signup] - [message: Successfully signed up]");
            return Json(json!({ "status": "success", "message": "Successfully signed up" }));
        }
        Err(AuthError::DatabaseError(ref error)) => {
            if let Some(db_error) = error.as_db_error() {
                if db_error.code() == &SqlState::UNIQUE_VIOLATION {
                    error!("[auth_controller] - [signup] - [message: db_error = {}]", db_error);
                    return Json(json!({
                        "status": StatusCode::CONFLICT.to_string(),
                        "message": "すでに登録されたユーザーです。"
                    }));
                }
            }

            error!("[auth_controller] - [signup] - [message: error = {}]", error);
            Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "message": "Internal server error"
            }))
        }
        Err(auth_error) => {
            error!("[auth_controller] - [signup] - [message: auth_error = {}]", auth_error);
            Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "message": "Internal server error"
            }))
        }
    }
}

/// ログイン
pub async fn login(
    Json(req): Json<LoginRequest>,
    // State(app_state): State<AppState>
) -> Json<Value> {
    println!("login 関数を呼び出しました。");

    if let Err(validation_errors) = req.validate() {
        return Json(json!({
            "status": StatusCode::BAD_REQUEST.to_string(),
            "data": validation_errors
        }));
    }

    let pool = get_db_pool().await;
    let auth_repository: AuthRepositoryArc = Arc::new(AuthRepositoryImpl::new(pool.clone()));
    let auth_service: AuthServiceArc = Arc::new(AuthServiceImpl::new(auth_repository.clone(), pool.clone()));

    // let auth_service = &app_state.auth_service;

    match auth_service.login(&req).await {
        Ok(Some(user_data)) => Json(json!({ "status": "success", "data": user_data })),
        Ok(None) => {
            error!("[auth_controller] - [login] - [message: USER NOT FOUND]");
            Json(json!({
                "status": StatusCode::UNAUTHORIZED.to_string(),
                "message": "登録されていないユーザーです。"
            }))
        }
        Err(auth_error) => {
            error!("[auth_controller] - [login] - [message: auth_error = {}]", auth_error);
            Json(json!({
                "status": StatusCode::UNAUTHORIZED.to_string(),
                "message": "登録されていないユーザーです。"
            }))
        }
    }
}

pub async fn current_user(headers: HeaderMap) -> impl IntoResponse {
    println!("current_user 関数を呼び出しました。");

    match jwt::verify(&headers) {
        Ok(user_info) => Json(json!({ "status": "success", "data": user_info })),
        Err(error) => {
            error!("[auth_controller] - [current_user] - [message: error = {}]", error);
            Json(json!({
                "status": StatusCode::UNAUTHORIZED.to_string(),
                "message": "登録されていないユーザーです。"
            }))
        }
    }
}