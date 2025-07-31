# 数据库迁移指南

## 🎯 生产环境迁移最佳实践

### ✅ 推荐的做法（已实现）

1. **独立的SQL文件**: 将SQL代码从应用代码中分离
2. **版本控制**: 使用时间戳命名确保迁移顺序
3. **原子性**: 每个迁移文件作为单个事务执行
4. **可回滚**: 支持迁移回滚功能
5. **CLI工具**: 独立的迁移管理工具

### ❌ 避免的做法（原实现）

1. **硬编码SQL**: 在代码中直接写SQL语句
2. **无版本追踪**: 无法确定数据库当前状态
3. **不可回滚**: 出现问题时难以恢复
4. **混合关注点**: 业务逻辑与数据库结构混合

## 🚀 使用方法

### 运行迁移
```bash
# 应用所有待执行的迁移
cargo run --bin migrate up
```

### 回滚迁移
```bash
# 回滚最近的1步迁移
cargo run --bin migrate down 1

# 回滚最近的3步迁移
cargo run --bin migrate down 3
```

## 📁 文件结构

```
src/migrations/
├── 20241201000001_create_users_table.sql   # 创建用户表
├── 20241201000002_add_user_roles.sql       # 添加用户角色（示例）
└── 20241201000003_create_posts_table.sql   # 创建文章表（示例）
```

## 📝 创建新迁移

### 1. 命名规范
```
{timestamp}_{description}.sql
```
例如：`20241201120000_add_email_verification.sql`

### 2. 迁移文件内容
```sql
-- 20241201120000_add_email_verification.sql
-- 添加邮箱验证功能

ALTER TABLE users 
ADD COLUMN email_verified BOOLEAN DEFAULT FALSE,
ADD COLUMN verification_token VARCHAR(255);

CREATE INDEX idx_users_verification_token ON users(verification_token);
```

## 🏭 生产环境部署流程

### 1. 开发环境
```bash
# 1. 创建迁移文件
# 2. 测试迁移
cargo run --bin migrate up

# 3. 测试回滚
cargo run --bin migrate down 1
cargo run --bin migrate up
```

### 2. 测试环境
```bash
# 部署前运行迁移
cargo run --bin migrate up
```

### 3. 生产环境
```bash
# 备份数据库
pg_dump your_database > backup_$(date +%Y%m%d_%H%M%S).sql

# 运行迁移
cargo run --bin migrate up

# 验证迁移结果
psql -d your_database -c "\dt"  # 检查表结构
```

## 🔧 SQLx 迁移系统特性

### 自动追踪
- SQLx 自动创建 `_sqlx_migrations` 表来追踪迁移历史
- 每个迁移只会执行一次
- 支持校验和检查，防止迁移文件被意外修改

### 事务安全
- 每个迁移文件在单独的事务中执行
- 失败时自动回滚，保证数据库一致性

### 性能优化
- 支持并发安全的迁移执行
- 适合生产环境的高可用部署

## 🌟 与其他工具对比

| 特性 | 自定义代码 | SQLx Migrate | Diesel | Sea-ORM |
|------|------------|--------------|---------|---------|
| SQL文件支持 | ❌ | ✅ | ✅ | ✅ |
| 版本追踪 | ❌ | ✅ | ✅ | ✅ |
| 回滚支持 | ❌ | ✅ | ✅ | ✅ |
| CLI工具 | ❌ | ✅ | ✅ | ✅ |
| 学习成本 | 低 | 低 | 中 | 中 |

## 📚 参考资源

- [SQLx Migrations 文档](https://docs.rs/sqlx/latest/sqlx/migrate/index.html)
- [数据库迁移最佳实践](https://martinfowler.com/articles/evodb.html)
- [PostgreSQL 迁移策略](https://www.postgresql.org/docs/current/ddl-alter.html)