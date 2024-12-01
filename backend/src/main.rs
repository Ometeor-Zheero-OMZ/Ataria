use axum::{Router, Extension, http::Method};
use tower_http::cors::{CorsLayer, Any};
use lambda_http::{run, tracing, Error};
use std::env::set_var;
use dotenvy::dotenv;

use application::middlewares::jwt_middleware::JwtMiddlewareLayer;
use presentation::routes::{auth_routes::auth_scope, todo_routes::todo_scope};

mod application;
mod domain;
mod infrastructure;
mod presentation;

// 自作ロガーを使用する場合に使用
// const PROJECT_PATH: &'static str = env!("CARGO_MANIFEST_DIR");

#[tokio::main]
async fn main() -> Result<(), Error> {
    // API Gatewayで /dev/ や /prod/ などのステージ名をパスに含めておきたい場合、true にする
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "false");

    // CloudWatch へのログ出力を許可
    tracing::init_default_subscriber();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    dotenv().ok();

    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let app = Router::new()
        .nest("/api", todo_scope())
        .nest("/api/auth", auth_scope())
        .route_layer(Extension(JwtMiddlewareLayer::new()))
        .layer(cors_layer);

    run(app).await
}