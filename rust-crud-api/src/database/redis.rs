use crate::config::Config;
use redis::{Client, Connection, RedisError};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Redis连接池类型别名
pub type RedisPool = Arc<Mutex<Connection>>;

/// 创建Redis连接池
pub async fn create_redis_pool(config: &Config) -> Result<RedisPool, RedisError> {
    log::info!("🔗 正在连接Redis: {}", config.redis_url);
    
    let client = Client::open(config.redis_url.as_str())?;
    let connection = client.get_connection()?;
    
    log::info!("✅ Redis连接建立成功");
    
    Ok(Arc::new(Mutex::new(connection)))
}

/// 测试Redis连接
pub async fn test_redis_connection(pool: &RedisPool) -> Result<(), RedisError> {
    use redis::Commands;
    
    let mut conn = pool.lock().await;
    let _: String = conn.set("test_key", "test_value")?;
    let result: String = conn.get("test_key")?;
    
    if result == "test_value" {
        let _: () = conn.del("test_key")?;
        log::info!("✅ Redis连接测试成功");
        Ok(())
    } else {
        Err(RedisError::from((redis::ErrorKind::ResponseError, "Redis连接测试失败")))
    }
}