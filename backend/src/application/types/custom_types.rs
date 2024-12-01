use std::sync::Arc;
use crate::{
    domain::services::{auth_service::AuthService, todo_service::TodoService},
    domain::repositories::{auth_repository::AuthRepository, todo_repository::TodoRepository}
};

pub type AuthServiceArc = Arc<dyn AuthService>;
pub type AuthRepositoryArc = Arc<dyn AuthRepository>;

pub type TodoServiceArc = Arc<dyn TodoService>;
pub type TodoRepositoryArc = Arc<dyn TodoRepository>;