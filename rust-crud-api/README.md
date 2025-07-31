# Rust CRUD API 项目

一个使用 Rust 开发的用户管理 Web API，支持完整的 CRUD（增删改查）功能。

## 🛠 技术栈

- **Web 框架**: Actix-web 4.4
- **数据库操作**: SQLx 0.7 (支持 PostgreSQL)
- **异步运行时**: Tokio
- **序列化**: Serde
- **密码加密**: bcrypt
- **日志**: env_logger

## 📋 功能特性

- ✅ 用户注册（带密码加密）
- ✅ 用户信息查询（按 ID 或用户名）
- ✅ 用户信息修改
- ✅ 用户删除
- ✅ 获取所有用户列表
- ✅ 健康检查端点
- ✅ 完整的错误处理和响应格式

## 🚀 快速开始

### 1. 环境准备

确保你的系统已安装：
- Rust (最新稳定版)
- PostgreSQL (本地运行)
- jq (用于测试脚本，可选)

### 2. 数据库设置

```bash
# 连接到 PostgreSQL
psql -U postgres

# 创建数据库
CREATE DATABASE rust_crud_db;

# 创建用户 (可选，或使用现有用户)
CREATE USER your_username WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE rust_crud_db TO your_username;
```

### 3. 配置环境变量

复制环境变量模板：
```bash
cp .env.example .env
```

编辑 `.env` 文件，设置你的数据库连接信息：
```env
DATABASE_URL=postgresql://your_username:your_password@localhost:5432/rust_crud_db
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

### 4. 构建和运行

```bash
# 构建项目
cargo build

# 运行项目
cargo run
```

服务器将在 `http://127.0.0.1:8080` 启动。

## 📚 API 文档

### 基础 URL
```
http://127.0.0.1:8080
```

### 端点列表

| 方法 | 路径 | 描述 |
|------|------|------|
| POST | `/api/users` | 用户注册 |
| GET | `/api/users` | 获取所有用户 |
| GET | `/api/users/{id}` | 根据 ID 获取用户 |
| GET | `/api/users/username/{username}` | 根据用户名获取用户 |
| PUT | `/api/users/{id}` | 更新用户信息 |
| DELETE | `/api/users/{id}` | 删除用户 |
| GET | `/health` | 健康检查 |

### 请求示例

#### 1. 用户注册
```bash
curl -X POST http://127.0.0.1:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "password123",
    "full_name": "John Doe"
  }'
```

#### 2. 获取用户信息
```bash
curl -X GET http://127.0.0.1:8080/api/users/{user_id}
```

#### 3. 更新用户信息
```bash
curl -X PUT http://127.0.0.1:8080/api/users/{user_id} \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newemail@example.com",
    "full_name": "New Name"
  }'
```

#### 4. 删除用户
```bash
curl -X DELETE http://127.0.0.1:8080/api/users/{user_id}
```

### 响应格式

所有 API 响应都遵循统一格式：

```json
{
  "success": true,
  "message": "操作成功",
  "data": {
    "id": "uuid",
    "username": "johndoe",
    "email": "john@example.com",
    "full_name": "John Doe",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

错误响应：
```json
{
  "success": false,
  "message": "错误描述",
  "data": null
}
```

## 🧪 测试

项目包含一个自动化测试脚本：

```bash
# 确保服务器正在运行，然后执行
./test_api.sh
```

这个脚本会测试所有的 CRUD 操作。

## 📁 项目结构

```
rust-crud-api/
├── Cargo.toml                    # 项目配置和依赖
├── README.md                     # 项目说明文档
├── ARCHITECTURE.md               # 架构文档
├── .env.example                  # 环境变量模板
├── test_api.sh                   # API 测试脚本
├── migrations/                   # 数据库迁移文件
│   └── 001_create_users_table.sql
├── src/                          # 源代码
│   ├── lib.rs                   # 库根模块，定义公共接口
│   ├── main.rs                  # 应用程序入口点
│   ├── config/                  # 配置管理
│   │   └── mod.rs              # 配置结构和环境变量处理
│   ├── database/                # 数据库层
│   │   ├── mod.rs              # 数据库模块根
│   │   ├── connection.rs       # 数据库连接池管理
│   │   └── migrations.rs       # 数据库迁移处理
│   ├── models/                  # 数据模型
│   │   ├── mod.rs              # 模型模块根
│   │   ├── user.rs             # 用户相关数据结构
│   │   └── response.rs         # API 响应结构
│   ├── services/                # 业务逻辑层
│   │   ├── mod.rs              # 服务模块根
│   │   └── user.rs             # 用户业务逻辑
│   ├── handlers/                # HTTP 请求处理器
│   │   ├── mod.rs              # 处理器模块根
│   │   ├── user.rs             # 用户相关 API 处理器
│   │   └── health.rs           # 健康检查处理器
│   ├── routes/                  # 路由配置
│   │   └── mod.rs              # 路由定义和配置
│   ├── middleware/              # 中间件
│   │   ├── mod.rs              # 中间件模块根
│   │   └── logging.rs          # 请求日志中间件
│   ├── utils/                   # 工具函数
│   │   ├── mod.rs              # 工具模块根
│   │   ├── validation.rs       # 数据验证工具
│   │   └── password.rs         # 密码处理工具
│   └── errors/                  # 错误处理
│       └── mod.rs              # 统一错误处理和响应
└── tests/                       # 测试文件
    ├── integration_tests.rs     # 集成测试
    └── common/                  # 测试辅助模块
        └── mod.rs              # 共享测试工具
```

## 🏗 架构设计

项目采用模块化的分层架构：

- **config/**: 应用程序配置管理
- **database/**: 数据库连接和迁移管理
- **models/**: 数据模型和验证逻辑
- **services/**: 核心业务逻辑
- **handlers/**: HTTP 请求处理
- **routes/**: 路由配置和组织
- **middleware/**: 中间件（日志、认证等）
- **utils/**: 通用工具函数
- **errors/**: 统一错误处理

详细架构说明请查看 [ARCHITECTURE.md](./ARCHITECTURE.md)

## 🔧 开发说明

### 数据库表结构

用户表字段：
- `id`: UUID 主键
- `username`: 用户名（唯一）
- `email`: 邮箱（唯一）
- `password_hash`: 加密后的密码
- `full_name`: 全名
- `created_at`: 创建时间
- `updated_at`: 更新时间（自动更新）

### 安全特性

- 密码使用 bcrypt 加密存储
- API 响应中不包含密码信息
- 输入验证和错误处理
- 数据库约束确保数据完整性

## 🧪 测试

项目包含多层测试：

### 单元测试
```bash
cargo test
```

### 集成测试
```bash
cargo test --test integration_tests
```

### API 测试
```bash
# 确保服务器正在运行
./test_api.sh
```

## 🚧 后续扩展

可以考虑添加的功能：
- JWT 身份认证
- API 限流和频次控制
- 输入验证中间件
- 缓存层（Redis）
- Docker 容器化
- API 文档生成 (Swagger/OpenAPI)
- 监控和指标收集
- 日志聚合和分析

## 📄 许可证

MIT License