// 测试 dotenvy 的三种使用方式

// 方式1：完整路径（不需要导入）
fn test_full_path() {
    dotenvy::dotenv().ok();
    println!("方式1：完整路径访问成功");
}

// 方式2：导入后使用
use dotenvy::dotenv;
fn test_with_import() {
    dotenv().ok();
    println!("方式2：导入后使用成功");
}

// 方式3：导入所有内容
use dotenvy::*;
fn test_with_star_import() {
    dotenv().ok();
    println!("方式3：星号导入成功");
}

fn main() {
    test_full_path();
    test_with_import();
    test_with_star_import();
}