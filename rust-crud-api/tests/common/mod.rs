/// 测试通用模块
/// 包含测试辅助函数和共享测试数据

use rust_crud_api::models::CreateUserRequest;

/// 创建测试用户请求
pub fn create_test_user_request(username: &str) -> CreateUserRequest {
    CreateUserRequest {
        username: username.to_string(),
        email: format!("{}@example.com", username),
        password: "password123".to_string(),
        full_name: format!("Test User {}", username),
    }
}