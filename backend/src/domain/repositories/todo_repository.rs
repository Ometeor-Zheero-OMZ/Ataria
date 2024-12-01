use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::{Transaction, NoTls};
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use crate::domain::entities::todo::*;
use crate::application::errors::todo_error::TodoError;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_todos(&self,
        user_id: i32,
        tx: &mut Transaction<'_>
    ) -> Result<Vec<TodoItem>, TodoError>;

    async fn create_todo(
        &self,
        user_id: i32,
        todo_req: &RequestCreateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<ResponseCreateTodoItem, TodoError>;

    async fn update_todo(
        &self,
        todo_req: &RequestUpdateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError>;

    async fn delete_todo(
        &self,
        todo_req: &RequestDeleteTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError>;
    
    async fn complete_todo(
        &self,
        todo_req: &RequestCompleteTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError>;
}

pub struct TodoRepositoryImpl {
    #[allow(dead_code)]
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl TodoRepositoryImpl {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        TodoRepositoryImpl { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn get_todos(
        &self,
        user_id: i32,
        tx: &mut Transaction<'_>
    ) -> Result<Vec<TodoItem>, TodoError> {
        let rows = tx.query(
            r#"
                SELECT
                    *
                FROM
                    todos
                WHERE
                    user_id = $1
            "#,
            &[&user_id],
        ).await?;

        let todos: Vec<TodoItem> = rows.into_iter().map(|row| {
            // データベースから抽出したタイムスタンプを YYYY-MM-dd HH:mm:ss フォーマットに変換
            let convert_timestamp = |time: SystemTime| -> NaiveDateTime {
                let duration = time.duration_since(UNIX_EPOCH).unwrap();
                DateTime::<Utc>::from_timestamp(duration.as_secs() as i64, 0)
                    .unwrap()
                    .naive_utc()
            };

            TodoItem {
                id: row.get("id"),
                user_id: row.get("user_id"),
                game_id: row.get("game_id"),
                title: row.get("title"),
                description: row.get("description"),
                is_completed: row.get("is_completed"),
                status: row.get("status"),
                priority: row.get("priority"),
                difficulty: row.get("difficulty"),
                deadline: row.get::<_, Option<SystemTime>>("deadline").map(convert_timestamp),
                created_at: convert_timestamp(row.get("created_at")),
                updated_at: convert_timestamp(row.get("updated_at")),
                deleted_at: row.get::<_, Option<SystemTime>>("deleted_at").map(convert_timestamp),
            }
        }).collect();

        Ok(todos)
    }

    async fn create_todo(
        &self,
        user_id: i32,
        todo_req: &RequestCreateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<ResponseCreateTodoItem, TodoError> {
        let row = tx.query_one(
            r#"
                INSERT INTO todos (
                    user_id,
                    title,
                    description,
                    is_completed
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    false
                )
                RETURNING *
            "#,
            &[
                &user_id,
                &todo_req.title,
                &todo_req.description,
            ]
        ).await?;

        Ok(ResponseCreateTodoItem {
            title: row.get("title"),
            description: row.get("description"),
            is_completed: row.get("is_completed")
        })
    }

    async fn delete_todo(
        &self,
        todo_req: &RequestDeleteTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError> {
        tx.execute(
            r#"
                UPDATE
                    todos
                SET
                    deleted_at = CURRENT_TIMESTAMP
                WHERE
                    id = $1
            "#,
            &[&todo_req.id]
        ).await?;

        Ok(())
    }

    async fn update_todo(
        &self,
        todo_req: &RequestUpdateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError> {
        tx.execute(
            r#"
                UPDATE
                    todos
                SET
                    title = $2,
                    description = $3,
                    is_completed = $4,
                    updated_at = CURRENT_TIMESTAMP
                WHERE
                    id = $1
            "#,
            &[
                &todo_req.id,
                &todo_req.title,
                &todo_req.description,
                &todo_req.is_completed
            ]
        ).await?;

        Ok(())
    }

    async fn complete_todo(
        &self,
        todo_req: &RequestCompleteTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError> {
        tx.execute(
            r#"
                UPDATE
                    todos
                SET
                    deleted_at = CURRENT_TIMESTAMP,
                    is_completed = true
                WHERE
                    id = $1
            "#,
            &[&todo_req.id]
        ).await?;

        Ok(())
    }
}