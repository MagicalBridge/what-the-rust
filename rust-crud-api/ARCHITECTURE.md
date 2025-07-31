# 项目架构文档

## 📁 项目结构

```
rust-crud-api/
├── Cargo.toml                    # 项目配置和依赖
├── README.md                     # 项目说明文档
├── ARCHITECTURE.md               # 架构文档（本文件）
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

### 分层架构

项目采用经典的分层架构模式：

```
┌─────────────────┐
│   HTTP Layer    │  ← handlers/ (HTTP 请求处理)
├─────────────────┤
│  Business Layer │  ← services/ (业务逻辑)
├─────────────────┤
│   Data Layer    │  ← database/ (数据访问)
└─────────────────┘
```

### 模块职责

#### 🔧 Config Layer (`src/config/`)
- **职责**: 管理应用程序配置
- **包含**: 环境变量处理、配置验证
- **特点**: 集中式配置管理，支持不同环境

#### 🗄 Database Layer (`src/database/`)
- **职责**: 数据库连接和管理
- **包含**: 连接池、迁移管理
- **特点**: 类型安全的 SQL 操作（SQLx）

#### 📊 Models Layer (`src/models/`)
- **职责**: 定义数据结构和验证规则
- **包含**: 实体模型、请求/响应 DTO
- **特点**: 序列化/反序列化支持，数据验证

#### 🏢 Services Layer (`src/services/`)
- **职责**: 核心业务逻辑
- **包含**: 数据操作、业务规则、事务处理
- **特点**: 独立于 HTTP，可复用的业务逻辑

#### 🌐 Handlers Layer (`src/handlers/`)
- **职责**: HTTP 请求处理
- **包含**: 参数解析、响应格式化、错误处理
- **特点**: 轻量级，主要负责协议转换

#### 🛣 Routes Layer (`src/routes/`)
- **职责**: 路由配置和组织
- **包含**: URL 映射、路由分组
- **特点**: 集中化路由管理

#### ⚙️ Middleware Layer (`src/middleware/`)
- **职责**: 横切关注点
- **包含**: 日志、认证、CORS 等
- **特点**: 可插拔的功能增强

#### 🔧 Utils Layer (`src/utils/`)
- **职责**: 通用工具函数
- **包含**: 验证、加密、格式化等
- **特点**: 无状态、可复用

#### ❌ Errors Layer (`src/errors/`)
- **职责**: 统一错误处理
- **包含**: 错误类型定义、HTTP 响应映射
- **特点**: 类型安全的错误处理

## 🔄 数据流

### 请求处理流程

```
HTTP Request
    ↓
Middleware (logging, auth, etc.)
    ↓
Routes (URL matching)
    ↓
Handlers (parameter parsing)
    ↓
Services (business logic)
    ↓
Database (data persistence)
    ↓
Services (result processing)
    ↓
Handlers (response formatting)
    ↓
HTTP Response
```

### 错误处理流程

```
Error Occurrence
    ↓
Custom Error Type (AppError)
    ↓
Error Handler (automatic conversion)
    ↓
HTTP Error Response
```

## 🧪 测试策略

### 测试分层

1. **单元测试**: 在各模块内部（`#[cfg(test)]`）
2. **集成测试**: 在 `tests/` 目录
3. **API 测试**: 使用 `test_api.sh` 脚本

### 测试覆盖

- ✅ 模型验证逻辑
- ✅ 业务逻辑（services）
- ✅ HTTP 接口（integration tests）
- ✅ 数据库操作

## 🔒 安全考虑

1. **密码安全**: bcrypt 哈希，不存储明文
2. **SQL 注入防护**: SQLx 参数化查询
3. **输入验证**: 多层验证（前端 + 后端）
4. **错误信息**: 不泄露敏感信息

## 📈 扩展性设计

### 水平扩展

- 无状态设计，支持多实例部署
- 数据库连接池优化
- 缓存层预留接口

### 功能扩展

- 模块化设计，易于添加新功能
- 中间件系统支持插件化
- 配置系统支持多环境

## 🚀 性能优化

1. **异步处理**: 全面使用 async/await
2. **连接池**: 数据库连接复用
3. **序列化优化**: 避免不必要的克隆
4. **错误处理**: 快速失败策略

## 📚 最佳实践

1. **模块边界清晰**: 职责单一，依赖明确
2. **错误处理完善**: 统一错误类型和处理
3. **类型安全**: 充分利用 Rust 类型系统
4. **文档完整**: 代码注释和架构文档
5. **测试覆盖**: 关键逻辑有测试保障