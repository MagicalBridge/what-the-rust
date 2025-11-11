use crate::{Config, database::DatabasePool};
use ethers::prelude::*;
use ethers::providers::{Provider, Ws, Http};
use futures_util::StreamExt;
use std::sync::Arc;

/// 将 U256 格式化为十进制字符串（wei）
fn u256_to_string(v: U256) -> String { format!("{}", v) }

/// 获取进度（last_block_number）
async fn get_last_block(pool: &DatabasePool, source: &str) -> Result<Option<i64>, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT last_block_number FROM indexer_progress WHERE source = $1"
    )
    .bind(source)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

/// 更新进度表
async fn update_last_block(pool: &DatabasePool, source: &str, last_block: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO indexer_progress (source, last_block_number, updated_at)
         VALUES ($1, $2, NOW())
         ON CONFLICT (source) DO UPDATE SET last_block_number = EXCLUDED.last_block_number, updated_at = NOW()"
    )
    .bind(source)
    .bind(last_block)
    .execute(pool)
    .await?;
    Ok(())
}

/// 幂等插入一条入金记录（ERC20，记录 token_address）
async fn insert_deposit(
    pool: &DatabasePool,
    tx_hash: &str,
    block_number: i64,
    tx_index: Option<i64>,
    sender: &str,
    to_address: &str,
    amount_wei: &str,
    token_address: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO vault_deposits (tx_hash, block_number, tx_index, sender, to_address, amount_wei, token_address, status)
         VALUES ($1, $2, $3, $4, $5, $6, $7, 'confirmed')
         ON CONFLICT (tx_hash) DO NOTHING"
    )
    .bind(tx_hash)
    .bind(block_number)
    .bind(tx_index)
    .bind(sender)
    .bind(to_address)
    .bind(amount_wei)
    .bind(token_address)
    .execute(pool)
    .await?;
    Ok(())
}

/// 启动 Arbitrum 上的 Vault 监听器：
/// - 补扫区块：从上次处理到的区块到最新区块
/// - 实时监听：订阅新块，筛选直接向合约地址转入的 ETH 交易
pub async fn start_vault_watcher(config: Config, pool: DatabasePool) -> anyhow::Result<()> {
    if !config.enable_vault_watcher {
        log::info!("🔕 Vault 监听已禁用（ENABLE_VAULT_WATCHER=false）");
        return Ok(());
    }

    let http_url = match &config.arbitrum_http_url { Some(u) => u.clone(), None => anyhow::bail!("ARBITRUM_HTTP_URL 未设置") };
    let ws_url = match &config.arbitrum_ws_url { Some(u) => u.clone(), None => anyhow::bail!("ARBITRUM_WS_URL 未设置") };
    let vault_addr_str = match &config.vault_contract_address { Some(a) => a.clone(), None => anyhow::bail!("VAULT_CONTRACT_ADDRESS 未设置") };
    let usdc_addr_str = match &config.usdc_token_address { Some(a) => a.clone(), None => anyhow::bail!("USDC_TOKEN_ADDRESS 未设置") };

    let vault_addr: Address = vault_addr_str.parse()?;
    let usdc_addr: Address = usdc_addr_str.parse()?;
    let source = "arbitrum_vault";

    // HTTP provider 用于补扫
    let http = Provider::<Http>::try_from(http_url.clone())?;
    let latest = http.get_block_number().await?.as_u64() as i64;

    // 选择起始区块：优先用进度表，否则从最新往前回溯 10 个块
    let start_block = match get_last_block(&pool, source).await? {
        Some(last) => last + 1,
        None => (latest - 10).max(0),
    };

    if start_block <= latest {
        log::info!("📦 开始补扫 USDC Transfer 事件: {} -> {}", start_block, latest);
        // 通过 get_logs 拉取 USDC 的 Transfer 事件，再过滤 to == vault
        let filter = Filter::new()
            .address(usdc_addr)
            .from_block(start_block as u64)
            .to_block(latest as u64);

        let logs = http.get_logs(&filter).await?;
        for log in logs {
            // 只处理标准 Transfer 事件（主题数量为3，且包含 to）
            if log.topics.len() == 3 {
                // 主题2为 to 地址（indexed）
                let to_topic = log.topics[2];
                let to = Address::from_slice(&to_topic.as_bytes()[12..]);
                if to == vault_addr {
                    // 主题1为 from 地址
                    let from_topic = log.topics[1];
                    let from = Address::from_slice(&from_topic.as_bytes()[12..]);
                    // data 为 amount（uint256）
                    let amount = U256::from_big_endian(log.data.as_ref());

                    let tx_hash = format!("0x{:x}", log.transaction_hash.unwrap_or_default());
                    let block_number = log.block_number.unwrap_or_default().as_u64() as i64;
                    let tx_index = log.transaction_index.map(|i| i.as_u64() as i64);
                    let sender = format!("0x{:x}", from);
                    let to_address = format!("0x{:x}", to);
                    let amount_raw = u256_to_string(amount);

                    insert_deposit(&pool, &tx_hash, block_number, tx_index, &sender, &to_address, &amount_raw, &format!("0x{:x}", usdc_addr)).await?;
                    // 以当前日志所在块更新进度
                    update_last_block(&pool, source, block_number).await?;
                }
            }
        }
        // 最后更新到 latest
        update_last_block(&pool, source, latest).await?;
    }

    // WebSocket provider 用于订阅新块
    let ws = Provider::<Ws>::connect(ws_url.clone()).await?;
    let ws = Arc::new(ws);
    log::info!("🔌 WebSocket 连接已建立，开始订阅新块");

    // 实时订阅 USDC 的日志，并在本地过滤 to == vault
    let mut log_stream = ws.subscribe_logs(&Filter::new().address(usdc_addr)).await?;
    while let Some(log) = log_stream.next().await {
        if log.topics.len() == 3 {
            let to_topic = log.topics[2];
            let to = Address::from_slice(&to_topic.as_bytes()[12..]);
            if to == vault_addr {
                let from_topic = log.topics[1];
                let from = Address::from_slice(&from_topic.as_bytes()[12..]);
                let amount = U256::from_big_endian(log.data.as_ref());

                let tx_hash = format!("0x{:x}", log.transaction_hash.unwrap_or_default());
                let block_number = log.block_number.unwrap_or_default().as_u64() as i64;
                let tx_index = log.transaction_index.map(|i| i.as_u64() as i64);
                let sender = format!("0x{:x}", from);
                let to_address = format!("0x{:x}", to);
                let amount_raw = u256_to_string(amount);

                insert_deposit(&pool, &tx_hash, block_number, tx_index, &sender, &to_address, &amount_raw, &format!("0x{:x}", usdc_addr)).await?;
                update_last_block(&pool, source, block_number).await?;
            }
        }
    }

    Ok(())
}