use sqlx::SqlitePool;

pub async fn create_connection_pool() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect("sqlite://database.db").await;
    pool
}