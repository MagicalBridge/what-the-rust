use crate::errors::AppError;
use crate::models::{ApiResponse, CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::services::UserService;
use actix_web::{HttpResponse, Result, web};
use uuid::Uuid;

/// 创建用户 (用户注册)
pub async fn create_user(
    user_service: web::Data<UserService>,
    request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let req_data = request.into_inner();

    log::info!(
        "🆕 创建用户请求: username={}, email={}, full_name={}",
        req_data.username,
        req_data.email,
        req_data.full_name
    );

    let user = user_service.create_user(req_data).await?;

    log::info!(
        "✅ 用户创建成功: id={}, username={}",
        user.id,
        user.username
    );

    let response = ApiResponse::success_with_print(UserResponse::from(user), "用户注册成功");
    Ok(HttpResponse::Created().json(response))
}

/// 根据 ID 获取用户信息
pub async fn get_user_by_id(
    user_service: web::Data<UserService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    log::info!("🔍 查询用户: id={}", user_id);

    match user_service.get_user_by_id(user_id).await? {
        Some(user) => {
            let response = ApiResponse::success_with_print(UserResponse::from(user), "获取用户信息成功");
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(AppError::NotFound("用户不存在".to_string())),
    }
}

/// 根据用户名获取用户信息
pub async fn get_user_by_username(
    user_service: web::Data<UserService>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let username = path.into_inner();

    match user_service.get_user_by_username(&username).await? {
        Some(user) => {
            let response = ApiResponse::success_with_print(UserResponse::from(user), "获取用户信息成功");
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(AppError::NotFound("用户不存在".to_string())),
    }
}

/// 获取所有用户列表
pub async fn get_all_users(user_service: web::Data<UserService>) -> Result<HttpResponse, AppError> {
    log::info!("📋 获取所有用户列表请求");

    let users = user_service.get_all_users().await?;

    log::info!("✅ 获取用户列表成功: 共{}个用户", users.len());
    let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    let response = ApiResponse::success_with_print(user_responses, "获取用户列表成功");
    Ok(HttpResponse::Ok().json(response))
}

/// 更新用户信息
pub async fn update_user(
    user_service: web::Data<UserService>,
    path: web::Path<Uuid>,
    request: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let req_data = request.into_inner();

    log::info!(
        "✏️ 更新用户: id={}, 更新字段: email={:?}, full_name={:?}, password_changed={}",
        user_id,
        req_data.email,
        req_data.full_name,
        req_data.password.is_some()
    );

    let user = user_service.update_user(user_id, req_data).await?;

    log::info!(
        "✅ 用户更新成功: id={}, username={}",
        user.id,
        user.username
    );

    let response = ApiResponse::success(UserResponse::from(user), "用户信息更新成功");
    Ok(HttpResponse::Ok().json(response))
}

/// 删除用户
pub async fn delete_user(
    user_service: web::Data<UserService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    log::info!("🗑️ 删除用户请求: id={}", user_id);

    if user_service.delete_user(user_id).await? {
        log::info!("✅ 用户删除成功: id={}", user_id);
        let response = ApiResponse::success((), "用户删除成功");
        Ok(HttpResponse::Ok().json(response))
    } else {
        Err(AppError::NotFound("用户不存在".to_string()))
    }
}
