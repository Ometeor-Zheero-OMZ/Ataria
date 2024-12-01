use tokio_postgres::Error;
use lambda_http::tracing::error;

use crate::application::jwt::jwt::Claims;

pub async fn get_user_id(user: &Claims, transaction: &tokio_postgres::Transaction<'_>) -> Result<i32, Error> {
    // ユーザーの存在チェック
    let row_result = transaction.query_one(
        r#"
            SELECT
                user_id
            FROM
                user_profiles
            WHERE
                email = $1
        "#,
        &[&user.sub]
    ).await;

    match row_result {
        Ok(user_row) => {
            let user_id = user_row.get("user_id");

            Ok(user_id)
        },
        Err(error) => {
            error!("[user_service] - [get_user_id] - [message: error = {}]", error);
            Err(error)
        }
    }
}