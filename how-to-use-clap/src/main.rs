mod app;
mod config;

use anyhow::Result;
use clap::Parser;
use tracing::{error, info};

// ======================================================================
// clap 核心用法：通过 #[derive(Parser)] 自动生成命令行参数解析
// ======================================================================

// `#[derive(Parser)]` 做了什么？
// - 为 Args 实现 clap::Parser trait
// - 自动生成 Args::parse() 方法，该方法会：
//   1. 读取 std::env::args()
//   2. 根据字段名和 #[arg(...)] 属性解析参数
//   3. 返回填充好的 Args 实例
#[derive(Parser, Debug)]
#[command(name = "how-to-use-clap")]
#[command(about = "学习 clap 用法的 demo")]
struct Args {
    /// 日志级别
    // short: 生成 -l 短参数
    // long: 生成 --log-level 长参数（自动将下划线转为连字符）
    // default_value_t: 未指定时的默认值
    #[arg(short, long, default_value_t = String::from("info"))]
    log_level: String,

    /// 配置文件路径
    // short: 自动取首字母 -c
    // long: 自动生成 --config-path
    #[arg(short, long, default_value_t = String::from("config/app-config"))]
    config_path: String,

    /// 初始时间戳（毫秒）
    // short = 't': 手动指定短参数为 -t（避免与其他参数首字母冲突）
    // long: 自动生成 --init-timestamp-ms
    #[arg(short = 't', long, default_value_t = 0)]
    init_timestamp_ms: u64,
}

// ======================================================================
// 主函数流程：
//   1. Args::parse()              — 解析命令行参数
//   2. AppConfig::load_from_yaml() — 从 YAML 加载配置
//   3. init tracing               — 初始化日志
//   4. run_app()                  — 运行应用
// ======================================================================

#[tokio::main]
async fn main() -> Result<()> {
    // 第 1 步：解析命令行参数
    // Args::parse() 是 clap derive 自动生成的关联函数
    // 内部调用 std::env::args() 获取进程参数，然后匹配各个字段
    let args = Args::parse();

    // 第 2 步：初始化日志
    //
    // args.log_level 是 String（如 "info"、"debug"）
    // .parse() 即 str::parse::<T>()，Rust 根据上下文推断 T = tracing::Level
    // tracing::Level 实现了 FromStr trait，所以能把字符串解析为枚举值
    // .unwrap_or(...) — parse() 返回 Result，解析失败时回退到 INFO，不会 panic
    let log_level = args.log_level.parse().unwrap_or(tracing::Level::INFO);

    // 下面是 Builder 模式（建造者模式）的链式调用：
    // 1. fmt()          — 创建格式化日志订阅者的 Builder，日志以文本格式输出到终端
    // 2. with_max_level — 设置最大日志级别过滤器，低于该级别的日志被丢弃
    //                     例如设为 INFO，则 DEBUG/TRACE 会被过滤掉
    // 3. init()         — 消费 Builder，构建 Subscriber 并注册为全局默认
    //                     之后所有 info!()/error!()/debug!() 宏都经过这个订阅者处理
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();

    info!("命令行参数解析结果: {:?}", args);

    // 第 3 步：从 YAML 加载配置
    let app_config = config::AppConfig::load_from_yaml(&args.config_path)?;
    info!("配置文件加载成功: {}", app_config.name);

    // 第 4 步：运行应用
    if let Err(e) = app::run_app(&app_config, args.init_timestamp_ms).await {
        error!("应用运行错误: {}", e);
        return Err(e);
    }

    Ok(())
}
