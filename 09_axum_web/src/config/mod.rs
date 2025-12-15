use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // 加载 .env 文件中的环境变量，如果文件不存在也不报错
        dotenvy::dotenv().ok();

        // 从环境变量获取数据库连接地址，如果不存在则返回错误
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set")?;
        
        // 从环境变量获取服务器主机地址，如果不存在则使用默认值 127.0.0.1
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        
        // 从环境变量获取服务器端口，如果不存在则使用默认值 3000
        // 然后将字符串转换为数字，如果转换失败则返回错误
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| "PORT must be a number")?;

        // 创建并返回 Config 实例，包含所有配置项
        Ok(Config {
            database_url,
            host,
            port,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}