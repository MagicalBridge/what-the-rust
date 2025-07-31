use actix_web::{HttpResponse, ResponseError};
use crate::models::ApiResponse;
use std::fmt;
use std::error::Error;

/// 应用程序错误类型
#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    ValidationError(String),
    NotFound(String),
    Conflict(String),
    InternalServerError(String),
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "数据库错误: {}", e),
            AppError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
            AppError::NotFound(msg) => write!(f, "未找到: {}", msg),
            AppError::Conflict(msg) => write!(f, "冲突: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "内部服务器错误: {}", msg),
            AppError::BadRequest(msg) => write!(f, "请求错误: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(_) => {
                let response = ApiResponse::<()>::error("数据库操作失败");
                HttpResponse::InternalServerError().json(response)
            }
            AppError::ValidationError(msg) => {
                let response = ApiResponse::<()>::error(msg);
                HttpResponse::BadRequest().json(response)
            }
            AppError::NotFound(msg) => {
                let response = ApiResponse::<()>::error(msg);
                HttpResponse::NotFound().json(response)
            }
            AppError::Conflict(msg) => {
                let response = ApiResponse::<()>::error(msg);
                HttpResponse::Conflict().json(response)
            }
            AppError::InternalServerError(msg) => {
                let response = ApiResponse::<()>::error(msg);
                HttpResponse::InternalServerError().json(response)
            }
            AppError::BadRequest(msg) => {
                let response = ApiResponse::<()>::error(msg);
                HttpResponse::BadRequest().json(response)
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::DatabaseError(error)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError::InternalServerError(error.to_string())
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(error: bcrypt::BcryptError) -> Self {
        AppError::InternalServerError(format!("密码加密错误: {}", error))
    }
}

impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(error: sqlx::migrate::MigrateError) -> Self {
        AppError::DatabaseError(sqlx::Error::Migrate(Box::new(error)))
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::DatabaseError(e) => Some(e),
            _ => None,
        }
    }
}