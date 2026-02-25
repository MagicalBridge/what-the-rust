use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred")
            }
            AppError::Validation(err) => {
                tracing::warn!("Validation error: {:?}", err);
                (StatusCode::BAD_REQUEST, "Validation failed")
            }
            AppError::NotFound(ref message) => {
                tracing::warn!("Not found: {}", message);
                (StatusCode::NOT_FOUND, message.as_str())
            }
            AppError::Conflict(ref message) => {
                tracing::warn!("Conflict: {}", message);
                (StatusCode::CONFLICT, message.as_str())
            }
            AppError::BadRequest(ref message) => {
                tracing::warn!("Bad request: {}", message);
                (StatusCode::BAD_REQUEST, message.as_str())
            }
            AppError::InternalServerError(ref message) => {
                tracing::error!("Internal server error: {}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

// 为特定错误类型创建便捷构造函数
impl AppError {
    pub fn user_not_found(id: &str) -> Self {
        AppError::NotFound(format!("User with id {} not found", id))
    }

    pub fn email_already_exists(email: &str) -> Self {
        AppError::Conflict(format!("User with email {} already exists", email))
    }

    pub fn invalid_id() -> Self {
        AppError::BadRequest("Invalid user ID format".to_string())
    }
}