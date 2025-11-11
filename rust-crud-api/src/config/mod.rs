use std::env;

/// 应用程序配置
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: String,
    pub jwt_secret: Option<String>,
    pub redis_url: String,
    pub cache_ttl_seconds: u64,
    // 区块链监听配置
    pub arbitrum_ws_url: Option<String>,
    pub arbitrum_http_url: Option<String>,
    pub vault_contract_address: Option<String>,
    pub usdc_token_address: Option<String>,
    pub enable_vault_watcher: bool,
    pub watcher_confirmations: u64,
}

impl Config {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string()),
            jwt_secret: env::var("JWT_SECRET").ok(),
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            cache_ttl_seconds: env::var("CACHE_TTL_SECONDS")
                .unwrap_or_else(|_| "120".to_string()) // 默认2分钟
                .parse()
                .unwrap_or(120),
            arbitrum_ws_url: env::var("ARBITRUM_WS_URL").ok(),
            arbitrum_http_url: env::var("ARBITRUM_HTTP_URL").ok(),
            vault_contract_address: env::var("VAULT_CONTRACT_ADDRESS").ok(),
            usdc_token_address: env::var("USDC_TOKEN_ADDRESS").ok(),
            enable_vault_watcher: env::var("ENABLE_VAULT_WATCHER").map(|v| v == "true").unwrap_or(true),
            watcher_confirmations: env::var("WATCHER_CONFIRMATIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .unwrap_or(2),
        })
    }

    /// 获取服务器绑定地址
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}