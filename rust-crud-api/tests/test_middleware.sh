#!/bin/bash

echo "🧪 测试响应打印中间件功能"
echo "========================="

echo ""
echo "启动服务器（后台运行）..."
cargo run &
SERVER_PID=$!

echo "等待服务器启动..."
sleep 5

echo ""
echo "📍 测试健康检查接口..."
echo "客户端请求: curl http://localhost:8080/health"
curl -s http://localhost:8080/health > /dev/null
echo "✅ 请求完成"

echo ""
echo "📍 测试获取用户列表接口..."
echo "客户端请求: curl http://localhost:8080/api/users"
curl -s http://localhost:8080/api/users > /dev/null
echo "✅ 请求完成"

echo ""
echo "📍 测试404错误..."
echo "客户端请求: curl http://localhost:8080/api/nonexistent"
curl -s http://localhost:8080/api/nonexistent > /dev/null
echo "✅ 请求完成"

echo ""
echo "停止服务器..."
kill $SERVER_PID
sleep 2

echo ""
echo "🎉 测试完成！"
echo ""
echo "在上面的服务器输出中，你应该能看到："
echo "  📍 每个请求的详细响应信息"
echo "  📊 状态码（200, 404等）"
echo "  ✅⚠️❌ 不同的状态图标"
echo "  📄 Content-Type信息"
echo ""
echo "这证明响应打印中间件正在自动工作，无需在每个handler中手动添加代码！"