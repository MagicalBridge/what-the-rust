#!/bin/bash

# API 测试脚本 - 用于测试详细日志功能
BASE_URL="http://127.0.0.1:8080"

echo "🧪 开始测试 Rust CRUD API（带详细日志）"
echo "================================"

# 健康检查
echo "1. 健康检查..."
curl -s -X GET "$BASE_URL/health" | jq '.'
sleep 1

# 创建用户 (注册)
echo -e "\n2. 创建用户 (注册)..."
USER_RESPONSE=$(curl -s -X POST "$BASE_URL/api/users" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser123",
    "email": "test123@example.com",
    "password": "password123456",
    "full_name": "测试用户123"
  }')
echo $USER_RESPONSE | jq '.'

# 提取用户 ID
USER_ID=$(echo $USER_RESPONSE | jq -r '.data.id')
echo "用户 ID: $USER_ID"
sleep 1

# 获取所有用户
echo -e "\n3. 获取所有用户..."
curl -s -X GET "$BASE_URL/api/users" | jq '.'
sleep 1

# 根据 ID 获取用户
echo -e "\n4. 根据 ID 获取用户..."
curl -s -X GET "$BASE_URL/api/users/$USER_ID" | jq '.'
sleep 1

# 根据用户名获取用户
echo -e "\n5. 根据用户名获取用户..."
curl -s -X GET "$BASE_URL/api/users/username/testuser123" | jq '.'
sleep 1

# 更新用户信息
echo -e "\n6. 更新用户信息..."
curl -s -X PUT "$BASE_URL/api/users/$USER_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "updated123@example.com",
    "full_name": "更新后的用户名123"
  }' | jq '.'
sleep 1

# 再次获取用户信息确认更新
echo -e "\n7. 确认更新结果..."
curl -s -X GET "$BASE_URL/api/users/$USER_ID" | jq '.'
sleep 1

# 删除用户
echo -e "\n8. 删除用户..."
curl -s -X DELETE "$BASE_URL/api/users/$USER_ID" | jq '.'
sleep 1

# 确认删除
echo -e "\n9. 确认删除结果..."
curl -s -X GET "$BASE_URL/api/users/$USER_ID" | jq '.'

echo -e "\n✅ API 测试完成！请查看服务器控制台的详细日志。"