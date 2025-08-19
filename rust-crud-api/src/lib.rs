pub mod config; // 配置模块
pub mod database; // 数据库模块
pub mod models; // 数据库模型模块
pub mod services; // 业务逻辑服务模块
pub mod handlers; // http请求处理器模块
pub mod routes; // 路由模块
pub mod middleware; // 中间件模块
pub mod utils; // 工具函数模块
pub mod errors; // 错误处理模块

pub use config::Config; // 导出配置模块
pub use database::DatabasePool; // 导出数据库连接池模块
pub use errors::AppError; // 导出错误处理模块