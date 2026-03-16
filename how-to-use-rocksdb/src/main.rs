use anyhow::Result;
use rocksdb::{
    ColumnFamilyDescriptor, DBWithThreadMode, IteratorMode, MultiThreaded, Options, WriteBatchWithTransaction,
};
use tempfile::TempDir;

/// 演示基本的打开数据库、CRUD 操作
fn demo_basic_crud(path: &std::path::Path) -> Result<()> {
    println!("=== 1. 基本 CRUD 操作 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);

    let db = rocksdb::DB::open(&opts, path)?;

    // Put - 写入数据
    db.put(b"name", b"Alice")?;
    db.put(b"age", b"30")?;
    db.put(b"city", b"Shanghai")?;
    println!("写入: name=Alice, age=30, city=Shanghai");

    // Get - 读取数据
    if let Some(value) = db.get(b"name")? {
        println!("读取: name={}", String::from_utf8_lossy(&value));
    }

    // Delete - 删除数据
    db.delete(b"age")?;
    println!("删除: age");

    // 验证删除
    let result = db.get(b"age")?;
    println!("读取已删除的 key: age={:?}", result);

    println!();
    Ok(())
}

/// 演示 WriteBatch 批量写入
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

    db.write(batch)?;
    println!("批量写入 5 条数据");

    // 验证写入结果
    for i in 0..5 {
        let key = format!("batch_key_{i}");
        if let Some(value) = db.get(key.as_bytes())? {
            println!("  {key} = {}", String::from_utf8_lossy(&value));
        }
    }

    println!();
    Ok(())
}

/// 演示迭代器遍历
fn demo_iterator(path: &std::path::Path) -> Result<()> {
    println!("=== 3. 迭代器遍历 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);

    let db = rocksdb::DB::open(&opts, path)?;

    // 写入一些数据
    for i in 0..5 {
        db.put(format!("iter_{i:03}").as_bytes(), format!("value_{i}").as_bytes())?;
    }

    // 正向遍历所有 KV 对
    println!("正向遍历:");
    let iter = db.iterator(IteratorMode::Start);
    for item in iter {
        let (key, value) = item?;
        println!(
            "  {} = {}",
            String::from_utf8_lossy(&key),
            String::from_utf8_lossy(&value)
        );
    }

    // 反向遍历
    println!("反向遍历:");
    let iter = db.iterator(IteratorMode::End);
    for item in iter {
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

/// 演示 Column Family（列族）
fn demo_column_families(path: &std::path::Path) -> Result<()> {
    println!("=== 4. Column Family 列族 ===\n");

    // 创建带有多个 Column Family 的数据库
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);

    let cf_user = ColumnFamilyDescriptor::new("users", Options::default());
    let cf_order = ColumnFamilyDescriptor::new("orders", Options::default());

    let db: DBWithThreadMode<MultiThreaded> =
        DBWithThreadMode::open_cf_descriptors(&opts, path, vec![cf_user, cf_order])?;

    // 向 "users" 列族写入数据
    let cf_users = db.cf_handle("users").expect("users CF not found");
    db.put_cf(&cf_users, b"user:1", b"Alice")?;
    db.put_cf(&cf_users, b"user:2", b"Bob")?;
    println!("写入 users CF: user:1=Alice, user:2=Bob");

    // 向 "orders" 列族写入数据
    let cf_orders = db.cf_handle("orders").expect("orders CF not found");
    db.put_cf(&cf_orders, b"order:1001", b"iPhone 16")?;
    db.put_cf(&cf_orders, b"order:1002", b"MacBook Pro")?;
    println!("写入 orders CF: order:1001=iPhone 16, order:1002=MacBook Pro");

    // 从不同列族读取数据
    if let Some(value) = db.get_cf(&cf_users, b"user:1")? {
        println!("从 users CF 读取: user:1={}", String::from_utf8_lossy(&value));
    }
    if let Some(value) = db.get_cf(&cf_orders, b"order:1001")? {
        println!(
            "从 orders CF 读取: order:1001={}",
            String::from_utf8_lossy(&value)
        );
    }

    // 遍历某个列族
    println!("遍历 users CF:");
    let iter = db.iterator_cf(&cf_users, IteratorMode::Start);
    for item in iter {
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

/// 演示前缀扫描
fn demo_prefix_scan(path: &std::path::Path) -> Result<()> {
    println!("=== 5. 前缀扫描 ===\n");

    let mut opts = Options::default();
    opts.create_if_missing(true);
    // 设置前缀提取器，提取前 4 个字节作为前缀
    opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(4));

    let db = rocksdb::DB::open(&opts, path)?;

    // 写入不同前缀的数据
    db.put(b"usr:alice", b"Alice Data")?;
    db.put(b"usr:bob", b"Bob Data")?;
    db.put(b"usr:charlie", b"Charlie Data")?;
    db.put(b"ord:1001", b"Order 1001")?;
    db.put(b"ord:1002", b"Order 1002")?;
    db.put(b"log:2024-01", b"January Log")?;
    println!("写入多种前缀的数据: usr:*, ord:*, log:*");

    // 按前缀 "usr:" 扫描
    println!("\n前缀扫描 'usr:':");
    let iter = db.prefix_iterator(b"usr:");
    for item in iter {
        let (key, value) = item?;
        let key_str = String::from_utf8_lossy(&key);
        // prefix_iterator 可能会返回前缀之外的数据，需要手动检查
        if !key_str.starts_with("usr:") {
            break;
        }
        println!("  {key_str} = {}", String::from_utf8_lossy(&value));
    }

    // 按前缀 "ord:" 扫描
    println!("前缀扫描 'ord:':");
    let iter = db.prefix_iterator(b"ord:");
    for item in iter {
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

fn main() -> Result<()> {
    println!("RocksDB 常见操作演示\n");

    // 使用临时目录，程序结束后自动清理
    let tmp_dir = TempDir::new()?;

    // 每个 demo 使用独立的子目录，避免冲突
    demo_basic_crud(&tmp_dir.path().join("basic"))?;
    demo_write_batch(&tmp_dir.path().join("batch"))?;
    demo_iterator(&tmp_dir.path().join("iter"))?;
    demo_column_families(&tmp_dir.path().join("cf"))?;
    demo_prefix_scan(&tmp_dir.path().join("prefix"))?;

    println!("所有演示完成！临时数据库目录已自动清理。");
    Ok(())
}
