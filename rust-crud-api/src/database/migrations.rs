use crate::database::DatabasePool;
use crate::errors::AppError;
use sqlx::migrate::Migrator;
use std::path::Path;

/// 使用SQLx的迁移系统运行数据库迁移
/// 这是生产环境推荐的方式：
/// 1. 使用独立的SQL文件
/// 2. 版本控制和追踪
/// 3. 可回滚
/// 4. 原子性操作
pub async fn run_migrations(pool: &DatabasePool) -> Result<(), AppError> {
    // 指定迁移文件目录
    let migrator = Migrator::new(Path::new("./src/migrations"))
        .await
        .map_err(|e| {
            eprintln!("❌ 无法加载迁移文件: {}", e);
            AppError::from(e)
        })?;
    
    // 运行迁移
    migrator
        .run(pool)
        .await
        .map_err(|e| {
            eprintln!("❌ 迁移执行失败: {}", e);
            AppError::from(e)
        })?;
    
    println!("✅ 数据库迁移完成");
    Ok(())
}

/// 回滚迁移（生产环境重要功能）
pub async fn rollback_migrations(pool: &DatabasePool, steps: i64) -> Result<(), AppError> {
    let migrator = Migrator::new(Path::new("./src/migrations"))
        .await
        .map_err(|e| {
            eprintln!("❌ 无法加载迁移文件: {}", e);
            AppError::from(e)
        })?;
    
    // 回滚指定步数的迁移
    migrator
        .undo(pool, steps)
        .await
        .map_err(|e| {
            eprintln!("❌ 迁移回滚失败: {}", e);
            AppError::from(e)
        })?;
    
    println!("✅ 成功回滚 {} 步迁移", steps);
    Ok(())
}