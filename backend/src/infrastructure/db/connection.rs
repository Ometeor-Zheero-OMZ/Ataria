use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use tokio_postgres::NoTls;

use crate::infrastructure::config::config::get_config;

pub async fn get_db_pool () -> Pool<PostgresConnectionManager<NoTls>> {
    let config = get_config();
    let pg_mgr = PostgresConnectionManager::new(config, NoTls);
    let pool = Pool::builder().max_size(100).build(pg_mgr).await.unwrap();
    return pool;
}