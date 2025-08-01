#!/bin/bash

# Redis 缓存演示脚本 - 用户手动操作演示
BASE_URL="http://127.0.0.1:8080"

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}🎯 Redis缓存功能演示${NC}"
echo -e "${BLUE}================================${NC}"
echo -e "${YELLOW}请按任意键开始演示...${NC}"
read -n 1

echo -e "\n${BLUE}步骤1: 创建一个测试用户${NC}"
echo "curl -X POST $BASE_URL/api/users -H 'Content-Type: application/json' -d '{\"username\": \"demo_user\", \"email\": \"demo@example.com\", \"password\": \"password123\", \"full_name\": \"演示用户\"}'"

USER_RESPONSE=$(curl -s -X POST "$BASE_URL/api/users" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "demo_user_'$(date +%s)'",
    "email": "demo_'$(date +%s)'@example.com",
    "password": "password123456",
    "full_name": "Redis演示用户"
  }')

USER_ID=$(echo $USER_RESPONSE | jq -r '.data.id')
USERNAME=$(echo $USER_RESPONSE | jq -r '.data.username')

echo -e "${GREEN}✅ 用户创建成功！${NC}"
echo "用户ID: $USER_ID"
echo "用户名: $USERNAME"

echo -e "\n${YELLOW}按任意键继续...${NC}"
read -n 1

echo -e "\n${BLUE}步骤2: 第一次查询用户（从数据库）${NC}"
echo "curl -s $BASE_URL/api/users/$USER_ID"
echo -e "${YELLOW}观察: 这次查询会从数据库获取数据并写入Redis缓存${NC}"

curl -s -X GET "$BASE_URL/api/users/$USER_ID" | jq '.'

echo -e "\n${YELLOW}按任意键继续...${NC}"
read -n 1

echo -e "\n${BLUE}步骤3: 检查Redis中的缓存${NC}"
echo "redis-cli keys 'user:*'"
echo -e "${GREEN}当前Redis中的用户缓存:${NC}"
redis-cli keys 'user:*'

echo -e "\n${BLUE}查看具体缓存内容:${NC}"
redis-cli get "user:id:$USER_ID" | jq '.'

echo -e "\n${YELLOW}按任意键继续...${NC}"
read -n 1

echo -e "\n${BLUE}步骤4: 第二次查询同一用户（从缓存）${NC}"
echo "curl -s $BASE_URL/api/users/$USER_ID"
echo -e "${YELLOW}观察: 这次查询会直接从Redis缓存获取数据，速度更快${NC}"

curl -s -X GET "$BASE_URL/api/users/$USER_ID" | jq '.'

echo -e "\n${YELLOW}按任意键继续...${NC}"
read -n 1

echo -e "\n${BLUE}步骤5: 更新用户信息（观察缓存失效）${NC}"
echo "curl -X PUT $BASE_URL/api/users/$USER_ID -H 'Content-Type: application/json' -d '{\"full_name\": \"更新后的演示用户\"}'"

curl -s -X PUT "$BASE_URL/api/users/$USER_ID" \
  -H "Content-Type: application/json" \
  -d '{"full_name": "更新后的演示用户"}' | jq '.'

echo -e "\n${BLUE}检查缓存是否被清除:${NC}"
echo "redis-cli keys 'user:*'"
redis-cli keys 'user:*'

echo -e "\n${YELLOW}按任意键继续...${NC}"
read -n 1

echo -e "\n${BLUE}步骤6: 再次查询用户（重新缓存）${NC}"
echo "curl -s $BASE_URL/api/users/$USER_ID"
echo -e "${YELLOW}观察: 缓存已被清除，这次查询会重新从数据库获取并缓存${NC}"

curl -s -X GET "$BASE_URL/api/users/$USER_ID" | jq '.'

echo -e "\n${BLUE}验证新的缓存内容:${NC}"
redis-cli get "user:id:$USER_ID" | jq '.'

echo -e "\n${YELLOW}按任意键清理测试数据...${NC}"
read -n 1

echo -e "\n${BLUE}清理测试数据${NC}"
curl -s -X DELETE "$BASE_URL/api/users/$USER_ID" | jq '.'

echo -e "\n${GREEN}🎉 Redis缓存演示完成！${NC}"
echo -e "${BLUE}================================${NC}"
echo -e "${YELLOW}主要观察点:${NC}"
echo "1. 第一次查询后，Redis中出现了缓存key"
echo "2. 第二次查询直接从缓存获取，没有查询数据库"
echo "3. 更新用户后，相关缓存被自动清除"
echo "4. 再次查询时重新建立缓存"
echo ""
echo -e "${YELLOW}你可以使用以下命令监控Redis操作:${NC}"
echo "redis-cli monitor    # 实时监控Redis操作"
echo "redis-cli keys '*'   # 查看所有缓存key"
echo "redis-cli flushall   # 清空所有缓存（慎用）"