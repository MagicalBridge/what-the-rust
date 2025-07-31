use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 用户数据模型
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] // 密码不序列化到响应中
    pub password_hash: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 用户注册请求数据
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
}

impl CreateUserRequest {
    /// 验证用户注册数据
    pub fn validate(&self) -> Result<(), String> {
        if self.username.is_empty() {
            return Err("用户名不能为空".to_string());
        }
        
        if self.username.len() < 3 {
            return Err("用户名至少需要3个字符".to_string());
        }
        
        if self.email.is_empty() {
            return Err("邮箱不能为空".to_string());
        }
        
        if !self.email.contains('@') {
            return Err("邮箱格式无效".to_string());
        }
        
        if self.password.len() < 6 {
            return Err("密码至少需要6个字符".to_string());
        }
        
        if self.full_name.is_empty() {
            return Err("姓名不能为空".to_string());
        }
        
        Ok(())
    }
}

/// 用户更新请求数据
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub password: Option<String>,
}

impl UpdateUserRequest {
    /// 验证用户更新数据
    pub fn validate(&self) -> Result<(), String> {
        if let Some(email) = &self.email {
            if email.is_empty() {
                return Err("邮箱不能为空".to_string());
            }
            if !email.contains('@') {
                return Err("邮箱格式无效".to_string());
            }
        }
        
        if let Some(password) = &self.password {
            if password.len() < 6 {
                return Err("密码至少需要6个字符".to_string());
            }
        }
        
        if let Some(full_name) = &self.full_name {
            if full_name.is_empty() {
                return Err("姓名不能为空".to_string());
            }
        }
        
        Ok(())
    }
}

/// 用户响应数据 (不包含密码)
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            full_name: user.full_name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}