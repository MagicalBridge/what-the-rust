use serde::Serialize;

/// API 响应结构
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }

    /// 打印响应体内容到控制台（用于调试）
    pub fn print_response_body(&self) {
        println!("📋 响应体内容:");
        match serde_json::to_string_pretty(self) {
            Ok(json_str) => {
                println!("   {}", json_str.replace('\n', "\n   "));
            }
            Err(e) => {
                println!("   ❌ 无法序列化响应: {}", e);
            }
        }
    }

    /// 创建成功响应并打印内容
    pub fn success_with_print(data: T, message: &str) -> Self {
        let response = Self::success(data, message);
        response.print_response_body();
        response
    }
}
