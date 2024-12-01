use std::sync::Arc;

use async_trait::async_trait;
use bb8_postgres::{bb8::Pool, PostgresConnectionManager};
use postgres::NoTls;
use validator::Validate;
use lambda_http::tracing::error;

use crate::{
    application::errors::auth_error::AuthError,
    application::types::custom_types::AuthRepositoryArc,
    domain::entities::{auth::{LoginRequest, SignupRequest}, user::User}
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
    async fn signup(&self, req: &SignupRequest) -> Result<(), AuthError>;
    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
}

pub struct AuthServiceImpl {
    auth_repository: AuthRepositoryArc,
    pool: Arc<Pool<PostgresConnectionManager<NoTls>>>
}

impl AuthServiceImpl {
    pub fn new(auth_repository: AuthRepositoryArc, pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        AuthServiceImpl { auth_repository, pool: Arc::new(pool) }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError> {
        let user_opt = self.auth_repository.guest_login(req).await?;
        match user_opt {
            Some(user) => Ok(Some(user)),
            None => Ok(None)
        }
    }

    async fn signup(&self, req: &SignupRequest) -> Result<(), AuthError> {
        let pool = self.pool.clone();
        let mut conn = pool.get().await.map_err(AuthError::from)?;
        let mut tx = conn.transaction().await.map_err(AuthError::from)?;

        let result = self.auth_repository.signup(req, &mut tx).await;

        match result {
            Ok(value) => {
                tx.commit().await.map_err(AuthError::from)?;
                Ok(value)
            }
            Err(auth_error) => {
                tx.rollback().await.map_err(AuthError::from)?;
                error!("[auth_service] - [signup] - [message: auth_error = {}]", auth_error);

                Err(auth_error)
            }
        }
    }

    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError> {
        if let Err(validation_errors) = req.validate() {
            return Err(AuthError::ValidationError(validation_errors));
        }

        self.auth_repository.login(req).await
    }
}