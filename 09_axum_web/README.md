# Axum CRUD API

这是一个使用 Rust 的 Axum 框架构建的简单 CRUD (创建、读取、更新、删除) Web 应用程序，用于管理用户数据。

## 功能特性

- RESTful API 设计
- 用户 CRUD 操作
- 输入验证
- 错误处理
- 分页支持
- CORS 支持
- 日志记录
- 数据库迁移
- 单元测试和集成测试

## 技术栈

- **Web 框架**: axum
- **异步运行时**: tokio
- **数据库**: PostgreSQL
- **ORM**: SQLx
- **序列化**: serde
- **验证**: validator
- **错误处理**: thiserror
- **日志**: tracing
- **UUID**: uuid

## 项目结构

```
src/
├── main.rs              # 应用程序入口点
├── config/              # 配置模块
│   └── mod.rs
├── models/              # 数据模型
│   ├── mod.rs
│   └── user.rs
├── handlers/            # 请求处理器
│   ├── mod.rs
│   └── user.rs
├── routes/              # 路由定义
│   ├── mod.rs
│   └── user.rs
├── db/                  # 数据库相关
│   ├── mod.rs
│   ├── connection.rs    # 数据库连接
│   └── migrations/      # 数据库迁移
├── error.rs             # 错误类型定义
└── utils/               # 工具函数
    └── mod.rs
```

## 环境要求

- Rust 1.70+
- PostgreSQL 12+
- 环境变量配置文件 (.env)

## 安装与运行

### 1. 克隆项目

```bash
git clone <repository-url>
cd axum-crud
```

### 2. 设置数据库

创建 PostgreSQL 数据库：

```sql
CREATE DATABASE axum_crud;
CREATE DATABASE axum_crud_test;  -- 用于测试
```

### 3. 配置环境变量

复制并编辑 `.env` 文件：

```bash
cp .env.example .env
```

编辑 `.env` 文件，设置数据库连接信息：

```env
# Database
DATABASE_URL=postgresql://postgres:password@localhost/axum_crud

# Server
HOST=127.0.0.1
PORT=3000

# Log
RUST_LOG=info
```

### 4. 安装依赖

```bash
cargo build
```

### 5. 运行应用

```bash
cargo run
```

服务器将在 `http://127.0.0.1:3000` 上启动。

## API 端点

### 用户管理

| 方法 | 端点 | 描述 |
|------|------|------|
| POST | `/api/users` | 创建新用户 |
| GET | `/api/users` | 获取用户列表（支持分页和过滤） |
| GET | `/api/users/{id}` | 根据 ID 获取特定用户 |
| PUT | `/api/users/{id}` | 更新现有用户 |
| DELETE | `/api/users/{id}` | 删除用户 |

### 健康检查

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/health` | 应用程序健康检查 |

## API 使用示例

### 创建用户

```bash
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john@example.com",
    "age": 30
  }'
```

### 获取用户列表

```bash
curl http://localhost:3000/api/users
```

### 获取特定用户

```bash
curl http://localhost:3000/api/users/{user_id}
```

### 更新用户

```bash
curl -X PUT http://localhost:3000/api/users/{user_id} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Smith",
    "age": 35
  }'
```

### 删除用户

```bash
curl -X DELETE http://localhost:3000/api/users/{user_id}
```

### 分页和过滤

```bash
# 分页
curl "http://localhost:3000/api/users?page=1&limit=10"

# 按名称过滤
curl "http://localhost:3000/api/users?name=John"

# 按年龄范围过滤
curl "http://localhost:3000/api/users?min_age=25&max_age=40"
```

## 测试

### 运行单元测试

```bash
cargo test --lib
```

### 运行集成测试

```bash
# 设置测试数据库环境变量
export TEST_DATABASE_URL=postgresql://postgres:password@localhost/axum_crud_test

# 运行集成测试
cargo test --test api_tests
```

### 运行所有测试

```bash
cargo test
```

## 开发

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

### 数据库迁移

如果需要修改数据库结构，可以创建新的迁移文件：

```bash
# 创建新的迁移文件
# 文件名格式: {timestamp}_{description}.sql
# 例如: 20231203000002_add_new_field.sql
```

## 部署

### 使用 Docker

1. 创建 Dockerfile：

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/axum-crud /usr/local/bin/
EXPOSE 3000
CMD ["axum-crud"]
```

2. 构建镜像：

```bash
docker build -t axum-crud .
```

3. 运行容器：

```bash
docker run -p 3000:3000 --env-file .env axum-crud
```

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 联系方式

如有问题或建议，请通过 Issue 联系我们。