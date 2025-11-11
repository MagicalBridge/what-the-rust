-- 创建 vault_deposits 表用于记录入金事件
CREATE TABLE IF NOT EXISTS vault_deposits (
    id BIGSERIAL PRIMARY KEY,
    tx_hash TEXT NOT NULL UNIQUE,
    block_number BIGINT NOT NULL,
    tx_index BIGINT,
    sender TEXT NOT NULL,
    to_address TEXT NOT NULL,
    amount_wei TEXT NOT NULL,
    token_address TEXT,
    status TEXT NOT NULL DEFAULT 'confirmed',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 进度表：记录监听器已处理到的区块
CREATE TABLE IF NOT EXISTS indexer_progress (
    source TEXT PRIMARY KEY,
    last_block_number BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);