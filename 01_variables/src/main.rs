mod basic_variables;

fn main() {
    println!("=== Rust 变量学习项目 ===\n");

    // 1. 基本变量声明
    basic_variables::basic_variables();
    
    // 2. 可变性演示
    mutability_demo();
    
    // 3. 变量作用域
    scope_demo();
    
    // 4. 类型推断
    type_inference();
    
    // 5. 常量声明
    constants_demo();
    
    // 6. 静态变量
    static_variables();
    
    // 7. 变量遮蔽(Shadowing)
    shadowing_demo();
    
    // 8. 类型转换
    type_conversion();
    
    // 9. 元组和数组
    tuples_and_arrays();
    
    // 10. 字符串类型
    string_types();
}

// 2. 可变性演示
fn mutability_demo() {
    println!("2. 可变性演示:");
    
    // 默认不可变
    let x = 5;
    println!("   初始值: x = {}", x);
    
    // 可变变量
    let mut y = 10;
    println!("   初始值: y = {}", y);
    
    y = 15; // 可以修改
    println!("   修改后: y = {}", y);
    
    // 尝试修改不可变变量会编译错误
    // x = 6; // 这行会报错
    
    println!("   注意: 默认变量是不可变的，需要 mut 关键字才能修改");
    println!();
}

// 3. 变量作用域
fn scope_demo() {
    println!("3. 变量作用域:");
    
    let outer_var = 100;
    println!("   外层变量: outer_var = {}", outer_var);
    
    {
        let inner_var = 200;
        println!("   内层变量: inner_var = {}", inner_var);
        println!("   内层可以访问外层: outer_var = {}", outer_var);
    }
    
    // inner_var 在这里已经不存在了
    println!("   外层变量仍然存在: outer_var = {}", outer_var);
    println!();
}

// 4. 类型推断
fn type_inference() {
    println!("4. 类型推断:");
    
    // Rust 编译器会根据使用方式推断类型
    let x = 5; // 推断为 i32
    let y = 5.0; // 推断为 f64
    let z = true; // 推断为 bool
    
    println!("   x = {} (类型: i32)", x);
    println!("   y = {} (类型: f64)", y);
    println!("   z = {} (类型: bool)", z);
    
    // 可以通过使用来强制类型推断
    let mut vec = Vec::new(); // 类型未知
    vec.push(1); // 现在推断为 Vec<i32>
    vec.push(2);
    vec.push(3);
    
    println!("   vec = {:?} (类型: Vec<i32>)", vec);
    println!();
}

// 5. 常量声明
fn constants_demo() {
    println!("5. 常量声明:");
    
    // 常量必须指定类型
    const MAX_POINTS: u32 = 100_000;
    const PI: f64 = 3.14159265359;
    const PROGRAM_NAME: &str = "Rust Variables Demo";
    
    println!("   MAX_POINTS = {}", MAX_POINTS);
    println!("   PI = {}", PI);
    println!("   PROGRAM_NAME = {}", PROGRAM_NAME);
    
    // 常量可以在任何作用域中使用
    {
        const LOCAL_CONST: i32 = 42;
        println!("   局部常量: LOCAL_CONST = {}", LOCAL_CONST);
    }
    println!();
}

// 6. 静态变量
fn static_variables() {
    println!("6. 静态变量:");
    
    // 静态变量在整个程序运行期间都存在
    static mut COUNTER: u32 = 0;
    static PROGRAM_VERSION: &str = "1.0.0";
    
    println!("   PROGRAM_VERSION = {}", PROGRAM_VERSION);
    
    // 修改静态变量需要 unsafe 块
    unsafe {
        COUNTER += 1;
        let counter_value = COUNTER;
        println!("   计数器: COUNTER = {}", counter_value);
        
        COUNTER += 1;
        let counter_value = COUNTER;
        println!("   计数器: COUNTER = {}", counter_value);
    }
    println!();
}

// 7. 变量遮蔽(Shadowing)
fn shadowing_demo() {
    println!("7. 变量遮蔽(Shadowing):");
    
    let x = 5;
    println!("   初始值: x = {}", x);
    
    {
        let x = x * 2; // 遮蔽外层变量
        println!("   内层遮蔽: x = {}", x);
        
        let x = x + 1; // 再次遮蔽
        println!("   再次遮蔽: x = {}", x);
    }
    
    println!("   外层变量不变: x = {}", x);
    
    // 遮蔽可以改变类型
    let spaces = "   ";
    let spaces = spaces.len(); // 从字符串变为数字
    
    println!("   遮蔽改变类型: spaces = {} (从字符串变为 usize)", spaces);
    println!();
}

// 8. 类型转换
fn type_conversion() {
    println!("8. 类型转换:");
    
    // 整数类型转换
    let x: i32 = 42;
    let y: u32 = x as u32;
    let z: f64 = x as f64;
    
    println!("   i32: x = {}", x);
    println!("   转换为 u32: y = {}", y);
    println!("   转换为 f64: z = {}", z);
    
    // 浮点数转换
    let pi = 3.14159;
    let pi_int = pi as i32;
    println!("   f64: pi = {}", pi);
    println!("   转换为 i32: pi_int = {} (小数部分被截断)", pi_int);
    
    // 字符转换
    let char_a = 'A';
    let ascii_a = char_a as u8;
    println!("   字符 'A' 的 ASCII 码: {}", ascii_a);
    
    // 布尔值转换
    let true_val = true;
    let false_val = false;
    println!("   true 转换为数字: {}", true_val as i32);
    println!("   false 转换为数字: {}", false_val as i32);
    println!();
}

// 9. 元组和数组
fn tuples_and_arrays() {
    println!("9. 元组和数组:");
    
    // 元组 元素数量固定，类型可以不同
    let tuple: (i32, f64, &str) = (500, 6.4, "hello");
    println!("   元组: {:?}", tuple);
    println!("   元组第一个元素: {}", tuple.0);
    println!("   元组第二个元素: {}", tuple.1);
    println!("   元组第三个元素: {}", tuple.2);
    
    // 解构元组
    let (x, y, z) = tuple;
    println!("   解构后: x={}, y={}, z={}", x, y, z);
    
    // 数组 元素数量固定，类型相同
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("   数组: {:?}", arr);
    println!("   数组第一个元素: {}", arr[0]);
    println!("   数组长度: {}", arr.len());
    
    // 数组初始化
    let zeros = [0; 10];
    println!("   全零数组: {:?}", zeros);
    println!();
}

// 10. 字符串类型
fn string_types() {
    println!("10. 字符串类型:");
    
    // 字符串字面量 (&str)
    let str_literal = "Hello, Rust!";
    println!("   字符串字面量: {}", str_literal);
    
    // String 类型
    let mut string = String::from("Hello");
    string.push_str(", World!");
    println!("   String 类型: {}", string);
    
    // 字符串切片
    let slice = &string[0..5];
    println!("   字符串切片 [0..5]: {}", slice);
    
    // 字符串连接
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let combined = s1 + " " + &s2;
    println!("   字符串连接: {}", combined);
    
    // 格式化字符串
    let name = "Rust";
    let version = 1.0;
    let formatted = format!("{} 版本 {}", name, version);
    println!("   格式化字符串: {}", formatted);
    println!();
    
    println!("=== 学习完成！===");
    println!("这些示例涵盖了 Rust 中变量的主要概念。");
    println!("你可以运行 'cargo run' 来查看所有示例的执行结果。");
}
