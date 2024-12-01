use axum::{
    routing::{get, post},
    Router
};

use crate::presentation::handlers::auth_handler::{
    current_user,
    guest_login,
    login, signup
};

pub fn auth_scope() -> Router {
    Router::new()
        .route("/guest_login", post(guest_login))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/current_user", get(current_user))
        .fallback(|| async { "404 Not Found" })
}