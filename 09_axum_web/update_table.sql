-- 修改users表结构以匹配应用程序期望的字段

-- 添加新字段
ALTER TABLE users ADD COLUMN IF NOT EXISTS name VARCHAR(100);
ALTER TABLE users ADD COLUMN IF NOT EXISTS age INTEGER CHECK (age >= 0 AND age <= 150);

-- 如果name字段为空，从full_name复制数据
UPDATE users SET name = full_name WHERE name IS NULL AND full_name IS NOT NULL;

-- 设置name字段为非空
ALTER TABLE users ALTER COLUMN name SET NOT NULL;

-- 删除不需要的字段
ALTER TABLE users DROP COLUMN IF EXISTS username;
ALTER TABLE users DROP COLUMN IF EXISTS password_hash;
ALTER TABLE users DROP COLUMN IF EXISTS full_name;

-- 创建新的索引
CREATE INDEX IF NOT EXISTS idx_users_name ON users(name);
CREATE INDEX IF NOT EXISTS idx_users_age ON users(age);