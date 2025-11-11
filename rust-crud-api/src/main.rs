use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use rust_crud_api::{Config, database, services, routes, middleware};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv().ok();
    
    // 设置日志级别（如果未设置）
    if env::var("RUST_LOG").is_err() {
        unsafe {
            env::set_var("RUST_LOG", "info,actix_web=info,rust_crud_api=debug");
        }
    }
    
    // 初始化日志
    env_logger::init();

    // 加载配置
    let config = Config::from_env()
        .expect("Failed to load configuration");

    // 创建数据库连接池
    let pool = database::create_pool(&config).await
        .expect("Failed to create database pool");
    // 供监听器使用的克隆
    let pool_for_watcher = pool.clone();

    // 创建Redis连接池
    let redis_pool = database::create_redis_pool(&config).await
        .expect("Failed to create Redis pool");

    // 测试Redis连接
    database::test_redis_connection(&redis_pool).await
        .expect("Failed to connect to Redis");
    
    // 创建缓存服务
    let cache_service = services::cache::CacheService::new(redis_pool, config.cache_ttl_seconds);

    // 创建用户服务
    let user_service = services::UserService::new(pool, cache_service);

    // 启动区块链监听（后台任务）
    {
        let config_clone = config.clone();
        tokio::spawn(async move {
            if let Err(e) = rust_crud_api::listeners::arbitrum_vault::start_vault_watcher(config_clone, pool_for_watcher).await {
                log::error!("❌ Vault 监听器启动失败: {}", e);
            }
        });
    }

    println!("🚀 服务器启动在 http://{}", config.bind_address());
    println!("💾 Redis缓存已启用，TTL: {}秒", config.cache_ttl_seconds);
    println!("📚 API 文档:");
    println!("  POST   /api/users          - 用户注册");
    println!("  GET    /api/users          - 获取所有用户 (缓存支持)");
    println!("  GET    /api/users/{{id}}     - 根据 ID 获取用户 (缓存支持)");
    println!("  GET    /api/users/username/{{username}} - 根据用户名获取用户 (缓存支持)");
    println!("  PUT    /api/users/{{id}}     - 更新用户信息");
    println!("  DELETE /api/users/{{id}}     - 删除用户");
    println!("  GET    /health             - 健康检查");

    // 启动 HTTP 服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .wrap(middleware::RequestLogging::dev())
            .wrap(middleware::ResponsePrinter)  // 添加响应打印中间件
            .service(routes::user_routes())
            .configure(routes::health_routes())
    })
    .bind(config.bind_address())?
    .run()
    .await
}
