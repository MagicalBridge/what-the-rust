#!/bin/bash

# Redis 缓存功能测试脚本
BASE_URL="http://127.0.0.1:8080"
REDIS_HOST="127.0.0.1"
REDIS_PORT="6379"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印函数
print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# 检查Redis服务是否运行
check_redis() {
    print_header "检查Redis服务状态"
    
    if command -v redis-cli >/dev/null 2>&1; then
        print_info "Redis CLI 已安装"
        
        if redis-cli -h $REDIS_HOST -p $REDIS_PORT ping >/dev/null 2>&1; then
            print_success "Redis服务运行正常"
            
            # 显示当前Redis中的缓存key数量
            KEY_COUNT=$(redis-cli -h $REDIS_HOST -p $REDIS_PORT eval "return #redis.call('keys', '*')" 0)
            print_info "当前Redis中有 $KEY_COUNT 个缓存key"
            
            return 0
        else
            print_error "Redis服务未运行"
            print_warning "请启动Redis服务: redis-server"
            return 1
        fi
    else
        print_error "Redis CLI 未安装"
        print_warning "请安装Redis: brew install redis (macOS)"
        return 1
    fi
}

# 检查API服务是否运行
check_api() {
    print_header "检查API服务状态"
    
    if curl -s --connect-timeout 3 "$BASE_URL/health" >/dev/null 2>&1; then
        print_success "API服务运行正常"
        return 0
    else
        print_error "API服务未运行"
        print_warning "请启动API服务: cargo run"
        return 1
    fi
}

# 清除Redis缓存
clear_redis_cache() {
    print_header "清除Redis缓存"
    
    # 只删除应用相关的缓存key，不删除其他可能存在的Redis数据
    redis-cli -h $REDIS_HOST -p $REDIS_PORT eval "
        local keys = redis.call('keys', 'user:*')
        for i=1,#keys do
            redis.call('del', keys[i])
        end
        keys = redis.call('keys', 'users:*')
        for i=1,#keys do
            redis.call('del', keys[i])
        end
        return #keys
    " 0 >/dev/null 2>&1
    
    print_success "应用缓存已清除"
}

# 创建测试用户
create_test_user() {
    print_header "创建测试用户"
    
    USER_RESPONSE=$(curl -s -X POST "$BASE_URL/api/users" \
      -H "Content-Type: application/json" \
      -d '{
        "username": "redis_test_user_'$(date +%s)'",
        "email": "redis_test_'$(date +%s)'@example.com",
        "password": "password123456",
        "full_name": "Redis测试用户"
      }')
    
    if echo "$USER_RESPONSE" | jq -e '.success' >/dev/null 2>&1; then
        USER_ID=$(echo $USER_RESPONSE | jq -r '.data.id')
        USERNAME=$(echo $USER_RESPONSE | jq -r '.data.username')
        print_success "用户创建成功: ID=$USER_ID, Username=$USERNAME"
        
        # 显示创建的用户信息
        echo "$USER_RESPONSE" | jq '.data | {id, username, email, full_name, created_at}'
        
        return 0
    else
        print_error "用户创建失败"
        echo "$USER_RESPONSE" | jq '.'
        return 1
    fi
}

# 测试缓存性能对比
test_cache_performance() {
    print_header "缓存性能测试"
    
    if [ -z "$USER_ID" ]; then
        print_error "没有可用的测试用户ID"
        return 1
    fi
    
    print_info "测试用户ID查询性能..."
    
    # 第一次查询 - 从数据库加载并写入缓存
    print_info "第一次查询（数据库 → 缓存）..."
    START_TIME=$(python3 -c "import time; print(int(time.time() * 1000))")
    RESPONSE1=$(curl -s -X GET "$BASE_URL/api/users/$USER_ID")
    END_TIME=$(python3 -c "import time; print(int(time.time() * 1000))")
    TIME1=$(( END_TIME - START_TIME ))
    
    if echo "$RESPONSE1" | jq -e '.success' >/dev/null 2>&1; then
        print_success "第一次查询成功 - 耗时: ${TIME1}ms"
    else
        print_error "第一次查询失败"
        return 1
    fi
    
    # 等待一小段时间确保缓存写入完成
    sleep 0.5
    
    # 第二次查询 - 从缓存获取
    print_info "第二次查询（缓存命中）..."
    START_TIME=$(python3 -c "import time; print(int(time.time() * 1000))")
    RESPONSE2=$(curl -s -X GET "$BASE_URL/api/users/$USER_ID")
    END_TIME=$(python3 -c "import time; print(int(time.time() * 1000))")
    TIME2=$(( END_TIME - START_TIME ))
    
    if echo "$RESPONSE2" | jq -e '.success' >/dev/null 2>&1; then
        print_success "第二次查询成功 - 耗时: ${TIME2}ms"
        
        # 计算性能提升
        if [ $TIME1 -gt 0 ] && [ $TIME2 -gt 0 ]; then
            IMPROVEMENT=$(( (TIME1 - TIME2) * 100 / TIME1 ))
            if [ $IMPROVEMENT -gt 0 ]; then
                print_success "缓存性能提升: ${IMPROVEMENT}%"
            else
                print_warning "此次测试未显示明显性能提升（可能网络波动影响）"
            fi
        fi
    else
        print_error "第二次查询失败"
        return 1
    fi
    
    # 验证两次查询结果一致
    if [ "$(echo $RESPONSE1 | jq -c '.data')" = "$(echo $RESPONSE2 | jq -c '.data')" ]; then
        print_success "两次查询结果一致，缓存数据正确"
    else
        print_error "两次查询结果不一致，缓存可能有问题"
    fi
}

# 测试用户名查询缓存
test_username_cache() {
    print_header "用户名查询缓存测试"
    
    if [ -z "$USERNAME" ]; then
        print_error "没有可用的测试用户名"
        return 1
    fi
    
    print_info "测试用户名: $USERNAME"
    
    # 清除可能存在的用户名缓存
    redis-cli -h $REDIS_HOST -p $REDIS_PORT del "user:username:$USERNAME" >/dev/null 2>&1
    
    # 第一次按用户名查询
    print_info "第一次按用户名查询..."
    RESPONSE1=$(curl -s -X GET "$BASE_URL/api/users/username/$USERNAME")
    
    if echo "$RESPONSE1" | jq -e '.success' >/dev/null 2>&1; then
        print_success "按用户名查询成功"
        
        # 检查缓存是否已创建
        sleep 0.5
        if redis-cli -h $REDIS_HOST -p $REDIS_PORT exists "user:username:$USERNAME" | grep -q "1"; then
            print_success "用户名缓存已创建"
            
            # 第二次查询验证缓存命中
            print_info "第二次按用户名查询（验证缓存）..."
            RESPONSE2=$(curl -s -X GET "$BASE_URL/api/users/username/$USERNAME")
            
            if echo "$RESPONSE2" | jq -e '.success' >/dev/null 2>&1; then
                print_success "缓存查询成功"
                
                # 验证结果一致性
                if [ "$(echo $RESPONSE1 | jq -c '.data')" = "$(echo $RESPONSE2 | jq -c '.data')" ]; then
                    print_success "用户名缓存数据一致"
                else
                    print_error "用户名缓存数据不一致"
                fi
            else
                print_error "缓存查询失败"
            fi
        else
            print_warning "用户名缓存未创建"
        fi
    else
        print_error "按用户名查询失败"
        echo "$RESPONSE1" | jq '.'
    fi
}

# 测试用户列表缓存
test_users_list_cache() {
    print_header "用户列表缓存测试"
    
    # 清除用户列表缓存
    redis-cli -h $REDIS_HOST -p $REDIS_PORT del "users:all" >/dev/null 2>&1
    
    print_info "第一次查询用户列表..."
    RESPONSE1=$(curl -s -X GET "$BASE_URL/api/users")
    
    if echo "$RESPONSE1" | jq -e '.success' >/dev/null 2>&1; then
        USER_COUNT=$(echo "$RESPONSE1" | jq '.data | length')
        print_success "用户列表查询成功，共 $USER_COUNT 个用户"
        
        # 检查缓存
        sleep 0.5
        if redis-cli -h $REDIS_HOST -p $REDIS_PORT exists "users:all" | grep -q "1"; then
            print_success "用户列表缓存已创建"
            
            # 第二次查询验证缓存
            print_info "第二次查询用户列表（验证缓存）..."
            RESPONSE2=$(curl -s -X GET "$BASE_URL/api/users")
            
            if echo "$RESPONSE2" | jq -e '.success' >/dev/null 2>&1; then
                USER_COUNT2=$(echo "$RESPONSE2" | jq '.data | length')
                print_success "缓存查询成功，共 $USER_COUNT2 个用户"
                
                if [ "$USER_COUNT" = "$USER_COUNT2" ]; then
                    print_success "用户列表缓存数据一致"
                else
                    print_error "用户列表缓存数据不一致"
                fi
            else
                print_error "用户列表缓存查询失败"
            fi
        else
            print_warning "用户列表缓存未创建"
        fi
    else
        print_error "用户列表查询失败"
        echo "$RESPONSE1" | jq '.'
    fi
}

# 测试缓存失效机制
test_cache_invalidation() {
    print_header "缓存失效机制测试"
    
    if [ -z "$USER_ID" ]; then
        print_error "没有可用的测试用户ID"
        return 1
    fi
    
    # 确保用户数据已缓存
    curl -s -X GET "$BASE_URL/api/users/$USER_ID" >/dev/null 2>&1
    sleep 0.5
    
    # 检查缓存存在
    USER_CACHE_EXISTS=$(redis-cli -h $REDIS_HOST -p $REDIS_PORT exists "user:id:$USER_ID")
    USERNAME_CACHE_EXISTS=$(redis-cli -h $REDIS_HOST -p $REDIS_PORT exists "user:username:$USERNAME")
    ALL_USERS_CACHE_EXISTS=$(redis-cli -h $REDIS_HOST -p $REDIS_PORT exists "users:all")
    
    print_info "更新前缓存状态:"
    print_info "  用户ID缓存: $([ "$USER_CACHE_EXISTS" = "1" ] && echo "存在" || echo "不存在")"
    print_info "  用户名缓存: $([ "$USERNAME_CACHE_EXISTS" = "1" ] && echo "存在" || echo "不存在")"
    print_info "  用户列表缓存: $([ "$ALL_USERS_CACHE_EXISTS" = "1" ] && echo "存在" || echo "不存在")"
    
    # 更新用户信息
    print_info "更新用户信息..."
    UPDATE_RESPONSE=$(curl -s -X PUT "$BASE_URL/api/users/$USER_ID" \
      -H "Content-Type: application/json" \
      -d '{
        "email": "updated_redis_test_'$(date +%s)'@example.com",
        "full_name": "Redis测试用户（已更新）"
      }')
    
    if echo "$UPDATE_RESPONSE" | jq -e '.success' >/dev/null 2>&1; then
        print_success "用户信息更新成功"
        
        # 等待缓存清除操作完成
        sleep 1
        
        # 检查缓存是否被清除
        USER_CACHE_EXISTS_AFTER=$(redis-cli -h $REDIS_HOST -p $REDIS_PORT exists "user:id:$USER_ID")
        USERNAME_CACHE_EXISTS_AFTER=$(redis-cli -h $REDIS_HOST -p $REDIS_PORT exists "user:username:$USERNAME")
        ALL_USERS_CACHE_EXISTS_AFTER=$(redis-cli -h $REDIS_HOST -p $REDIS_PORT exists "users:all")
        
        print_info "更新后缓存状态:"
        print_info "  用户ID缓存: $([ "$USER_CACHE_EXISTS_AFTER" = "1" ] && echo "存在" || echo "已清除")"
        print_info "  用户名缓存: $([ "$USERNAME_CACHE_EXISTS_AFTER" = "1" ] && echo "存在" || echo "已清除")"
        print_info "  用户列表缓存: $([ "$ALL_USERS_CACHE_EXISTS_AFTER" = "1" ] && echo "存在" || echo "已清除")"
        
        # 验证缓存失效
        if [ "$USER_CACHE_EXISTS_AFTER" = "0" ] && [ "$USERNAME_CACHE_EXISTS_AFTER" = "0" ] && [ "$ALL_USERS_CACHE_EXISTS_AFTER" = "0" ]; then
            print_success "缓存失效机制工作正常，相关缓存已清除"
        else
            print_warning "部分缓存未被清除，可能需要检查失效逻辑"
        fi
        
        # 验证更新后的数据是否正确
        print_info "验证更新后的数据..."
        VERIFY_RESPONSE=$(curl -s -X GET "$BASE_URL/api/users/$USER_ID")
        if echo "$VERIFY_RESPONSE" | jq -e '.success' >/dev/null 2>&1; then
            UPDATED_EMAIL=$(echo "$VERIFY_RESPONSE" | jq -r '.data.email')
            UPDATED_FULL_NAME=$(echo "$VERIFY_RESPONSE" | jq -r '.data.full_name')
            print_success "更新验证成功"
            print_info "  新邮箱: $UPDATED_EMAIL"
            print_info "  新姓名: $UPDATED_FULL_NAME"
        else
            print_error "更新验证失败"
        fi
    else
        print_error "用户信息更新失败"
        echo "$UPDATE_RESPONSE" | jq '.'
    fi
}

# 显示Redis缓存状态
show_redis_status() {
    print_header "Redis缓存状态"
    
    # 显示应用相关的所有缓存key
    echo "当前应用缓存key:"
    redis-cli -h $REDIS_HOST -p $REDIS_PORT eval "
        local user_keys = redis.call('keys', 'user:*')
        local users_keys = redis.call('keys', 'users:*')
        local all_keys = {}
        for i=1,#user_keys do
            table.insert(all_keys, user_keys[i])
        end
        for i=1,#users_keys do
            table.insert(all_keys, users_keys[i])
        end
        return all_keys
    " 0 | while read -r key; do
        if [ -n "$key" ] && [ "$key" != "(empty list or set)" ]; then
            TTL=$(redis-cli -h $REDIS_HOST -p $REDIS_PORT ttl "$key")
            if [ "$TTL" = "-1" ]; then
                TTL_STR="永久"
            elif [ "$TTL" = "-2" ]; then
                TTL_STR="已过期"
            else
                TTL_STR="${TTL}秒"
            fi
            print_info "  $key (TTL: $TTL_STR)"
        fi
    done
    
    # 显示Redis基本信息
    echo -e "\nRedis基本信息:"
    REDIS_INFO=$(redis-cli -h $REDIS_HOST -p $REDIS_PORT info server | grep -E "redis_version|os|tcp_port")
    echo "$REDIS_INFO" | while read -r line; do
        if [ -n "$line" ]; then
            print_info "  $line"
        fi
    done
}

# 清理测试数据
cleanup_test_data() {
    print_header "清理测试数据"
    
    if [ -n "$USER_ID" ]; then
        print_info "删除测试用户: $USER_ID"
        DELETE_RESPONSE=$(curl -s -X DELETE "$BASE_URL/api/users/$USER_ID")
        
        if echo "$DELETE_RESPONSE" | jq -e '.success' >/dev/null 2>&1; then
            print_success "测试用户删除成功"
        else
            print_warning "测试用户删除失败（可能已被删除）"
        fi
    fi
    
    # 清除可能残留的测试缓存
    print_info "清理残留缓存..."
    redis-cli -h $REDIS_HOST -p $REDIS_PORT eval "
        local keys = redis.call('keys', '*redis_test*')
        for i=1,#keys do
            redis.call('del', keys[i])
        end
        return #keys
    " 0 >/dev/null 2>&1
    
    print_success "测试数据清理完成"
}

# 主测试流程
main() {
    print_header "🧪 Redis缓存功能测试"
    echo -e "${BLUE}本测试将验证Redis缓存的完整功能${NC}"
    echo ""
    
    # 检查依赖
    if ! check_redis; then
        exit 1
    fi
    
    if ! check_api; then
        exit 1
    fi
    
    # 清除旧缓存
    clear_redis_cache
    
    # 创建测试用户
    if ! create_test_user; then
        exit 1
    fi
    
    echo ""
    
    # 运行各项测试
    test_cache_performance
    echo ""
    
    test_username_cache
    echo ""
    
    test_users_list_cache
    echo ""
    
    test_cache_invalidation
    echo ""
    
    show_redis_status
    echo ""
    
    # 询问是否清理测试数据
    echo -e "${YELLOW}是否清理测试数据？(y/n) [默认: y]${NC}"
    read -r -t 10 CLEANUP_CHOICE
    CLEANUP_CHOICE=${CLEANUP_CHOICE:-y}
    
    if [ "$CLEANUP_CHOICE" = "y" ] || [ "$CLEANUP_CHOICE" = "Y" ]; then
        cleanup_test_data
    else
        print_info "保留测试数据，用户ID: $USER_ID"
    fi
    
    echo ""
    print_header "🎉 Redis缓存测试完成"
    print_success "所有测试已执行完毕，请查看服务器日志了解详细的缓存操作"
    print_info "你可以使用以下命令监控Redis操作："
    print_info "  redis-cli monitor"
    print_info "  redis-cli keys '*'"
}

# 运行主程序
main "$@"