#!/bin/bash

# 演示API响应打印功能的脚本
echo "🎯 演示：API接口响应打印调试功能"
echo "====================================="

echo ""
echo "1️⃣ 首先启动项目："
echo "   cargo run"
echo ""

echo "2️⃣ 在另一个终端窗口中调用API接口："
echo ""

echo "🏥 健康检查："
echo 'curl http://localhost:8080/health'
echo ""

echo "🧑‍💼 创建用户："
echo 'curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d "{
    \"username\": \"zhangsan\",
    \"email\": \"zhangsan@example.com\", 
    \"password\": \"password123\",
    \"full_name\": \"张三\"
  }"'
echo ""

echo "📋 获取所有用户："
echo 'curl http://localhost:8080/api/users'
echo ""

echo "🔍 根据用户名查询："
echo 'curl http://localhost:8080/api/users/username/zhangsan'
echo ""

echo "✏️ 更新用户信息："
echo 'curl -X PUT http://localhost:8080/api/users/{用户ID} \
  -H "Content-Type: application/json" \
  -d "{
    \"email\": \"newemail@example.com\",
    \"full_name\": \"张三丰\"
  }"'
echo ""

echo "🗑️ 删除用户："
echo 'curl -X DELETE http://localhost:8080/api/users/{用户ID}'
echo ""

echo "✨ 功能特性："
echo "   - 使用中间件架构，非侵入式自动打印所有API响应信息"
echo "   - 显示接口路径、HTTP方法、状态码和响应状态"
echo "   - 包含响应头信息（如Content-Type）"
echo "   - 成功和错误请求都会被自动监控和打印"
echo "   - 无需在每个处理器中手动添加打印代码"
echo ""

echo "📖 服务器终端输出示例："
echo "   🔍 API响应调试信息"
echo "   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "   📍 接口: POST /api/users"
echo "   📊 状态码: 201 Created"
echo "   ✅ 响应状态: 成功"
echo "   📄 内容类型: application/json"
echo "   💡 提示: 响应体内容会在请求日志中间件显示"
echo "   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"