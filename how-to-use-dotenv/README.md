# 🦀 Rust dotenv 使用示例

这个项目演示了如何在 Rust 中使用 `dotenv` 包来管理环境变量和配置。

## 📋 项目概述

`dotenv` 是一个非常实用的 Rust 包，它可以从 `.env` 文件中加载环境变量到你的程序中。这在配置管理、不同环境部署（开发、测试、生产）中非常有用。

## 🚀 快速开始

### 1. 运行项目

```bash
# 进入项目目录
cd how-to-use-dotenv

# 运行项目
cargo run
```

### 2. 查看输出

程序会显示各种环境变量的使用方法，包括：
- 基本环境变量读取
- 数据库配置管理
- 数值类型处理
- 布尔类型处理
- 可选配置项
- 敏感信息处理
- 环境变量列表

## 📁 项目结构

```
how-to-use-dotenv/
├── Cargo.toml          # 项目配置和依赖
├── .env                # 环境变量配置文件
├── README.md           # 项目说明文档
└── src/
    └── main.rs         # 主程序文件
```

## 🔧 核心功能演示

### 1. 基本环境变量读取

```rust
// 读取环境变量，处理可能的错误
match env::var("APP_NAME") {
    Ok(app_name) => println!("应用名称: {}", app_name),
    Err(_) => println!("未找到 APP_NAME 环境变量"),
}

// 使用默认值
let app_version = env::var("APP_VERSION").unwrap_or("未知版本".to_string());
```

### 2. 数值类型处理

```rust
// 将字符串转换为数值类型
let port: u16 = env::var("APP_PORT")
    .unwrap_or("3000".to_string())
    .parse()
    .unwrap_or(3000);
```

### 3. 布尔类型处理

```rust
// 解析布尔值
let debug = env::var("DEBUG")
    .unwrap_or("false".to_string())
    .to_lowercase() == "true";
```

### 4. 敏感信息处理

```rust
// 检查敏感配置是否存在，而不显示具体值
let has_api_key = env::var("API_KEY").is_ok();
println!("API密钥: {}", if has_api_key { "已配置 ✓" } else { "未配置 ✗" });
```

## 📝 配置文件说明

`.env` 文件包含以下类型的配置：

- **数据库配置**: `DATABASE_URL`, `DATABASE_HOST` 等
- **应用配置**: `APP_NAME`, `APP_VERSION`, `APP_ENV` 等
- **API配置**: `API_KEY`, `API_TIMEOUT` 等
- **日志配置**: `LOG_LEVEL`, `LOG_FILE` 等
- **可选配置**: `MAX_CONNECTIONS`, `CACHE_TTL` 等
- **密钥配置**: `JWT_SECRET`, `ENCRYPTION_KEY` 等

## 🛡️ 最佳实践

### 1. 环境变量命名
- 使用大写字母和下划线
- 使用有意义的前缀（如 `DATABASE_`, `API_`）
- 保持命名一致性

### 2. 默认值处理
```rust
// 推荐：提供合理的默认值
let timeout = env::var("API_TIMEOUT")
    .unwrap_or("30".to_string())
    .parse()
    .unwrap_or(30);
```

### 3. 敏感信息保护
- 不要在日志中输出敏感信息
- 在生产环境中使用强密码
- 考虑使用密钥管理服务

### 4. 环境区分
```rust
let is_production = env::var("APP_ENV")
    .unwrap_or("development".to_string()) == "production";
```

## 📦 依赖说明

### dotenv = "0.15"

这是项目的核心依赖，提供了以下主要功能：

- `dotenv::dotenv()`: 加载 `.env` 文件
- 自动搜索当前目录及父目录中的 `.env` 文件
- 与 `std::env` 完美集成

## 🔍 高级用法

### 1. 条件加载
```rust
// 只在开发环境加载 .env 文件
#[cfg(debug_assertions)]
dotenv::dotenv().ok();
```

### 2. 自定义 .env 文件路径
```rust
// 从指定路径加载
dotenv::from_filename("custom.env").ok();
```

### 3. 环境变量验证
```rust
// 验证必需的环境变量
let required_vars = ["DATABASE_URL", "JWT_SECRET"];
for var in required_vars.iter() {
    env::var(var).expect(&format!("环境变量 {} 是必需的", var));
}
```

## 🚨 注意事项

1. **安全性**: 不要将 `.env` 文件提交到版本控制系统
2. **部署**: 在生产环境中使用环境变量而不是 `.env` 文件
3. **备份**: 提供 `.env.example` 文件作为模板
4. **文档**: 为每个环境变量编写清晰的说明

## 📚 参考资源

- [dotenv 官方文档](https://docs.rs/dotenv/)
- [Rust 环境变量文档](https://doc.rust-lang.org/std/env/)
- [配置管理最佳实践](https://12factor.net/config)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来改进这个示例项目！

---

*这个项目是 Rust 学习系列的一部分，专注于实用的工具和库的使用方法。*
