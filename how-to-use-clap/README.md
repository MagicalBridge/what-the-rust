# how-to-use-clap

学习 Rust clap 命令行参数解析的 demo 项目，演示如何在实际项目中使用 clap derive 模式配合 YAML 配置文件。

## 项目概述

本项目模拟一个典型的 Rust 命令行应用结构：通过 clap 解析命令行参数，从 YAML 加载配置，初始化日志，启动各功能模块。适合学习 clap 在真实项目中的用法。

## 项目结构

```
how-to-use-clap/
├── Cargo.toml              # 依赖配置
├── config/
│   └── app-config.yml      # YAML 配置文件
├── src/
│   ├── main.rs             # 入口：Args 定义 + 参数解析
│   ├── config.rs           # AppConfig + load_from_yaml
│   └── app.rs              # StartOptions + run_app
└── README.md
```

## clap 学习要点

### 1. derive 模式基本用法
```rust
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = String::from("info"))]
    log_level: String,
}

let args = Args::parse();  // 自动从 std::env::args() 解析
```

### 2. `#[arg(...)]` 常用属性
- `short`：生成短参数（如 `-l`），默认取字段名首字母
- `short = 't'`：手动指定短参数字母
- `long`：生成长参数（如 `--log-level`），自动将 `_` 转为 `-`
- `default_value_t = value`：设置默认值

### 3. `#[command(...)]` 常用属性
- `name`：程序名称
- `about`：程序描述（出现在 `--help` 中）

### 4. 文档注释 `///` 的双重作用
```rust
/// 日志级别        <-- 这行会同时作为 --help 中该参数的说明
#[arg(short, long)]
log_level: String,
```

## 快速开始

```bash
cd how-to-use-clap

# 默认运行（使用 config/app-config.yml）
cargo run

# 指定参数运行
cargo run -- --log-level debug --config-path config/app-config -t 1700000000000

# 查看帮助信息（理解 clap 自动生成了什么）
cargo run -- --help

# 只编译
cargo build
```

## 运行效果示例

```
$ cargo run -- --help
学习 clap 用法的 demo

Usage: how-to-use-clap [OPTIONS]

Options:
  -l, --log-level <LOG_LEVEL>              日志级别 [default: info]
  -c, --config-path <CONFIG_PATH>          配置文件路径 [default: config/app-config]
  -t, --init-timestamp-ms <INIT_TIMESTAMP_MS>  初始时间戳（毫秒） [default: 0]
  -h, --help                               Print help
```

## 执行流程

```
Args::parse()
    │  clap 从 std::env::args() 解析命令行参数
    ▼
AppConfig::load_from_yaml(&args.config_path)
    │  从 YAML 文件加载应用配置
    ▼
run_app(&app_config, args.init_timestamp_ms)
    │  根据配置启动各功能模块
    ▼
等待 Ctrl+C 退出信号
```
