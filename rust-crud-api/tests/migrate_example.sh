#!/bin/bash

# 数据库迁移示例脚本
# 演示生产环境迁移最佳实践

echo "🚀 数据库迁移系统演示"
echo "========================"

# 检查环境变量
if [ -z "$DATABASE_URL" ]; then
    echo "❌ 请设置 DATABASE_URL 环境变量"
    echo '例如: export DATABASE_URL="postgresql://username:password@localhost/database"'
    exit 1
fi

echo "📋 可用的迁移命令："
echo ""
echo "1. 运行所有待执行的迁移:"
echo "   cargo run --bin migrate up"
echo ""
echo "2. 回滚最近的1步迁移:"
echo "   cargo run --bin migrate down 1"
echo ""
echo "3. 回滚最近的3步迁移:"
echo "   cargo run --bin migrate down 3"
echo ""

# 询问用户想要执行的操作
echo "请选择要执行的操作："
echo "1) 运行迁移 (up)"
echo "2) 回滚迁移 (down)"
echo "3) 退出"
read -p "输入选择 (1-3): " choice

case $choice in
    1)
        echo "🚀 开始运行迁移..."
        cargo run --bin migrate up
        ;;
    2)
        read -p "请输入要回滚的步数: " steps
        echo "⏪ 开始回滚 $steps 步迁移..."
        cargo run --bin migrate down $steps
        ;;
    3)
        echo "👋 退出"
        exit 0
        ;;
    *)
        echo "❌ 无效的选择"
        exit 1
        ;;
esac

echo ""
echo "✅ 操作完成！"
echo ""
echo "💡 提示："
echo "- 在生产环境中，请先备份数据库"
echo "- 建议在测试环境中先验证迁移"
echo "- 迁移历史保存在 _sqlx_migrations 表中"