use axum_crud::db::{create_connection_pool, run_migrations};
use axum_crud::routes::user::create_user_routes;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::json;
use tower::ServiceExt;

// 创建测试应用
async fn create_app() -> Router {
    // 使用测试数据库
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost/axum_crud_test".to_string());
    
    let pool = create_connection_pool(&database_url).await.expect("Failed to create connection pool");
    
    // 运行迁移
    run_migrations(&pool).await.expect("Failed to run migrations");
    
    Router::new()
        .nest("/api/users", create_user_routes())
        .with_state(pool)
}

#[tokio::test]
async fn test_health_check() {
    let app = create_app().await;
    
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_user() {
    let app = create_app().await;
    
    let user_data = json!({
        "name": "Test User",
        "email": "test@example.com",
        "age": 30
    });
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/users")
        .header("content-type", "application/json")
        .body(Body::from(user_data.to_string()))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(response_json["user"]["id"].is_string());
    assert_eq!(response_json["user"]["name"], "Test User");
    assert_eq!(response_json["user"]["email"], "test@example.com");
    assert_eq!(response_json["user"]["age"], 30);
}

#[tokio::test]
async fn test_create_user_validation_error() {
    let app = create_app().await;
    
    // 无效的邮箱格式
    let user_data = json!({
        "name": "Test User",
        "email": "invalid-email",
        "age": 30
    });
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/users")
        .header("content-type", "application/json")
        .body(Body::from(user_data.to_string()))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_get_users() {
    let app = create_app().await;
    
    // 首先创建一个用户
    let user_data = json!({
        "name": "Test User",
        "email": "test2@example.com",
        "age": 25
    });
    
    let create_request = Request::builder()
        .method("POST")
        .uri("/api/users")
        .header("content-type", "application/json")
        .body(Body::from(user_data.to_string()))
        .unwrap();
    
    let _create_response = app.clone().oneshot(create_request).await.unwrap();
    
    // 获取用户列表
    let request = Request::builder()
        .method("GET")
        .uri("/api/users")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(response_json["users"].as_array().unwrap().len() > 0);
    assert!(response_json["pagination"]["total"].as_i64().unwrap() > 0);
}

#[tokio::test]
async fn test_get_user_by_id() {
    let app = create_app().await;
    
    // 首先创建一个用户
    let user_data = json!({
        "name": "Test User",
        "email": "test3@example.com",
        "age": 35
    });
    
    let create_request = Request::builder()
        .method("POST")
        .uri("/api/users")
        .header("content-type", "application/json")
        .body(Body::from(user_data.to_string()))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap();
    let create_json: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
    let user_id = create_json["user"]["id"].as_str().unwrap();
    
    // 获取用户
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/users/{}", user_id))
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["id"], user_id);
    assert_eq!(response_json["name"], "Test User");
    assert_eq!(response_json["email"], "test3@example.com");
    assert_eq!(response_json["age"], 35);
}

#[tokio::test]
async fn test_get_nonexistent_user() {
    let app = create_app().await;
    
    let request = Request::builder()
        .method("GET")
        .uri("/api/users/00000000-0000-0000-0000-000000000000")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_user() {
    let app = create_app().await;
    
    // 首先创建一个用户
    let user_data = json!({
        "name": "Test User",
        "email": "test4@example.com",
        "age": 40
    });
    
    let create_request = Request::builder()
        .method("POST")
        .uri("/api/users")
        .header("content-type", "application/json")
        .body(Body::from(user_data.to_string()))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap();
    let create_json: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
    let user_id = create_json["user"]["id"].as_str().unwrap();
    
    // 更新用户
    let update_data = json!({
        "name": "Updated User",
        "age": 45
    });
    
    let update_request = Request::builder()
        .method("PUT")
        .uri(&format!("/api/users/{}", user_id))
        .header("content-type", "application/json")
        .body(Body::from(update_data.to_string()))
        .unwrap();
    
    let response = app.clone().oneshot(update_request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["user"]["id"], user_id);
    assert_eq!(response_json["user"]["name"], "Updated User");
    assert_eq!(response_json["user"]["email"], "test4@example.com"); // 未更改
    assert_eq!(response_json["user"]["age"], 45);
}

#[tokio::test]
async fn test_delete_user() {
    let app = create_app().await;
    
    // 首先创建一个用户
    let user_data = json!({
        "name": "Test User",
        "email": "test5@example.com",
        "age": 50
    });
    
    let create_request = Request::builder()
        .method("POST")
        .uri("/api/users")
        .header("content-type", "application/json")
        .body(Body::from(user_data.to_string()))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap();
    let create_json: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
    let user_id = create_json["user"]["id"].as_str().unwrap();
    
    // 删除用户
    let delete_request = Request::builder()
        .method("DELETE")
        .uri(&format!("/api/users/{}", user_id))
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(delete_request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    
    // 验证用户已被删除
    let get_request = Request::builder()
        .method("GET")
        .uri(&format!("/api/users/{}", user_id))
        .body(Body::empty())
        .unwrap();
    
    let get_response = app.oneshot(get_request).await.unwrap();
    
    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}