fn main() {
    println!("=== Rust 流程控制学习项目 ===\n");

    // 1. 条件语句 (if-else)
    conditional_statements();
    
    // 2. 循环语句 (loop)
    loop_statements();
    
    // 3. 条件循环 (while)
    while_loops();
    
    // 4. for 循环
    for_loops();
    
    // 5. 模式匹配 (match)
    pattern_matching();
    
    // 6. if let 语句
    if_let_statements();
    
    // 7. 嵌套控制结构
    nested_control_structures();
    
    // 8. 错误处理控制流
    error_handling_control_flow();
    
    // 9. 迭代器和闭包控制流
    iterator_closure_control_flow();
    
    // 10. 高级控制流模式
    advanced_control_flow_patterns();
    
    // 总结
    main_end();
}

// 1. 条件语句 (if-else)
fn conditional_statements() {
    println!("1. 条件语句 (if-else):");
    
    // 基本 if 语句
    let number = 5;
    println!("   测试数字: {}", number);
    
    if number > 0 {
        println!("   数字是正数");
    }
    
    if number < 0 {
        println!("   数字是负数");
    } else {
        println!("   数字是非负数");
    }
    
    // if-else-if 链式条件
    let score = 85;
    println!("   考试分数: {}", score);
    
    if score >= 90 {
        println!("   等级：优秀");
    } else if score >= 80 {
        println!("   等级：良好");
    } else if score >= 70 {
        println!("   等级：中等");
    } else if score >= 60 {
        println!("   等级：及格");
    } else {
        println!("   等级：不及格");
    }
    
    // if 表达式返回值
    let condition = true;
    let result = if condition { "真" } else { "假" };
    println!("   条件结果: {}", result);
    
    // 复杂条件表达式
    let x = 10;
    let y = 20;
    if x > 5 && y < 30 {
        println!("   复合条件为真: x({}) > 5 且 y({}) < 30", x, y);
    }
    
    // 三元表达式风格
    let age = 18;
    let status = if age >= 18 { "成年人" } else { "未成年人" };
    println!("   年龄 {} 岁，身份: {}", age, status);
    
    println!();
}

// 2. 循环语句 (loop)
fn loop_statements() {
    println!("2. 循环语句 (loop):");
    
    // 基本无限循环（用 break 退出）
    let mut counter = 0;
    println!("   基本 loop 循环:");
    loop {
        counter += 1;
        println!("     计数: {}", counter);
        
        if counter >= 5 {
            println!("     达到5次，退出循环");
            break;
        }
    }
    
    // 使用 continue 跳过迭代
    let mut skip_counter = 0;
    println!("   带 continue 的 loop 循环:");
    loop {
        skip_counter += 1;
        
        if skip_counter > 10 {
            break;
        }
        
        if skip_counter % 2 == 0 {
            continue; // 跳过偶数
        }
        
        println!("     奇数: {}", skip_counter);
    }
    
    // loop 表达式返回值
    let mut return_counter = 0;
    let result = loop {
        return_counter += 1;
        
        if return_counter == 3 {
            break return_counter * 10; // 返回值
        }
    };
    println!("   loop 返回值: {}", result);
    
    println!();
}

// 3. 条件循环 (while)
fn while_loops() {
    println!("3. 条件循环 (while):");
    
    // 基本 while 循环
    let mut number = 5;
    println!("   倒计时:");
    while number > 0 {
        println!("     {}!", number);
        number -= 1;
    }
    println!("     发射! 🚀");
    
    // while let 模式匹配循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);
    
    println!("   while let 弹出栈元素:");
    while let Some(top) = stack.pop() {
        println!("     弹出: {}", top);
    }
    
    println!();
}

// 4. for 循环
fn for_loops() {
    println!("4. for 循环:");
    
    // 遍历数组
    let arr = [10, 20, 30, 40, 50];
    println!("   遍历数组:");
    for element in arr {
        println!("     值: {}", element);
    }
    
    // 使用范围 (range)
    println!("   使用范围 1..6:");
    for number in 1..6 {
        println!("     数字: {}", number);
    }
    
    // enumerate 获取索引
    let colors = ["红色", "绿色", "蓝色"];
    println!("   enumerate 获取索引:");
    for (index, color) in colors.iter().enumerate() {
        println!("     索引 {}: {}", index, color);
    }
    
    // 反向迭代
    println!("   反向迭代 1..=5:");
    for number in (1..=5).rev() {
        println!("     数字: {}", number);
    }
    
    println!();
}

// 交通灯枚举示例
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 5. 模式匹配 (match)
fn pattern_matching() {
    println!("5. 模式匹配 (match):");
    
    // 基本 match 表达式
    let number = 13;
    println!("   匹配数字 {}:", number);
    match number {
        1 => println!("     一"),
        2 | 3 | 5 | 7 | 11 => println!("     这是一个小于20的质数"),
        13..=19 => println!("     13到19之间的数字"),
        _ => println!("     其他数字"),
    }
    
    // 匹配枚举
    let messages = [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];
    println!("   匹配交通灯:");
    for message in messages {
        match message {
            TrafficLight::Red => println!("     停止！"),
            TrafficLight::Yellow => println!("     准备！"),
            TrafficLight::Green => println!("     通行！"),
        }
    }
    
    // 匹配 Option
    let some_value = Some(42);
    let none_value: Option<i32> = None;
    
    println!("   匹配 Option 类型:");
    for opt in &[some_value, none_value] {
        match opt {
            Some(value) => println!("     有值: {}", value),
            None => println!("     无值"),
        }
    }
    
    println!();
}

// 配置结构体示例
struct Config {
    timeout: u32,
}

// 6. if let 语句
fn if_let_statements() {
    println!("6. if let 语句:");
    
    // 简化的 Option 匹配
    let some_option = Some(5);
    println!("   if let 匹配 Option:");
    if let Some(value) = some_option {
        println!("     得到值: {}", value);
    } else {
        println!("     没有值");
    }
    
    // 处理 Result
    let result: Result<i32, &str> = Ok(42);
    println!("   if let 匹配 Result:");
    if let Ok(value) = result {
        println!("     成功结果: {}", value);
    }
    
    // 匹配复杂结构
    let config = Some(Config { timeout: 30 });
    println!("   匹配结构体:");
    if let Some(Config { timeout }) = config {
        println!("     超时设置: {} 秒", timeout);
    }
    
    println!();
}

// 7. 嵌套控制结构
fn nested_control_structures() {
    println!("7. 嵌套控制结构:");
    
    // 嵌套循环
    println!("   嵌套循环 - 乘法表:");
    for i in 1..=3 {
        for j in 1..=3 {
            print!("     {} × {} = {}  ", i, j, i * j);
        }
        println!();
    }
    
    // 带标签的嵌套循环
    println!("   带标签的嵌套循环:");
    'outer: for i in 1..5 {
        for j in 1..5 {
            if i * j > 6 {
                println!("     当 i={}, j={} 时退出外层循环", i, j);
                break 'outer;
            }
            println!("     {} × {} = {}", i, j, i * j);
        }
    }
    
    println!();
}

// 8. 错误处理控制流
fn error_handling_control_flow() {
    println!("8. 错误处理控制流:");
    
    // Result 类型的 match 处理
    println!("   使用 match 处理 Result:");
    let results = vec![
        divide(10.0, 2.0),
        divide(10.0, 0.0),
    ];
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("     结果 {}: {:.2}", i + 1, value),
            Err(msg) => println!("     错误 {}: {}", i + 1, msg),
        }
    }
    
    // 使用 if let 处理错误
    println!("   使用 if let 处理 Result:");
    let test_result = divide(15.0, 3.0);
    if let Ok(value) = test_result {
        println!("     除法成功: {:.2}", value);
    }
    
    println!();
}

// 除法函数示例
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(x / y)
    }
}

// 9. 迭代器和闭包控制流
fn iterator_closure_control_flow() {
    println!("9. 迭代器和闭包控制流:");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // map 变换
    println!("   使用 map 将每个数字乘以2:");
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("     原始: {:?}", numbers);
    println!("     翻倍: {:?}", doubled);
    
    // filter 过滤
    println!("   使用 filter 过滤偶数:");
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("     偶数: {:?}", evens);
    
    // 链式操作
    println!("   链式操作: 过滤偶数，翻倍，求和:");
    let sum: i32 = numbers
        .iter()
        .filter(|&x| x % 2 == 0)  // 过滤偶数
        .map(|x| x * 2)           // 翻倍
        .sum();                   // 求和
    println!("     结果: {}", sum);
    
    println!();
}

// 10. 高级控制流模式
fn advanced_control_flow_patterns() {
    println!("10. 高级控制流模式:");
    
    // 递归函数
    println!("   递归计算阶乘:");
    for i in 1..=5 {
        println!("     {}! = {}", i, factorial(i));
    }
    
    // 状态机模式
    println!("   状态机模式演示:");
    let mut state = State::Start;
    let mut step = 0;
    
    loop {
        step += 1;
        println!("     步骤 {}: {:?}", step, state);
        
        let new_state = process_state(state);
        if matches!(new_state, State::End) && matches!(state, State::End) {
            break;
        }
        state = new_state;
        
        if step > 6 {  // 防止无限循环
            break;
        }
    }
    
    println!();
}

// 递归计算阶乘
fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

// 状态机枚举
#[derive(Debug, Clone, Copy)]
enum State {
    Start,
    Middle(i32),
    End,
}

// 状态机处理函数
fn process_state(state: State) -> State {
    match state {
        State::Start => State::Middle(1),
        State::Middle(x) if x < 3 => State::Middle(x + 1),
        State::Middle(_) => State::End,
        State::End => State::End,
    }
}

// 主函数结束
fn main_end() {
    println!("=== 学习完成！===");
    println!("这些示例涵盖了 Rust 中流程控制的主要概念:");
    println!("- 条件语句 (if-else)");
    println!("- 循环语句 (loop, while, for)");
    println!("- 模式匹配 (match)");
    println!("- if let 和 while let 语句");
    println!("- 嵌套控制结构");
    println!("- 错误处理控制流");
    println!("- 迭代器和闭包控制流");
    println!("- 高级控制流模式");
    println!();
    println!("你可以运行 'cargo run' 来查看所有示例的执行结果。");
    println!("尝试修改参数和条件来更好地理解这些概念！");
}
