use crate::database::RedisPool;
use crate::errors::AppError;
use redis::Commands;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// 缓存服务
#[derive(Clone)]
pub struct CacheService {
    redis_pool: RedisPool,
    ttl_seconds: u64,
}

impl CacheService {
    /// 创建新的缓存服务实例
    pub fn new(redis_pool: RedisPool, ttl_seconds: u64) -> Self {
        Self {
            redis_pool,
            ttl_seconds,
        }
    }

    /// 获取缓存数据
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, AppError>
    where
        T: DeserializeOwned + Debug,
    {
        let mut conn = self.redis_pool.lock().await;
        
        match conn.get::<_, Option<String>>(key) {
            Ok(Some(data)) => {
                log::debug!("🎯 缓存命中: {}", key);
                match serde_json::from_str(&data) {
                    Ok(value) => Ok(Some(value)),
                    Err(e) => {
                        log::error!("❌ 缓存数据反序列化失败: {}, key: {}", e, key);
                        // 删除损坏的缓存数据
                        let _: Result<(), _> = conn.del(key);
                        Ok(None)
                    }
                }
            }
            Ok(None) => {
                log::debug!("❌ 缓存未命中: {}", key);
                Ok(None)
            }
            Err(e) => {
                log::error!("❌ Redis查询失败: {}, key: {}", e, key);
                // 缓存失败不应该影响主流程，返回None让其回落到数据库查询
                Ok(None)
            }
        }
    }

    /// 设置缓存数据
    pub async fn set<T>(&self, key: &str, value: &T) -> Result<(), AppError>
    where
        T: Serialize + Debug,
    {
        let mut conn = self.redis_pool.lock().await;
        
        match serde_json::to_string(value) {
            Ok(serialized) => {
                match conn.set_ex::<_, _, ()>(key, serialized, self.ttl_seconds) {
                    Ok(_) => {
                        log::debug!("💾 缓存设置成功: {}, TTL: {}秒", key, self.ttl_seconds);
                        Ok(())
                    }
                    Err(e) => {
                        log::error!("❌ Redis设置失败: {}, key: {}", e, key);
                        // 缓存设置失败不应该影响主流程
                        Ok(())
                    }
                }
            }
            Err(e) => {
                log::error!("❌ 缓存数据序列化失败: {}, key: {}", e, key);
                Ok(())
            }
        }
    }

    /// 删除缓存数据
    pub async fn delete(&self, key: &str) -> Result<(), AppError> {
        let mut conn = self.redis_pool.lock().await;
        
        match conn.del::<_, ()>(key) {
            Ok(_) => {
                log::debug!("🗑️ 缓存删除成功: {}", key);
                Ok(())
            }
            Err(e) => {
                log::error!("❌ Redis删除失败: {}, key: {}", e, key);
                Ok(())
            }
        }
    }

    /// 删除多个缓存key
    pub async fn delete_many(&self, keys: &[&str]) -> Result<(), AppError> {
        if keys.is_empty() {
            return Ok(());
        }

        let mut conn = self.redis_pool.lock().await;
        
        match conn.del::<_, ()>(keys) {
            Ok(_) => {
                log::debug!("🗑️ 批量缓存删除成功: {:?}", keys);
                Ok(())
            }
            Err(e) => {
                log::error!("❌ Redis批量删除失败: {}, keys: {:?}", e, keys);
                Ok(())
            }
        }
    }

    /// 检查缓存key是否存在
    pub async fn exists(&self, key: &str) -> Result<bool, AppError> {
        let mut conn = self.redis_pool.lock().await;
        
        match conn.exists(key) {
            Ok(exists) => Ok(exists),
            Err(e) => {
                log::error!("❌ Redis检查key存在性失败: {}, key: {}", e, key);
                Ok(false)
            }
        }
    }

    /// 生成用户相关的缓存key
    pub fn user_cache_key(user_id: &uuid::Uuid) -> String {
        format!("user:id:{}", user_id)
    }

    /// 生成用户名查询的缓存key
    pub fn username_cache_key(username: &str) -> String {
        format!("user:username:{}", username)
    }

    /// 生成所有用户列表的缓存key
    pub fn all_users_cache_key() -> String {
        "users:all".to_string()
    }

    /// 删除用户相关的所有缓存
    pub async fn invalidate_user_cache(&self, user_id: &uuid::Uuid, username: &str) -> Result<(), AppError> {
        let user_key = Self::user_cache_key(user_id);
        let username_key = Self::username_cache_key(username);
        let all_users_key = Self::all_users_cache_key();
        
        let keys = vec![
            user_key.as_str(),
            username_key.as_str(),
            all_users_key.as_str(),
        ];
        
        self.delete_many(&keys).await
    }
}