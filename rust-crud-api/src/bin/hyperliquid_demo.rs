use rust_crud_api::examples::run_hyperliquid_example;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    env_logger::init();
    
    println!("🔥 Hyperliquid Rust SDK 演示程序");
    println!("================================");
    
    // 运行示例
    run_hyperliquid_example().await?;
    
    Ok(())
}