use bcrypt::{hash, verify, DEFAULT_COST};
use crate::errors::AppError;

/// 密码工具类
pub struct PasswordUtils;

impl PasswordUtils {
    /// 加密密码
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        hash(password, DEFAULT_COST).map_err(AppError::from)
    }

    /// 验证密码
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
        verify(password, hash).map_err(AppError::from)
    }
}