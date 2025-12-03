use axum::{
    http::Method,
    routing::get,
    Router,
};
use axum_crud::config::Config;
use axum_crud::db::{create_connection_pool, run_migrations};
use axum_crud::handlers::user::health_check;
use axum_crud::routes::user::create_user_routes;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_crud=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = Config::from_env()?;
    info!("Loaded configuration: {:?}", config);

    // 创建数据库连接池
    let pool = create_connection_pool(&config.database_url).await?;
    info!("Database connection pool created");

    // 运行数据库迁移
    run_migrations(&pool).await?;
    info!("Database migrations completed");

    // 配置CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    // 创建应用路由
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/users", create_user_routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        )
        .with_state(pool);

    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}