use std::fmt;
use bb8_postgres::bb8;
use tokio_postgres;
use argon2;
use jsonwebtoken;

#[derive(Debug)]
pub enum TodoError {
    DatabaseError(String),
    PoolError(bb8::RunError<tokio_postgres::Error>),
    HashingError(argon2::password_hash::Error),
    TokenCreationError(jsonwebtoken::errors::Error),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::DatabaseError(err) => write!(f, "Database connection error: {}", err),
            TodoError::PoolError(err) => write!(f, "Pool error: {}", err),
            TodoError::HashingError(err) => write!(f, "Password hashing error: {}", err),
            TodoError::TokenCreationError(err) => write!(f, "JWT error: {}", err),
        }
    }
}

impl std::error::Error for TodoError {}

impl From<tokio_postgres::Error> for TodoError {
    fn from(error: tokio_postgres::Error) -> Self {
        TodoError::DatabaseError(error.to_string())
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for TodoError {
    fn from(error: bb8::RunError<tokio_postgres::Error>) -> Self {
        TodoError::PoolError(error)
    }
}

impl From<argon2::password_hash::Error> for TodoError {
    fn from(error: argon2::password_hash::Error) -> Self {
        TodoError::HashingError(error)
    }
}

impl From<jsonwebtoken::errors::Error> for TodoError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        TodoError::TokenCreationError(error)
    }
}