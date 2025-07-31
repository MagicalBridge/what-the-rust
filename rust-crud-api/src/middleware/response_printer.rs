use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, Result, body::MessageBody,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

/// 响应打印中间件
pub struct ResponsePrinter;

impl<S, B> Transform<S, ServiceRequest> for ResponsePrinter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ResponsePrinterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ResponsePrinterMiddleware { service: Rc::new(service) }))
    }
}

pub struct ResponsePrinterMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ResponsePrinterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        
        Box::pin(async move {
            // 提取请求信息
            let method = req.method().to_string();
            let path = req.path().to_string();
            
            // 调用下一个服务
            let res = service.call(req).await?;
            
            // 获取响应状态码和头信息
            let status = res.status();
            let headers = res.headers();
            
            // 打印响应信息到终端
            println!("\n🔍 API响应调试信息");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📍 接口: {} {}", method, path);
            println!("📊 状态码: {} {}", status.as_u16(), status.canonical_reason().unwrap_or("Unknown"));
            
            // 根据状态码显示不同的状态图标
            let status_icon = match status.as_u16() {
                200..=299 => "✅",
                400..=499 => "⚠️", 
                500..=599 => "❌",
                _ => "ℹ️",
            };
            
            println!("{} 响应状态: {}", status_icon, 
                if status.is_success() { "成功" }
                else if status.is_client_error() { "客户端错误" }
                else if status.is_server_error() { "服务器错误" }
                else { "其他" }
            );
            
            // 打印响应头信息（部分重要的）
            if let Some(content_type) = headers.get("content-type") {
                if let Ok(content_type_str) = content_type.to_str() {
                    println!("📄 内容类型: {}", content_type_str);
                }
            }
            
            // 打印内容长度
            if let Some(content_length) = headers.get("content-length") {
                if let Ok(length_str) = content_length.to_str() {
                    println!("📏 内容长度: {} 字节", length_str);
                }
            }
            
            println!("💡 提示: 响应体内容已在处理程序中打印（如果启用）");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            
            Ok(res)
        })
    }
}