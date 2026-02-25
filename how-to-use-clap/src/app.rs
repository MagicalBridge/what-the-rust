use anyhow::Result;
use tracing::info;

use crate::config::AppConfig;

/// 控制哪些功能模块需要启动
#[derive(Clone, Debug, Default)]
pub struct StartOptions {
    pub is_test: bool,
    pub enable_sync: bool,
    pub enable_api: bool,
    pub enable_monitor: bool,
}

impl StartOptions {
    pub fn all() -> Self {
        Self {
            is_test: false,
            enable_sync: true,
            enable_api: true,
            enable_monitor: true,
        }
    }

    pub fn test_all() -> Self {
        Self {
            is_test: true,
            enable_sync: true,
            enable_api: true,
            enable_monitor: true,
        }
    }
}

/// 根据配置启动各功能模块
pub async fn run_app(config: &AppConfig, init_timestamp_ms: u64) -> Result<()> {
    let options = StartOptions {
        is_test: false,
        enable_sync: config.enable_sync,
        enable_api: config.enable_api,
        enable_monitor: config.enable_monitor,
    };

    info!("========================================");
    info!("  应用名称: {}", config.name);
    info!("  网络: {}", config.network);
    info!("  API 版本: {}", config.api_version);
    info!("  数据库路径: {}", config.db_path);
    info!("  配置目录: {}", config.config_dir);
    info!("  初始时间戳: {} ms", init_timestamp_ms);
    info!("========================================");

    info!("启动选项: {:?}", options);

    if options.enable_sync {
        info!("[Sync] 链数据同步模块已启动");
        for chain in &config.chains {
            info!(
                "  -> 监听链: {} (chain_id={}, rpc={}, confirmations={})",
                chain.name, chain.chain_id, chain.rpc_url, chain.confirmations
            );
        }
    }

    if options.enable_api {
        info!("[API] API 服务模块已启动");
    }

    if options.enable_monitor {
        info!("[Monitor] 监控模块已启动");
        if let Some(mq) = &config.mq_config {
            info!("  -> 消息队列: broker={}, topic={}", mq.broker, mq.topic);
        }
    }

    info!("所有模块启动完毕，应用运行中...");
    info!("(按 Ctrl+C 退出)");

    tokio::signal::ctrl_c().await?;
    info!("收到退出信号，应用关闭");

    Ok(())
}
