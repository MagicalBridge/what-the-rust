use sqlx::{Pool, Postgres};
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

pub async fn create_connection_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    // 配置连接池
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await?;

    // 测试连接
    sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::migrate::MigrateError> {
    // 直接运行迁移，如果表已存在会报错，但我们可以在main.rs中处理
    sqlx::migrate!("src/db/migrations").run(pool).await
}