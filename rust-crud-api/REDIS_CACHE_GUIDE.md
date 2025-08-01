# Redis 缓存集成指南

## 概述

你的 Rust CRUD API 项目现在已经成功集成了 Redis 缓存功能！此功能将显著提升用户查询接口的性能，减轻数据库负载。

## 🎯 功能特性

### 缓存的接口
- **GET /api/users/{id}** - 根据用户ID查询（缓存支持）
- **GET /api/users/username/{username}** - 根据用户名查询（缓存支持）
- **GET /api/users** - 获取所有用户列表（缓存支持）

### 缓存配置
- **默认TTL**: 2分钟（120秒）
- **Redis地址**: `redis://127.0.0.1:6379`（默认）
- **缓存策略**: 优先查询缓存，未命中时查询数据库并写入缓存

## ⚙️ 环境配置

### 环境变量
你可以通过环境变量自定义缓存配置：

```bash
# Redis服务器地址（可选，默认本地Redis）
REDIS_URL=redis://127.0.0.1:6379

# 缓存时效性（可选，默认120秒）
CACHE_TTL_SECONDS=120
```

### .env 文件示例
在项目根目录的 `.env` 文件中添加：
```
DATABASE_URL=postgresql://username:password@localhost/dbname
REDIS_URL=redis://127.0.0.1:6379
CACHE_TTL_SECONDS=120
```

## 🚀 启动项目

1. **确保Redis服务运行**：
   ```bash
   # macOS (使用 Homebrew)
   redis-server
   
   # 或者后台运行
   brew services start redis
   ```

2. **启动API服务**：
   ```bash
   cargo run
   ```

3. **查看启动日志**：
   ```
   🔗 正在连接Redis: redis://127.0.0.1:6379
   ✅ Redis连接建立成功
   ✅ Redis连接测试成功
   🚀 服务器启动在 http://127.0.0.1:8080
   💾 Redis缓存已启用，TTL: 120秒
   ```

## 📊 缓存工作原理

### 查询流程
1. **接收查询请求** → 检查Redis缓存
2. **缓存命中** → 直接返回缓存数据 ⚡
3. **缓存未命中** → 查询数据库 → 写入缓存 → 返回数据

### 缓存失效策略
- **用户创建**: 清除"所有用户列表"缓存
- **用户更新**: 清除该用户的所有相关缓存（ID、用户名、用户列表）
- **用户删除**: 清除该用户的所有相关缓存

### 缓存Key规则
- 用户ID查询: `user:id:{uuid}`
- 用户名查询: `user:username:{username}`
- 所有用户列表: `users:all`

## 🔍 性能监控

### 日志观察
在应用运行时，你会看到详细的缓存操作日志：

```
🎯 缓存命中: user:id:123e4567-e89b-12d3-a456-426614174000
💾 用户数据已缓存: id=123e4567-e89b-12d3-a456-426614174000
🗑️ 缓存删除成功: users:all
```

### Redis命令行监控
你可以使用Redis CLI监控缓存状态：

```bash
# 连接Redis
redis-cli

# 查看所有缓存key
KEYS *

# 查看特定key的值
GET "user:id:123e4567-e89b-12d3-a456-426614174000"

# 查看key的TTL
TTL "user:id:123e4567-e89b-12d3-a456-426614174000"

# 清除所有缓存
FLUSHALL
```

## 🎭 测试缓存效果

### 1. 创建测试用户
```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123",
    "full_name": "Test User"
  }'
```

### 2. 测试查询性能
```bash
# 第一次查询（从数据库）
time curl http://localhost:8080/api/users/{user_id}

# 第二次查询（从缓存）- 应该更快
time curl http://localhost:8080/api/users/{user_id}
```

### 3. 观察日志差异
- 第一次：应该看到数据库查询和缓存写入日志
- 第二次：应该看到"🎯 缓存命中"日志

## 🛠️ 故障排除

### Redis连接失败
1. 确保Redis服务正在运行：`redis-cli ping`
2. 检查Redis服务地址配置
3. 查看防火墙设置

### 缓存不工作
1. 检查Redis连接是否成功建立
2. 查看应用日志中的缓存操作信息
3. 使用`redis-cli MONITOR`实时观察Redis操作

### 性能问题
1. 调整`CACHE_TTL_SECONDS`值
2. 监控Redis内存使用情况
3. 考虑实施缓存分片策略

## 📈 预期性能提升

- **查询响应时间**: 减少60-90%（具体取决于数据库复杂度）
- **数据库负载**: 显著降低重复查询压力
- **用户体验**: 更快的API响应速度

## 🔄 后续优化建议

1. **缓存预热**: 在应用启动时预加载热点数据
2. **缓存分层**: 实现多级缓存策略
3. **缓存监控**: 添加缓存命中率统计
4. **缓存压缩**: 对大型数据实施压缩存储

---

🎉 **恭喜！你的API现在拥有了高性能的Redis缓存功能！**