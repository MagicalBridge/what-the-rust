use anyhow::Result;
use rocksdb::{
    checkpoint::Checkpoint, ColumnFamilyDescriptor, DBWithThreadMode, IteratorMode,
    MultiThreaded, Options, WriteBatchWithTransaction,
};
use tempfile::TempDir;

// ============================================================
// 1. 基本 CRUD
// ============================================================

fn demo_basic_crud(path: &std::path::Path) -> Result<()> {
    println!("=== 1. 基本 CRUD 操作 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);

    let db = rocksdb::DB::open(&opts, path)?;

    // Put
    db.put(b"name", b"Alice")?;
    db.put(b"age", b"30")?;
    db.put(b"city", b"Shanghai")?;
    println!("写入: name=Alice, age=30, city=Shanghai");

    // Get
    if let Some(value) = db.get(b"name")? {
        println!("读取: name={}", String::from_utf8_lossy(&value));
    }

    // Delete
    db.delete(b"age")?;
    println!("删除: age");

    // 验证删除
    let result = db.get(b"age")?;
    println!("读取已删除的 key: age={:?}", result);

    println!();
    Ok(())
}

// ============================================================
// 2. WriteBatch — 原子批量写入
// ============================================================

fn demo_write_batch(path: &std::path::Path) -> Result<()> {
    println!("=== 2. WriteBatch 批量写入 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);

    let db = rocksdb::DB::open(&opts, path)?;

    // 使用 WriteBatch 原子性地写入多个 KV 对
    let mut batch = WriteBatchWithTransaction::<false>::default();
    for i in 0..5 {
        let key = format!("batch_key_{i}");
        let value = format!("batch_value_{i}");
        batch.put(key.as_bytes(), value.as_bytes());
    }
    // 还可以在同一个 batch 中混合 put 和 delete
    batch.delete(b"batch_key_2");

    db.write(batch)?;
    println!("批量写入 5 条 + 删除 1 条（原子操作）");

    for i in 0..5 {
        let key = format!("batch_key_{i}");
        match db.get(key.as_bytes())? {
            Some(value) => println!("  {key} = {}", String::from_utf8_lossy(&value)),
            None => println!("  {key} = <deleted>"),
        }
    }

    println!();
    Ok(())
}

// ============================================================
// 3. Iterator — 遍历与范围扫描
// ============================================================

fn demo_iterator(path: &std::path::Path) -> Result<()> {
    println!("=== 3. 迭代器遍历 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);

    let db = rocksdb::DB::open(&opts, path)?;

    // 写入带前缀的数据
    for i in 0u32..5 {
        db.put(format!("order:{i:04}").as_bytes(), format!("order_data_{i}").as_bytes())?;
        db.put(
            format!("account:{i:04}").as_bytes(),
            format!("account_data_{i}").as_bytes(),
        )?;
    }

    // 正向遍历所有
    println!("正向遍历所有 KV:");
    for item in db.iterator(IteratorMode::Start) {
        let (key, value) = item?;
        println!(
            "  {} = {}",
            String::from_utf8_lossy(&key),
            String::from_utf8_lossy(&value)
        );
    }

    // 从指定位置开始扫描（seek）— 只取 order: 前缀
    println!("\n范围扫描 order:*:");
    let iter = db.iterator(IteratorMode::From(b"order:", rocksdb::Direction::Forward));
    for item in iter {
        let (key, _value) = item?;
        let key_str = String::from_utf8_lossy(&key);
        if !key_str.starts_with("order:") {
            break; // 超出前缀范围
        }
        println!("  {key_str}");
    }

    // Raw Iterator — seek_to_last 获取最大 key
    println!("\nRaw Iterator — 最后一条:");
    let mut raw_iter = db.raw_iterator();
    raw_iter.seek_to_last();
    if raw_iter.valid() {
        println!(
            "  {} = {}",
            String::from_utf8_lossy(raw_iter.key().unwrap()),
            String::from_utf8_lossy(raw_iter.value().unwrap())
        );
    }

    println!();
    Ok(())
}

// ============================================================
// 4. Column Family — 逻辑隔离
// ============================================================

fn demo_column_families(path: &std::path::Path) -> Result<()> {
    println!("=== 4. Column Family 列族 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);

    let cf_user = ColumnFamilyDescriptor::new("users", Options::default());
    let cf_order = ColumnFamilyDescriptor::new("orders", Options::default());

    let db: DBWithThreadMode<MultiThreaded> =
        DBWithThreadMode::open_cf_descriptors(&opts, path, vec![cf_user, cf_order])?;

    // 向不同 CF 写入
    let cf_users = db.cf_handle("users").expect("users CF not found");
    let cf_orders = db.cf_handle("orders").expect("orders CF not found");

    db.put_cf(&cf_users, b"user:1", b"Alice")?;
    db.put_cf(&cf_users, b"user:2", b"Bob")?;
    db.put_cf(&cf_orders, b"order:1001", b"iPhone 16")?;
    db.put_cf(&cf_orders, b"order:1002", b"MacBook Pro")?;

    println!("写入 users CF: user:1=Alice, user:2=Bob");
    println!("写入 orders CF: order:1001=iPhone 16, order:1002=MacBook Pro");

    // 跨 CF 的 WriteBatch — 原子性地修改多个 CF
    let mut batch = WriteBatchWithTransaction::<false>::default();
    batch.put_cf(&cf_users, b"user:3", b"Charlie");
    batch.put_cf(&cf_orders, b"order:1003", b"iPad Air");
    batch.delete_cf(&cf_orders, b"order:1001");
    db.write(batch)?;
    println!("\n跨 CF 原子操作: 添加 user:3 + order:1003, 删除 order:1001");

    // 遍历各 CF
    println!("\n遍历 users CF:");
    for item in db.iterator_cf(&cf_users, IteratorMode::Start) {
        let (key, value) = item?;
        println!(
            "  {} = {}",
            String::from_utf8_lossy(&key),
            String::from_utf8_lossy(&value)
        );
    }

    println!("遍历 orders CF:");
    for item in db.iterator_cf(&cf_orders, IteratorMode::Start) {
        let (key, value) = item?;
        println!(
            "  {} = {}",
            String::from_utf8_lossy(&key),
            String::from_utf8_lossy(&value)
        );
    }

    println!();
    Ok(())
}

// ============================================================
// 5. 前缀扫描（带 Prefix Extractor）
// ============================================================

fn demo_prefix_scan(path: &std::path::Path) -> Result<()> {
    println!("=== 5. 前缀扫描 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);
    // 设置前缀提取器，提取前 4 个字节作为前缀
    opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(4));

    let db = rocksdb::DB::open(&opts, path)?;

    db.put(b"usr:alice", b"Alice Data")?;
    db.put(b"usr:bob", b"Bob Data")?;
    db.put(b"usr:charlie", b"Charlie Data")?;
    db.put(b"ord:1001", b"Order 1001")?;
    db.put(b"ord:1002", b"Order 1002")?;
    println!("写入: usr:alice, usr:bob, usr:charlie, ord:1001, ord:1002");

    // prefix_iterator 利用 Bloom Filter 跳过不包含前缀的 SST
    println!("\n前缀扫描 'usr:':");
    for item in db.prefix_iterator(b"usr:") {
        let (key, value) = item?;
        let key_str = String::from_utf8_lossy(&key);
        if !key_str.starts_with("usr:") {
            break;
        }
        println!("  {key_str} = {}", String::from_utf8_lossy(&value));
    }

    println!("前缀扫描 'ord:':");
    for item in db.prefix_iterator(b"ord:") {
        let (key, value) = item?;
        let key_str = String::from_utf8_lossy(&key);
        if !key_str.starts_with("ord:") {
            break;
        }
        println!("  {key_str} = {}", String::from_utf8_lossy(&value));
    }

    println!();
    Ok(())
}

// ============================================================
// 6. Checkpoint — 零开销快照
// ============================================================

fn demo_checkpoint(path: &std::path::Path) -> Result<()> {
    println!("=== 6. Checkpoint 快照 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);

    let db = rocksdb::DB::open(&opts, path)?;

    // 写入一些数据
    db.put(b"block_height", b"1000")?;
    db.put(b"account:alice", b"balance:500")?;
    db.put(b"account:bob", b"balance:300")?;
    println!("写入初始数据: block_height=1000, alice=500, bob=300");

    // 创建 checkpoint（硬链接 SST 文件，毫秒级）
    let snapshot_path = path.parent().unwrap().join("snapshot_v1");
    let checkpoint = Checkpoint::new(&db)?;
    checkpoint.create_checkpoint(&snapshot_path)?;
    println!("创建 Checkpoint: {:?}", snapshot_path);

    // 原库继续修改
    db.put(b"block_height", b"1001")?;
    db.put(b"account:alice", b"balance:400")?;
    println!("\n原库修改: block_height=1001, alice=400");

    // 打开快照 — 数据仍是快照时刻的
    let snapshot_db = rocksdb::DB::open_for_read_only(&opts, &snapshot_path, false)?;

    println!("\n原库当前值:");
    println!(
        "  block_height = {}",
        String::from_utf8_lossy(&db.get(b"block_height")?.unwrap())
    );
    println!(
        "  account:alice = {}",
        String::from_utf8_lossy(&db.get(b"account:alice")?.unwrap())
    );

    println!("快照值（保持不变）:");
    println!(
        "  block_height = {}",
        String::from_utf8_lossy(&snapshot_db.get(b"block_height")?.unwrap())
    );
    println!(
        "  account:alice = {}",
        String::from_utf8_lossy(&snapshot_db.get(b"account:alice")?.unwrap())
    );

    // 必须先关闭 snapshot_db，再清理目录
    drop(snapshot_db);
    // 清理快照目录
    std::fs::remove_dir_all(&snapshot_path)?;

    println!();
    Ok(())
}

// ============================================================
// 7. Key 设计模式 — 前缀字节 + 大端编码
// ============================================================

/// Key 前缀枚举 — 实际项目中用于区分数据类型
#[repr(u8)]
enum KeyPrefix {
    Account = 0,
    Order = 1,
    Position = 2,
    Config = 3,
}

fn make_account_key(address: &[u8; 20]) -> Vec<u8> {
    let mut key = Vec::with_capacity(21);
    key.push(KeyPrefix::Account as u8);
    key.extend_from_slice(address);
    key // [0x00][address: 20 bytes] = 21 bytes
}

fn make_order_key(order_id: u64) -> Vec<u8> {
    let mut key = Vec::with_capacity(9);
    key.push(KeyPrefix::Order as u8);
    key.extend_from_slice(&order_id.to_be_bytes()); // 大端编码！
    key // [0x01][order_id: 8 bytes BE] = 9 bytes
}

fn make_position_key(user: &[u8; 20], instrument_id: u64) -> Vec<u8> {
    let mut key = Vec::with_capacity(29);
    key.push(KeyPrefix::Position as u8);
    key.extend_from_slice(user);
    key.extend_from_slice(&instrument_id.to_be_bytes());
    key // [0x02][user: 20][instrument_id: 8 BE] = 29 bytes
}

fn make_config_key() -> Vec<u8> {
    vec![KeyPrefix::Config as u8] // 单例 key，仅 1 字节
}

fn demo_key_design(path: &std::path::Path) -> Result<()> {
    println!("=== 7. Key 设计模式 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);

    let db = rocksdb::DB::open(&opts, path)?;

    // 模拟地址
    let alice_addr: [u8; 20] = [0xAA; 20];
    let bob_addr: [u8; 20] = [0xBB; 20];

    // 写入不同类型的数据
    db.put(&make_account_key(&alice_addr), b"alice_account_data")?;
    db.put(&make_account_key(&bob_addr), b"bob_account_data")?;
    db.put(&make_order_key(1), b"order_1")?;
    db.put(&make_order_key(2), b"order_2")?;
    db.put(&make_order_key(256), b"order_256")?; // 测试大端排序
    db.put(&make_position_key(&alice_addr, 100), b"alice_pos_100")?;
    db.put(&make_position_key(&alice_addr, 200), b"alice_pos_200")?;
    db.put(&make_config_key(), b"global_config")?;

    println!("写入: 2 accounts, 3 orders, 2 positions, 1 config\n");

    // 遍历所有 key，观察排序
    println!("所有 key（按字典序）:");
    for item in db.iterator(IteratorMode::Start) {
        let (key, value) = item?;
        let prefix = key[0];
        let type_name = match prefix {
            0 => "Account ",
            1 => "Order   ",
            2 => "Position",
            3 => "Config  ",
            _ => "Unknown ",
        };
        println!(
            "  [{type_name}] key={} value={}",
            hex::encode(&key),
            String::from_utf8_lossy(&value)
        );
    }

    // 演示大端编码保证排序正确
    println!("\n验证 Order 排序（大端编码）:");
    let iter = db.iterator(IteratorMode::From(
        &[KeyPrefix::Order as u8],
        rocksdb::Direction::Forward,
    ));
    for item in iter {
        let (key, value) = item?;
        if key[0] != KeyPrefix::Order as u8 {
            break;
        }
        let order_id = u64::from_be_bytes(key[1..9].try_into().unwrap());
        println!(
            "  order_id={order_id} (hex key={}) value={}",
            hex::encode(&key),
            String::from_utf8_lossy(&value)
        );
    }

    // 演示前缀扫描某用户的所有仓位
    println!("\n前缀扫描 alice 的所有 Position:");
    let prefix = {
        let mut p = Vec::with_capacity(21);
        p.push(KeyPrefix::Position as u8);
        p.extend_from_slice(&alice_addr);
        p
    };
    let iter = db.iterator(IteratorMode::From(&prefix, rocksdb::Direction::Forward));
    for item in iter {
        let (key, value) = item?;
        if !key.starts_with(&prefix) {
            break;
        }
        let instrument_id = u64::from_be_bytes(key[21..29].try_into().unwrap());
        println!(
            "  instrument_id={instrument_id} value={}",
            String::from_utf8_lossy(&value)
        );
    }

    println!();
    Ok(())
}

// ============================================================
// 8. 性能调优配置演示
// ============================================================

fn demo_tuning(path: &std::path::Path) -> Result<()> {
    println!("=== 8. 性能调优配置 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);

    // --- 写入调优 ---
    opts.set_write_buffer_size(128 * 1024 * 1024); // MemTable 128MB（默认 64MB）
    opts.set_max_write_buffer_number(4); // 最多 4 个 Immutable MemTable
    opts.set_min_write_buffer_number_to_merge(2); // 至少 2 个才触发合并
    println!("写入调优: write_buffer=128MB, max_write_buffer=4, min_merge=2");

    // --- 读取调优 ---
    let cache = rocksdb::Cache::new_lru_cache(64 * 1024 * 1024); // 64MB Block Cache
    let mut block_opts = rocksdb::BlockBasedOptions::default();
    block_opts.set_block_cache(&cache);
    block_opts.set_block_size(16 * 1024); // 16KB block（默认 4KB）
    block_opts.set_bloom_filter(10.0, false); // Bloom Filter, 10 bits/key
    opts.set_block_based_table_factory(&block_opts);
    println!("读取调优: block_cache=64MB, block_size=16KB, bloom_filter=10bits/key");

    // --- 压缩配置 ---
    opts.set_compression_per_level(&[
        rocksdb::DBCompressionType::None, // L0: 不压缩（减少写入延迟）
        rocksdb::DBCompressionType::Lz4,  // L1: LZ4（快速）
        rocksdb::DBCompressionType::Zstd, // L2+: Zstd（高压缩比）
        rocksdb::DBCompressionType::Zstd,
        rocksdb::DBCompressionType::Zstd,
        rocksdb::DBCompressionType::Zstd,
        rocksdb::DBCompressionType::Zstd,
    ]);
    println!("压缩: L0=None, L1=LZ4, L2+=Zstd");

    // --- Compaction 调优 ---
    opts.set_level_compaction_dynamic_level_bytes(true);
    opts.set_level_zero_file_num_compaction_trigger(4);
    opts.set_level_zero_slowdown_writes_trigger(20);
    opts.set_level_zero_stop_writes_trigger(36);
    println!("Compaction: dynamic_level=true, trigger=4, slowdown=20, stop=36");

    let db = rocksdb::DB::open(&opts, path)?;

    // 写入测试数据
    let start = std::time::Instant::now();
    let mut batch = WriteBatchWithTransaction::<false>::default();
    for i in 0u64..10_000 {
        let key = i.to_be_bytes();
        let value = format!("value_{i:06}");
        batch.put(&key, value.as_bytes());
    }
    db.write(batch)?;
    let elapsed = start.elapsed();
    println!("\n写入 10,000 条数据: {:?}", elapsed);

    // 读取测试
    let start = std::time::Instant::now();
    for i in 0u64..10_000 {
        let _ = db.get(&i.to_be_bytes())?;
    }
    let elapsed = start.elapsed();
    println!("顺序读取 10,000 条数据: {:?}", elapsed);

    // 随机读取测试
    let start = std::time::Instant::now();
    for _ in 0..10_000 {
        let random_key = (rand_u64() % 10_000).to_be_bytes();
        let _ = db.get(&random_key)?;
    }
    let elapsed = start.elapsed();
    println!("随机读取 10,000 条数据: {:?}", elapsed);

    println!();
    Ok(())
}

/// 简易伪随机（避免引入 rand 依赖）
fn rand_u64() -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;
    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    std::thread::current().id().hash(&mut hasher);
    hasher.finish()
}

// ============================================================
// 9. Read-Only 模式
// ============================================================

fn demo_read_only(path: &std::path::Path) -> Result<()> {
    println!("=== 9. Read-Only 模式 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);

    // 先写入数据
    {
        let db = rocksdb::DB::open(&opts, path)?;
        db.put(b"key1", b"value1")?;
        db.put(b"key2", b"value2")?;
        db.put(b"key3", b"value3")?;
        // db 在这里 drop，释放锁
    }

    // 以只读模式打开 — 不抢写锁
    let ro_db = rocksdb::DB::open_for_read_only(&opts, path, false)?;

    println!("只读模式下读取:");
    for item in ro_db.iterator(IteratorMode::Start) {
        let (key, value) = item?;
        println!(
            "  {} = {}",
            String::from_utf8_lossy(&key),
            String::from_utf8_lossy(&value)
        );
    }

    // 尝试写入会编译失败（open_for_read_only 返回的类型没有 put 方法）
    // ro_db.put(b"key4", b"value4")?;  // 编译错误！

    println!("只读模式无法写入（编译期保证）\n");

    Ok(())
}

// ============================================================
// main
// ============================================================

fn main() -> Result<()> {
    println!("╔══════════════════════════════════════╗");
    println!("║   RocksDB 完整操作演示                ║");
    println!("╚══════════════════════════════════════╝\n");

    let tmp_dir = TempDir::new()?;

    demo_basic_crud(&tmp_dir.path().join("basic"))?;
    demo_write_batch(&tmp_dir.path().join("batch"))?;
    demo_iterator(&tmp_dir.path().join("iter"))?;
    demo_column_families(&tmp_dir.path().join("cf"))?;
    demo_prefix_scan(&tmp_dir.path().join("prefix"))?;
    demo_checkpoint(&tmp_dir.path().join("ckpt"))?;
    demo_key_design(&tmp_dir.path().join("key_design"))?;
    demo_tuning(&tmp_dir.path().join("tuning"))?;
    demo_read_only(&tmp_dir.path().join("readonly"))?;

    println!("所有演示完成！临时数据库目录已自动清理。");
    Ok(())
}
