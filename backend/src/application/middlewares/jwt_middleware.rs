use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use std::task::{Context, Poll};
use tower::{Layer, Service};
use futures::future::BoxFuture;

use crate::application::jwt::jwt;

#[derive(Clone)]
pub struct JwtMiddlewareLayer;

impl JwtMiddlewareLayer {
    pub fn new() -> Self {
        JwtMiddlewareLayer
    }
}

impl<S> Layer<S> for JwtMiddlewareLayer {
    type Service = JwtMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        JwtMiddleware { inner }
    }
}

pub struct JwtMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for JwtMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // 認証なしで呼び出し可能な API
        let exempt_paths = vec![
            "/api/auth/guest_login",
            "/api/auth/signup",
            "/api/auth/login",
            "/api/auth/current_user",
        ];

        let path = req.uri().path();

        if exempt_paths.contains(&path) {
            // 認証なしでリクエストを通す
            let fut = self.inner.call(req);
            return Box::pin(async move { fut.await });
        }

        let headers = req.headers();

        match jwt::verify(&headers) {
            Ok(_) => {
                // JWTが有効なら次のサービスを呼び出す
                let fut = self.inner.call(req);
                Box::pin(async move { fut.await })
            }
            Err(_) => {
                // JWTが無効なら401 Unauthorizedを返却
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Unauthorized"))
                    .unwrap();
                Box::pin(async move { Ok(response) })
            }
        }
    }
}
