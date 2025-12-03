use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,
    
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(range(min = 0, max = 150, message = "Age must be between 0 and 150"))]
    pub age: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: Option<String>,
    
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    
    #[validate(range(min = 0, max = 150, message = "Age must be between 0 and 150"))]
    pub age: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UserQueryParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub min_age: Option<i32>,
    pub max_age: Option<i32>,
}

impl Default for UserQueryParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(10),
            name: None,
            email: None,
            min_age: None,
            max_age: None,
        }
    }
}

impl UserQueryParams {
    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or(1)
    }

    pub fn get_limit(&self) -> u32 {
        self.limit.unwrap_or(10).min(100) // 限制最大值为100
    }

    pub fn get_offset(&self) -> u32 {
        (self.get_page() - 1) * self.get_limit()
    }
}