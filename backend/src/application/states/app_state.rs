// use std::sync::Arc;
// use bb8_postgres::{bb8::Pool, PostgresConnectionManager};
// use postgres::NoTls;

// use crate::application::types::custom_types::{AuthRepositoryArc, AuthServiceArc, TodoRepositoryArc, TodoServiceArc};
// use crate::domain::services::auth_service::AuthServiceImpl;
// use crate::domain::services::todo_service::TodoServiceImpl;
// use crate::domain::repositories::auth_repository::AuthRepositoryImpl;
// use crate::domain::repositories::todo_repository::TodoRepositoryImpl;

// #[derive(Clone)]
// pub struct AppState {
//     pub auth_service: AuthServiceArc,
//     pub todo_service: TodoServiceArc,
// }

// impl AppState {
//     pub fn init(pool: &Pool<PostgresConnectionManager<NoTls>>) -> AppState {
//         let auth_repository: AuthRepositoryArc = Arc::new(AuthRepositoryImpl::new(pool.clone()));
//         let todo_repository: TodoRepositoryArc = Arc::new(TodoRepositoryImpl::new(pool.clone()));
//         let auth_service: AuthServiceArc = Arc::new(AuthServiceImpl::new(auth_repository.clone(), pool.clone()));
//         let todo_service: TodoServiceArc = Arc::new(TodoServiceImpl::new(todo_repository.clone(), pool.clone()));

//         AppState {
//             auth_service,
//             todo_service,
//         }
//     }
// }
