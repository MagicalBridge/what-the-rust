use crate::database::DatabasePool;
use crate::errors::AppError;
use crate::models::user::{CreateUserRequest, UpdateUserRequest, User};
use sqlx::Row;
use uuid::Uuid;

/// 用户服务
#[derive(Clone)]
pub struct UserService {
    pool: DatabasePool,
}

impl UserService {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    /// 创建新用户
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User, AppError> {
        // 验证输入数据
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e))?;

        // 检查用户名是否已存在
        if self.username_exists(&request.username).await? {
            return Err(AppError::Conflict("用户名已存在".to_string()));
        }

        // 检查邮箱是否已存在
        if self.email_exists(&request.email).await? {
            return Err(AppError::Conflict("邮箱已存在".to_string()));
        }

        // 加密密码
        let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)?;

        // 插入用户数据
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, password_hash, full_name)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, email, password_hash, full_name, created_at, updated_at
            "#,
        )
        .bind(&request.username)
        .bind(&request.email)
        .bind(&password_hash)
        .bind(&request.full_name)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// 根据 ID 获取用户
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, full_name, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// 根据用户名获取用户
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, full_name, created_at, updated_at FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// 获取所有用户
    pub async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, full_name, created_at, updated_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    /// 更新用户信息
    pub async fn update_user(
        &self,
        user_id: Uuid,
        request: UpdateUserRequest,
    ) -> Result<User, AppError> {
        // 验证输入数据
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e))?;

        // 检查用户是否存在
        let mut user = self
            .get_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

        // 更新字段
        if let Some(email) = &request.email {
            // 检查新邮箱是否已被其他用户使用
            if email != &user.email && self.email_exists(email).await? {
                return Err(AppError::Conflict("邮箱已存在".to_string()));
            }
            user.email = email.clone();
        }

        if let Some(full_name) = &request.full_name {
            user.full_name = full_name.clone();
        }

        if let Some(password) = &request.password {
            user.password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        }

        // 执行更新
        let updated_user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users 
            SET email = $2, full_name = $3, password_hash = $4, updated_at = NOW()
            WHERE id = $1
            RETURNING id, username, email, password_hash, full_name, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(&user.email)
        .bind(&user.full_name)
        .bind(&user.password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_user)
    }

    /// 删除用户
    pub async fn delete_user(&self, user_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 检查用户名是否存在
    async fn username_exists(&self, username: &str) -> Result<bool, AppError> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;

        let count: i64 = row.get("count");
        Ok(count > 0)
    }

    /// 检查邮箱是否存在
    async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.pool)
            .await?;

        let count: i64 = row.get("count");
        Ok(count > 0)
    }
}
