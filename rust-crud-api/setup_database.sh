#!/bin/bash

# PostgreSQL 数据库设置脚本
echo "🚀 开始设置 rust-crud-api 项目数据库"
echo "================================"

# 数据库配置
DB_NAME="rust_crud_db"
DB_USER="chupengfei"
DB_PASSWORD="123456"
DB_HOST="localhost"
DB_PORT="5432"

echo "📋 数据库配置信息:"
echo "  数据库名: $DB_NAME"
echo "  用户名: $DB_USER"
echo "  密码: $DB_PASSWORD"
echo "  主机: $DB_HOST"
echo "  端口: $DB_PORT"
echo ""

# 检查 PostgreSQL 是否运行
echo "🔍 检查 PostgreSQL 服务状态..."
if ! pg_isready -h $DB_HOST -p $DB_PORT > /dev/null 2>&1; then
    echo "❌ PostgreSQL 服务未运行"
    echo "请启动 PostgreSQL 服务："
    echo "  macOS: brew services start postgresql"
    echo "  Linux: sudo systemctl start postgresql"
    echo "  Windows: net start postgresql-x64-[version]"
    exit 1
fi

echo "✅ PostgreSQL 服务正在运行"

# 连接到 PostgreSQL 并执行数据库设置
echo ""
echo "🔧 创建数据库..."

# 创建数据库设置的 SQL 脚本
cat > /tmp/setup_db.sql << EOF
-- 检查并创建数据库
SELECT 'CREATE DATABASE $DB_NAME OWNER $DB_USER' 
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = '$DB_NAME')\gexec

EOF

# 执行 SQL 脚本 (使用你的用户凭据)
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d postgres -f /tmp/setup_db.sql

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ 数据库设置完成！"
    
    # 清理临时文件
    rm /tmp/setup_db.sql
    
    # 生成 .env 文件
    echo ""
    echo "📝 生成 .env 配置文件..."
    cat > .env << EOF
DATABASE_URL=postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
EOF
    
    echo "✅ .env 文件已生成"
    
    # 测试连接
    echo ""
    echo "🧪 测试数据库连接..."
    if PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "SELECT version();" > /dev/null 2>&1; then
        echo "✅ 数据库连接测试成功！"
        echo ""
        echo "🎉 数据库设置完成！"
        echo ""
        echo "📋 连接信息："
        echo "  DATABASE_URL=postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"
        echo ""
        echo "🚀 现在可以运行项目："
        echo "  cargo run"
    else
        echo "❌ 数据库连接测试失败"
        echo "请检查配置或手动测试连接"
    fi
else
    echo ""
    echo "❌ 数据库设置失败"
    echo "请检查 PostgreSQL 配置和权限"
    rm -f /tmp/setup_db.sql
    exit 1
fi