use rust_crud_api::database::{create_pool, run_migrations, rollback_migrations};
use rust_crud_api::config::Config;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv::dotenv().ok();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("❌ 用法：");
        println!("  cargo run --bin migrate up       # 运行迁移");
        println!("  cargo run --bin migrate down <n> # 回滚n步迁移");
        return Ok(());
    }
    
    // 创建配置和数据库连接池
    let config = Config::from_env()?;
    let pool = create_pool(&config)
        .await
        .map_err(|e| format!("❌ 数据库连接失败: {}", e))?;
    
    match args[1].as_str() {
        "up" => {
            println!("🚀 开始运行数据库迁移...");
            run_migrations(&pool).await?;
        }
        "down" => {
            if args.len() < 3 {
                println!("❌ 请指定回滚步数: cargo run --bin migrate down <steps>");
                return Ok(());
            }
            
            let steps: i64 = args[2].parse()
                .map_err(|_| "❌ 无效的步数")?;
            
            println!("⏪ 开始回滚 {} 步迁移...", steps);
            rollback_migrations(&pool, steps).await?;
        }
        _ => {
            println!("❌ 未知命令: {}", args[1]);
            println!("支持的命令: up, down");
        }
    }
    
    Ok(())
}