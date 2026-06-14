pub fn basic_variables() {
    println!("1. 基本变量声明:");

    // 整数类型
    let x: i32 = 42; // 有符号整数
    let y: u32 = 100; // 无符号整数
    let z = 200; // 类型推断，默认是 i32

    println!("   x = {} (i32)", x);
    println!("   y = {} (u32)", y);
    println!("   z = {} (推断为 i32)", z);

    // 浮点数
    let pi: f64 = 3.14159;
    let e = 2.71828; // 推断为 f64

    println!("   pi = {} (f64)", pi);
    println!("   e = {} (f64)", e);

    // 布尔值
    let is_rust_awesome = true;
    let is_learning = false;

    println!("   is_rust_awesome = {}", is_rust_awesome);
    println!("   is_learning = {}", is_learning);

    // 字符 单个字符 只能用单引号
    let heart = '❤';
    let rust_logo = '🦀';

    println!("   heart = {}", heart);
    println!("   rust_logo = {}", rust_logo);
    println!();
}
