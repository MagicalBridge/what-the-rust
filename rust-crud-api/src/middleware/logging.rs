use actix_web::middleware::Logger;

/// 请求日志中间件
pub struct RequestLogging;

impl RequestLogging {
    /// 创建详细的日志中间件
    pub fn new() -> Logger {
        Logger::new(r#"🌐 %{r}a "%r" %s %b "%{Content-Type}i" %T ms"#)
            .log_target("actix_web::middleware::logger")
    }

    /// 创建简洁的日志中间件
    pub fn simple() -> Logger {
        Logger::new(r#"📝 %a %r -> %s (%b bytes) [%T s]"#)
    }

    /// 创建开发模式的详细日志
    pub fn dev() -> Logger {
        Logger::new(r#"🚀 %{r}a "%r" %s %b "%{User-Agent}i" %T s"#)
    }
}
