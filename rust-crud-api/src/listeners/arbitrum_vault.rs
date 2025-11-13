use crate::{Config, database::DatabasePool};
use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use std::time::Duration;
use tokio::time;

/// 获取ERC20 Transfer事件的签名哈希
/// Transfer(address indexed from, address indexed to, uint256 value)
fn get_transfer_event_signature() -> H256 {
    use ethers::utils::keccak256;
    H256::from(keccak256("Transfer(address,address,uint256)"))
}

/// 将 U256 格式化为十进制字符串（wei）
fn u256_to_string(v: U256) -> String { format!("{}", v) }

/// 处理Transfer事件日志，如果是向vault的转账则插入数据库
async fn process_transfer_log(
    pool: &DatabasePool,
    log: &Log,
    vault_addr: Address,
    usdc_addr: Address,
    source: &str,
) -> anyhow::Result<bool> {
    // 验证是否为标准 Transfer 事件
    if log.topics.len() != 3 || log.topics[0] != get_transfer_event_signature() {
        return Ok(false);
    }

    // 主题2为 to 地址（indexed）
    let to_topic = log.topics[2];
    let to = Address::from_slice(&to_topic.as_bytes()[12..]);
    
    // 只处理向vault的转账
    if to != vault_addr {
        return Ok(false);
    }

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

    insert_deposit(pool, &tx_hash, block_number, tx_index, &sender, &to_address, &amount_raw, &format!("0x{:x}", usdc_addr)).await?;
    update_last_block(pool, source, block_number).await?;
    
    Ok(true)
}

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
/// - 轮询新区块：定期检查新区块中的USDC转账事件
pub async fn start_vault_watcher(config: Config, pool: DatabasePool) -> anyhow::Result<()> {
    if !config.enable_vault_watcher {
        log::info!("🔕 Vault 监听已禁用（ENABLE_VAULT_WATCHER=false）");
        return Ok(());
    }

    let http_url = match &config.arbitrum_http_url { 
        Some(u) => u.clone(), 
        None => anyhow::bail!("ARBITRUM_HTTP_URL 未设置") 
    };
    let vault_addr_str = match &config.vault_contract_address { 
        Some(a) => a.clone(), 
        None => anyhow::bail!("VAULT_CONTRACT_ADDRESS 未设置") 
    };
    let usdc_addr_str = match &config.usdc_token_address { 
        Some(a) => a.clone(), 
        None => anyhow::bail!("USDC_TOKEN_ADDRESS 未设置") 
    };

    let vault_addr: Address = vault_addr_str.parse()?;
    let usdc_addr: Address = usdc_addr_str.parse()?;
    let source = "arbitrum_vault";

    // HTTP provider
    let http = Provider::<Http>::try_from(http_url.clone())?;
    let latest = http.get_block_number().await?.as_u64() as i64;

    // 选择起始区块：优先用进度表，否则使用配置的起始块，最后回退到最新往前回溯 10 个块
    let start_block = match get_last_block(&pool, source).await? {
        Some(last) => last + 1,
        None => {
            // 如果配置了起始块高度，使用配置值；否则从最新往前回溯 10 个块
            match config.vault_start_block {
                Some(configured_start) => {
                    log::info!("🎯 使用配置的起始块高度: {}", configured_start);
                    configured_start as i64
                }
                None => {
                    log::info!("📅 未配置起始块高度，从最新块往前回溯 10 个块");
                    (latest - 10).max(0)
                }
            }
        }
    };

    if start_block <= latest {
        log::info!("📦 开始补扫 USDC Transfer 事件: {} -> {}", start_block, latest);
        
        // 分批处理，每批最多10个区块（Alchemy免费计划限制）
        let mut current_block = start_block;
        while current_block <= latest {
            let end_block = (current_block + 9).min(latest);
            log::info!("🔍 处理区块范围: {} -> {}", current_block, end_block);
            
            // 通过 get_logs 拉取 USDC 的 Transfer 事件，再过滤 to == vault
            let filter = Filter::new()
                .address(usdc_addr)
                .from_block(current_block as u64)
                .to_block(end_block as u64);

            match http.get_logs(&filter).await {
                Ok(logs) => {
                    for log in logs {
                        if process_transfer_log(&pool, &log, vault_addr, usdc_addr, source).await? {
                            let tx_hash = format!("0x{:x}", log.transaction_hash.unwrap_or_default());
                            let from_topic = log.topics[1];
                            let from = Address::from_slice(&from_topic.as_bytes()[12..]);
                            let to_topic = log.topics[2];
                            let to = Address::from_slice(&to_topic.as_bytes()[12..]);
                            let amount = U256::from_big_endian(log.data.as_ref());
                            let amount_raw = u256_to_string(amount);
                            let sender = format!("0x{:x}", from);
                            let to_address = format!("0x{:x}", to);
                            
                            log::info!("💰 检测到入金: {} -> {} amount: {} USDC (tx: {})", sender, to_address, amount_raw, tx_hash);
                        }
                    }
                }
                Err(e) => {
                    log::error!("❌ 获取日志失败 (区块范围 {} -> {}): {}", current_block, end_block, e);
                    // 继续处理下一批，不中断整个流程
                }
            }
            
            current_block = end_block + 1;
        }
        
        // 最后更新到 latest
        update_last_block(&pool, source, latest).await?;
    }

    // 使用HTTP轮询新区块
    log::info!("🔄 开始轮询新区块，每5秒检查一次");
    let mut interval = time::interval(Duration::from_secs(5));
    
    loop {
        interval.tick().await;
        
        // 获取最新区块号
        let current_latest = match http.get_block_number().await {
            Ok(block_number) => block_number.as_u64() as i64,
            Err(e) => {
                log::error!("❌ 获取最新区块号失败: {}", e);
                continue;
            }
        };
        
        // 获取上次处理的区块号
        let last_processed = match get_last_block(&pool, source).await {
            Ok(Some(last)) => last,
            Ok(None) => {
                log::error!("❌ 无法获取上次处理的区块号");
                continue;
            }
            Err(e) => {
                log::error!("❌ 查询上次处理的区块号失败: {}", e);
                continue;
            }
        };
        
        // 如果有新区块，检查其中的USDC转账事件
        if current_latest > last_processed {
            log::info!("🔍 检查新区块: {} -> {}", last_processed + 1, current_latest);
            
            // 分批处理，每批最多10个区块（Alchemy免费计划限制）
            let mut current_block = last_processed + 1;
            let mut last_successfully_processed = last_processed;
            
            while current_block <= current_latest {
                let end_block = (current_block + 9).min(current_latest);
                
                let filter = Filter::new()
                    .address(usdc_addr)
                    .from_block(current_block as u64)
                    .to_block(end_block as u64);
                
                match http.get_logs(&filter).await {
                    Ok(logs) => {
                        for log in logs {
                            if process_transfer_log(&pool, &log, vault_addr, usdc_addr, source).await? {
                                let tx_hash = format!("0x{:x}", log.transaction_hash.unwrap_or_default());
                                let from_topic = log.topics[1];
                                let from = Address::from_slice(&from_topic.as_bytes()[12..]);
                                let to_topic = log.topics[2];
                                let to = Address::from_slice(&to_topic.as_bytes()[12..]);
                                let amount = U256::from_big_endian(log.data.as_ref());
                                let amount_raw = u256_to_string(amount);
                                let sender = format!("0x{:x}", from);
                                let to_address = format!("0x{:x}", to);
                                
                                log::info!("🔔 实时检测到入金: {} -> {} amount: {} USDC (tx: {})", sender, to_address, amount_raw, tx_hash);
                            }
                        }
                        // 更新成功处理的区块
                        last_successfully_processed = end_block;
                    }
                    Err(e) => {
                        log::error!("❌ 获取日志失败 (区块范围 {} -> {}): {}", current_block, end_block, e);
                        // 如果失败，停止处理后续批次，避免跳过区块
                        break;
                    }
                }
                
                current_block = end_block + 1;
            }
            
            // 更新已处理的区块号
            if last_successfully_processed > last_processed {
                if let Err(e) = update_last_block(&pool, source, last_successfully_processed).await {
                    log::error!("❌ 更新区块进度失败: {}", e);
                }
            }
        }
    }
}