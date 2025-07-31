use actix_web::{HttpResponse, Result};
use crate::models::ApiResponse;

/// 健康检查端点
pub async fn health_check() -> Result<HttpResponse> {
    let response = ApiResponse::success(
        "OK",
        "服务运行正常"
    );
    Ok(HttpResponse::Ok().json(response))
}