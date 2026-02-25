use std::env;

fn demonstrate_map_err() -> Result<String, Box<dyn std::error::Error>> {
    println!("=== map_err 演示 ===");
    
    // 1. 原始的 env::var 返回 Result<String, VarError>
    let original_result = env::var("NON_EXISTENT_VAR");
    println!("原始结果: {:?}", original_result);
    
    // 2. 使用 map_err 转换错误类型
    let converted_result = env::var("NON_EXISTENT_VAR")
        .map_err(|e| format!("环境变量不存在: {:?}", e));
    println!("转换后结果: {:?}", converted_result);
    
    // 3. 在你的代码中的实际用法
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set")?;
    
    println!("成功获取: {}", database_url);
    Ok(database_url)
}

fn main() {
    match demonstrate_map_err() {
        Ok(msg) => println!("成功: {}", msg),
        Err(e) => println!("错误: {}", e),
    }
}