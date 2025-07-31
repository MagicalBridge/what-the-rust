use sqlx::{PgPool, Pool, Postgres};
use crate::config::Config;
use crate::errors::AppError;

/// 数据库连接池类型别名
pub type DatabasePool = Pool<Postgres>;

/// 创建数据库连接池
pub async fn create_pool(config: &Config) -> Result<DatabasePool, AppError> {
    let pool = PgPool::connect(&config.database_url).await?;
    
    // 测试连接
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;
    
    println!("✅ 数据库连接池创建成功");
    Ok(pool)
}