use async_trait::async_trait;
use bcrypt::verify;
use tokio_postgres::{NoTls, Transaction};
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString
    },
    Argon2
};
use lambda_http::tracing::error;

use crate::application::jwt::jwt;
use crate::domain::entities::{auth::{LoginRequest, SignupRequest}, user::User};
use crate::application::errors::auth_error::AuthError;


#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
    async fn signup(&self, req: &SignupRequest, tx: &mut Transaction<'_>) -> Result<(), AuthError>;
    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
}

pub struct AuthRepositoryImpl {
    pool: Pool<PostgresConnectionManager<NoTls>>
}

impl AuthRepositoryImpl {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        AuthRepositoryImpl { pool }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn guest_login(
        &self,
        req: &LoginRequest,
    ) -> Result<Option<User>, AuthError> {
        let conn = self.pool.get().await?;
        
        let rows = conn.query(
            r#"
                SELECT 
                    users.id,
                    user_profiles.name,
                    user_profiles.email,
                    user_auth.password
                FROM
                    users
                INNER JOIN
                    user_auth
                ON
                    user_auth.user_id = users.id
                INNER JOIN
                    user_profiles
                ON
                    user_profiles.user_id = user_auth.user_id
                WHERE
                    email = $1;
            "#,
            &[&req.email]
        ).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let id: i32 = rows.get(0).unwrap().get("id");
        let password: String = rows.get(0).unwrap().get("password");

        if verify(&req.password, &password).is_err() {
            error!("[auth_repository] - [guest_login] - [Authentication Failed]");
            return Ok(None);
        }

        match jwt::create_token(&req.email, &id) {
            Ok(token) => {
                let user_data = User {
                    id,
                    name: req.name.clone(),
                    email: req.email.clone(),
                    token,
                };
                Ok(Some(user_data))
            }
            Err(error) => {
                error!("[auth_repository] - [guest_login] - [message: error = {}]", error);
                Err(AuthError::TokenCreationError(error))
            }
        }
    }

    async fn signup(
        &self,
        req: &SignupRequest,
        tx: &mut Transaction<'_>
    ) -> Result<(), AuthError> {
        let result_row = tx.query_one(
            r#"
                INSERT INTO users (
                    created_at,
                    updated_at
                ) VALUES (
                    CURRENT_TIMESTAMP,
                    CURRENT_TIMESTAMP
                )
                RETURNING id;
            "#,
            &[]
        ).await?;

        let user_id: i32 = result_row.get("id");

        tx.execute(
            r#"
                INSERT INTO user_profiles (
                    user_id,
                    name,
                    email,
                    created_at,
                    updated_at
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    CURRENT_TIMESTAMP,
                    CURRENT_TIMESTAMP
                );
            "#,
            &[&user_id, &req.name, &req.email]
        ).await?;

        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = argon2.hash_password(&req.password.as_bytes(), &salt)?.to_string();

        tx.execute(
            r#"
                INSERT INTO user_auth (
                    user_id,
                    password,
                    created_at,
                    updated_at
                ) VALUES (
                    $1,
                    $2,
                    CURRENT_TIMESTAMP,
                    CURRENT_TIMESTAMP
                );
            "#,
            &[&user_id, &hashed_password]
        ).await?;

        Ok(())
    }

    async fn login(
        &self,
        req: &LoginRequest
    ) -> Result<Option<User>, AuthError> {
        let conn = self.pool.get().await?;

        let rows = conn.query(
            r#"
                SELECT 
                    users.id,
                    user_profiles.name,
                    user_profiles.email,
                    user_auth.password
                FROM
                    users
                INNER JOIN
                    user_auth
                ON
                    user_auth.user_id = users.id
                INNER JOIN
                    user_profiles
                ON
                    user_profiles.user_id = user_auth.user_id
                WHERE
                    user_profiles.name = $1
                    AND user_profiles.email = $2;
            "#,
            &[&req.name, &req.email]
        ).await?;

        // ユーザー情報が取得できない場合 None を返却
        if rows.is_empty() {
            return Ok(None);
        }

        let id: i32 = rows.get(0).unwrap().get("id");
        let name: String = rows.get(0).unwrap().get("name");
        let email: String = rows.get(0).unwrap().get("email");
        let hashed_password: String = rows.get(0).unwrap().get("password");

        // Argon2を初期化
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&hashed_password)?;

        // パスワードが一致しない場合、None を返却
        if argon2.verify_password(req.password.as_bytes(), &parsed_hash).is_err() {
            error!("[auth_repository] - [login] - [message: Authentication Failed]");
            return Ok(None);
        };

        let token = jwt::create_token(&req.email, &id)?;

        let user_data = User {
            id,
            name,
            email,
            token,
        };

        Ok(Some(user_data))
    }
}