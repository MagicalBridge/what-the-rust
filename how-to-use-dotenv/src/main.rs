use std::env;  // 导入标准库中的 env 模块

fn main() {
    // 加载 .env 文件
    // dotenv::dotenv() 会在当前目录及其父目录中查找 .env 文件
    match dotenv::dotenv() {
        Ok(_) => println!("✅ 成功加载 .env 文件"),
        Err(e) => println!("⚠️  无法加载 .env 文件: {}", e),
    }

    println!("{}", "=".repeat(60));
    println!("🦀 Rust dotenv 使用示例");
    println!("{}", "=".repeat(60));

    // 1. 基本的环境变量读取
    println!("\n📋 1. 基本环境变量读取:");
    
    // 使用 env::var() 读取环境变量，返回 Result<String, VarError>
    match env::var("APP_NAME") {
        Ok(app_name) => println!("   应用名称: {}", app_name),
        Err(_) => println!("   ❌ 未找到 APP_NAME 环境变量"),
    }

    // 使用 unwrap_or() 提供默认值
    let app_version = env::var("APP_VERSION").unwrap_or("未知版本".to_string());
    println!("   应用版本: {}", app_version);

    // 使用 unwrap_or_else() 提供默认值（延迟计算）
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "production".to_string());
    println!("   运行环境: {}", app_env);

    // 2. 数据库配置示例
    println!("\n🗄️  2. 数据库配置:");
    show_database_config();

    // 3. 数值类型的环境变量
    println!("\n🔢 3. 数值类型环境变量:");
    show_numeric_config();

    // 4. 布尔类型的环境变量
    println!("\n✅ 4. 布尔类型环境变量:");
    show_boolean_config();

    // 5. 可选配置项（带默认值）
    println!("\n⚙️  5. 带默认值的可选配置:");
    show_optional_config();

    // 6. 敏感信息处理
    println!("\n🔐 6. 敏感信息处理:");
    show_sensitive_config();

    // 7. 所有环境变量列表
    println!("\n📜 7. 所有环境变量 (仅显示非敏感信息):");
    show_all_env_vars();

    println!("\n{}", "=".repeat(60));
    println!("✨ dotenv 示例完成！");
    println!("{}", "=".repeat(60));
}

fn show_database_config() {
    // 方式1: 构建完整的数据库URL
    if let Ok(db_url) = env::var("DATABASE_URL") {
        println!("   数据库URL: {}", db_url);
    }

    // 方式2: 分别读取各个组件
    let host = env::var("DATABASE_HOST").unwrap_or("localhost".to_string());
    let port = env::var("DATABASE_PORT").unwrap_or("5432".to_string());
    let name = env::var("DATABASE_NAME").unwrap_or("default".to_string());
    let user = env::var("DATABASE_USER").unwrap_or("root".to_string());
    
    println!("   数据库主机: {}", host);
    println!("   数据库端口: {}", port);
    println!("   数据库名称: {}", name);
    println!("   数据库用户: {}", user);
}

fn show_numeric_config() {
    // 端口号（整数）
    let port: u16 = env::var("APP_PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .unwrap_or(3000);
    println!("   应用端口: {}", port);

    // API超时（整数）
    let api_timeout: u32 = env::var("API_TIMEOUT")
        .unwrap_or("30".to_string())
        .parse()
        .unwrap_or(30);
    println!("   API超时: {}秒", api_timeout);

    // 最大连接数
    let max_conn: usize = env::var("MAX_CONNECTIONS")
        .unwrap_or("50".to_string())
        .parse()
        .unwrap_or(50);
    println!("   最大连接数: {}", max_conn);
}

fn show_boolean_config() {
    // 调试模式
    let debug = env::var("DEBUG")
        .unwrap_or("false".to_string())
        .to_lowercase() == "true";
    println!("   调试模式: {}", if debug { "开启" } else { "关闭" });

    // 更完善的布尔值解析
    let is_production = parse_bool_env("APP_ENV", |env| env == "production");
    println!("   生产环境: {}", if is_production { "是" } else { "否" });
}

fn show_optional_config() {
    // 缓存TTL（有默认值）
    let cache_ttl: u64 = env::var("CACHE_TTL")
        .unwrap_or("1800".to_string())  // 默认30分钟
        .parse()
        .unwrap_or(1800);
    println!("   缓存TTL: {}秒", cache_ttl);

    // 日志级别
    let log_level = env::var("LOG_LEVEL").unwrap_or("warn".to_string());
    println!("   日志级别: {}", log_level);

    // 日志文件
    if let Ok(log_file) = env::var("LOG_FILE") {
        println!("   日志文件: {}", log_file);
    } else {
        println!("   日志文件: 未配置（将输出到控制台）");
    }
}

fn show_sensitive_config() {
    // 对于敏感信息，我们不直接显示值，而是显示是否已配置
    let has_api_key = env::var("API_KEY").is_ok();
    println!("   API密钥: {}", if has_api_key { "已配置 ✓" } else { "未配置 ✗" });

    let has_jwt_secret = env::var("JWT_SECRET").is_ok();
    println!("   JWT密钥: {}", if has_jwt_secret { "已配置 ✓" } else { "未配置 ✗" });

    let has_encryption_key = env::var("ENCRYPTION_KEY").is_ok();
    println!("   加密密钥: {}", if has_encryption_key { "已配置 ✓" } else { "未配置 ✗" });

    // 如果需要显示部分内容（用于调试），可以这样做
    if let Ok(api_key) = env::var("API_KEY") {
        let masked = mask_sensitive_value(&api_key);
        println!("   API密钥（部分）: {}", masked);
    }
}

fn show_all_env_vars() {
    // 获取所有以特定前缀开头的环境变量
    let prefixes = ["APP_", "DATABASE_", "LOG_"];
    
    for prefix in prefixes.iter() {
        println!("   {}开头的变量:", prefix);
        for (key, value) in env::vars() {
            if key.starts_with(prefix) && !is_sensitive(&key) {
                println!("     {}: {}", key, value);
            }
        }
    }
}

// 辅助函数：解析布尔环境变量
fn parse_bool_env<F>(key: &str, condition: F) -> bool 
where 
    F: Fn(&str) -> bool,
{
    env::var(key)
        .map(|val| condition(&val))
        .unwrap_or(false)
}

// 辅助函数：遮盖敏感信息
fn mask_sensitive_value(value: &str) -> String {
    if value.len() <= 8 {
        "*".repeat(value.len())
    } else {
        format!("{}***{}", &value[..4], &value[value.len()-4..])
    }
}

// 辅助函数：判断是否为敏感信息
fn is_sensitive(key: &str) -> bool {
    let sensitive_keywords = ["PASSWORD", "SECRET", "KEY", "TOKEN"];
    sensitive_keywords.iter().any(|&keyword| key.contains(keyword))
}
