use crate::db::DbPool;
use crate::error::AppError;
use crate::models::user::{CreateUserRequest, UpdateUserRequest, User, UserQueryParams};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use serde_json::json;
use tracing::info;
use uuid::Uuid;
use validator::Validate;

// 创建用户
pub async fn create_user(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    // 验证输入
    payload.validate()
        .map_err(AppError::Validation)?;

    // 检查邮箱是否已存在
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, age, created_at, updated_at FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await?;

    if existing_user.is_some() {
        return Err(AppError::email_already_exists(&payload.email));
    }

    // 创建新用户
    let user_id = Uuid::new_v4();
    let now = Utc::now();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, name, email, age, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, email, age, created_at, updated_at
        "#
    )
    .bind(user_id)
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(payload.age)
    .bind(now)
    .bind(now)
    .fetch_one(&pool)
    .await?;

    info!("Created user: {}", user.id);
    
    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message": "User created successfully",
            "user": user
        }))
    ))
}

// 获取所有用户（支持分页和过滤）
pub async fn get_users(
    State(pool): State<DbPool>,
    Query(params): Query<UserQueryParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.get_page();
    let limit = params.get_limit();
    let offset = params.get_offset();

    // 构建查询条件
    let mut query_conditions = Vec::new();
    let mut param_index = 1;

    if let Some(_name) = &params.name {
        query_conditions.push(format!("name ILIKE ${}", param_index));
        param_index += 1;
    }

    if let Some(_email) = &params.email {
        query_conditions.push(format!("email ILIKE ${}", param_index));
        param_index += 1;
    }

    if let Some(_min_age) = params.min_age {
        query_conditions.push(format!("age >= ${}", param_index));
        param_index += 1;
    }

    if let Some(_max_age) = params.max_age {
        query_conditions.push(format!("age <= ${}", param_index));
        param_index += 1;
    }

    let where_clause = if query_conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", query_conditions.join(" AND "))
    };

    // 获取总数
    let count_query = format!(
        "SELECT COUNT(*) FROM users {}",
        where_clause
    );

    let mut count_query_builder = sqlx::query_scalar::<_, i64>(&count_query);

    if let Some(name) = &params.name {
        count_query_builder = count_query_builder.bind(format!("%{}%", name));
    }

    if let Some(email) = &params.email {
        count_query_builder = count_query_builder.bind(format!("%{}%", email));
    }

    if let Some(min_age) = params.min_age {
        count_query_builder = count_query_builder.bind(min_age);
    }

    if let Some(max_age) = params.max_age {
        count_query_builder = count_query_builder.bind(max_age);
    }

    let total = count_query_builder.fetch_one(&pool).await?;

    // 获取用户列表
    let users_query = format!(
        r#"
        SELECT id, name, email, age, created_at, updated_at 
        FROM users {} 
        ORDER BY created_at DESC 
        LIMIT ${} OFFSET ${}
        "#,
        where_clause,
        param_index,
        param_index + 1
    );

    let mut users_query_builder = sqlx::query_as::<_, User>(&users_query);

    if let Some(name) = &params.name {
        users_query_builder = users_query_builder.bind(format!("%{}%", name));
    }

    if let Some(email) = &params.email {
        users_query_builder = users_query_builder.bind(format!("%{}%", email));
    }

    if let Some(min_age) = params.min_age {
        users_query_builder = users_query_builder.bind(min_age);
    }

    if let Some(max_age) = params.max_age {
        users_query_builder = users_query_builder.bind(max_age);
    }

    users_query_builder = users_query_builder.bind(limit as i64).bind(offset as i64);

    let users = users_query_builder.fetch_all(&pool).await?;

    let total_pages = (total as f64 / limit as f64).ceil() as i64;

    Ok(Json(json!({
        "users": users,
        "pagination": {
            "page": page,
            "limit": limit,
            "total": total,
            "total_pages": total_pages
        }
    })))
}

// 根据ID获取用户
pub async fn get_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<String>,
) -> Result<Json<User>, AppError> {
    let uuid = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::invalid_id())?;

    let user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, age, created_at, updated_at FROM users WHERE id = $1"
    )
    .bind(uuid)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::user_not_found(&user_id))?;

    Ok(Json(user))
}

// 更新用户
pub async fn update_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // 验证输入
    payload.validate()
        .map_err(AppError::Validation)?;

    let uuid = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::invalid_id())?;

    // 检查用户是否存在
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, age, created_at, updated_at FROM users WHERE id = $1"
    )
    .bind(uuid)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::user_not_found(&user_id))?;

    // 如果更新邮箱，检查新邮箱是否已被其他用户使用
    if let Some(ref email) = payload.email {
        if email != &existing_user.email {
            let email_user = sqlx::query_as::<_, User>(
                "SELECT id, name, email, age, created_at, updated_at FROM users WHERE email = $1 AND id != $2"
            )
            .bind(email)
            .bind(uuid)
            .fetch_optional(&pool)
            .await?;

            if email_user.is_some() {
                return Err(AppError::email_already_exists(email));
            }
        }
    }

    // 构建动态更新查询
    let mut update_fields = Vec::new();
    let mut param_index = 1;

    if payload.name.is_some() {
        update_fields.push(format!("name = ${}", param_index));
        param_index += 1;
    }

    if payload.email.is_some() {
        update_fields.push(format!("email = ${}", param_index));
        param_index += 1;
    }

    if payload.age.is_some() {
        update_fields.push(format!("age = ${}", param_index));
        param_index += 1;
    }

    if update_fields.is_empty() {
        return Err(AppError::BadRequest("No fields to update".to_string()));
    }

    let update_query = format!(
        "UPDATE users SET {} WHERE id = ${} RETURNING id, name, email, age, created_at, updated_at",
        update_fields.join(", "),
        param_index
    );

    let mut query_builder = sqlx::query_as::<_, User>(&update_query);

    if let Some(name) = payload.name {
        query_builder = query_builder.bind(name);
    }

    if let Some(email) = payload.email {
        query_builder = query_builder.bind(email);
    }

    if let Some(age) = payload.age {
        query_builder = query_builder.bind(age);
    }

    query_builder = query_builder.bind(uuid);

    let updated_user = query_builder.fetch_one(&pool).await?;

    info!("Updated user: {}", updated_user.id);

    Ok(Json(json!({
        "message": "User updated successfully",
        "user": updated_user
    })))
}

// 删除用户
pub async fn delete_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let uuid = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::invalid_id())?;

    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uuid)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::user_not_found(&user_id));
    }

    info!("Deleted user: {}", user_id);

    Ok(StatusCode::NO_CONTENT)
}

// 健康检查端点
pub async fn health_check() -> &'static str {
    "OK"
}