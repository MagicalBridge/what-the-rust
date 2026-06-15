pub fn mutability_demo() {
    println!("2. 可变性演示:");

    // 默认不可变
    let x: i32 = 5;
    println!("   初始值: x = {}", x);

    // 可变变量
    let mut y: i32 = 10;
    println!("   初始值: y = {}", y);

    y = 15; // 可以修改
    println!("   修改后: y = {}", y);

    // 尝试修改不可变变量会编译错误
    // x = 6; // 这行会报错

    println!("   注意: 默认变量是不可变的，需要 mut 关键字才能修改");
    println!();
}
