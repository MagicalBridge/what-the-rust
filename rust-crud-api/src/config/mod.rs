use std::env;

/// 应用程序配置
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: String,
    pub jwt_secret: Option<String>,
}

impl Config {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string()),
            jwt_secret: env::var("JWT_SECRET").ok(),
        })
    }

    /// 获取服务器绑定地址
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}