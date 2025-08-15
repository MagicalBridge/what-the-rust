# Rust 流程控制学习项目

这个项目是一个全面的 Rust 流程控制学习指南，包含了条件语句、循环、模式匹配等各种流程控制结构的详细示例。

## 📋 项目概述

本项目旨在通过实际代码示例来学习 Rust 中的流程控制语句，涵盖了从基础到高级的各种控制流结构。每个示例都有详细的注释和输出说明，帮助理解 Rust 流程控制的工作原理。

## 🚀 快速开始

### 环境要求
- Rust 1.70+ 
- Cargo

### 运行项目
```bash
# 克隆项目（如果是从远程仓库）
git clone <repository-url>
cd 08_flow-control_demo

# 运行项目
cargo run
```

### 编译项目
```bash
cargo build
```

## 📚 学习内容

### 1. 条件语句 (if-else)
- 基本 if 语句
- if-else 语句
- else if 链式条件
- if 表达式返回值
- 条件表达式的简写

**示例代码：**
```rust
// 基本 if 语句
let number = 5;
if number > 0 {
    println!("数字是正数");
}

// if 表达式返回值
let result = if number > 0 { "正数" } else { "非正数" };
```

### 2. 循环语句 (loop)
- 无限循环 loop
- 使用 break 退出循环
- 使用 continue 跳过当前迭代
- 循环标签和嵌套循环
- loop 表达式返回值

**示例代码：**
```rust
// 无限循环
let mut counter = 0;
loop {
    counter += 1;
    if counter == 10 {
        break;
    }
}

// 循环返回值
let result = loop {
    counter += 1;
    if counter == 20 {
        break counter * 2;
    }
};
```

### 3. 条件循环 (while)
- 基本 while 循环
- while let 模式匹配循环
- 复杂条件的 while 循环

**示例代码：**
```rust
// 基本 while 循环
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}

// while let 模式匹配
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### 4. for 循环
- 遍历数组和集合
- 使用范围 (range)
- enumerate 获取索引
- 迭代引用和值
- 反向迭代

**示例代码：**
```rust
// 遍历数组
let arr = [10, 20, 30, 40, 50];
for element in arr {
    println!("值是: {}", element);
}

// 使用范围
for number in 1..4 {
    println!("{}!", number);
}

// 获取索引
for (i, value) in arr.iter().enumerate() {
    println!("索引 {} 的值是 {}", i, value);
}
```

### 5. 模式匹配 (match)
- 基本 match 表达式
- 匹配字面量
- 匹配变量
- 匹配通配符
- 匹配多个值
- 匹配范围
- 守卫条件 (match guards)

**示例代码：**
```rust
let number = 13;
match number {
    1 => println!("一"),
    2 | 3 | 5 | 7 | 11 => println!("这是一个小于20的质数"),
    13..=19 => println!("13到19之间的数字"),
    _ => println!("其他数字"),
}

// 带守卫的匹配
let num = Some(4);
match num {
    Some(x) if x < 5 => println!("小于五: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

### 6. if let 语句
- 简化的模式匹配
- Option 类型处理
- Result 类型处理
- 多个模式匹配

**示例代码：**
```rust
let some_option = Some(5);
if let Some(value) = some_option {
    println!("得到一个值: {}", value);
}

// 处理 Result
let result: Result<i32, &str> = Ok(42);
if let Ok(value) = result {
    println!("成功: {}", value);
}
```

### 7. 嵌套控制结构
- 嵌套 if 语句
- 嵌套循环
- 循环中的条件语句
- 复杂的控制流组合

**示例代码：**
```rust
// 嵌套循环
for i in 1..=3 {
    for j in 1..=3 {
        println!("i = {}, j = {}", i, j);
    }
}

// 带标签的嵌套循环
'outer: for i in 1..5 {
    for j in 1..5 {
        if i * j > 6 {
            break 'outer;
        }
        println!("{} * {} = {}", i, j, i * j);
    }
}
```

### 8. 错误处理控制流
- Result 类型的 match 处理
- 使用 ? 操作符
- panic! 和错误恢复
- unwrap 和 expect

**示例代码：**
```rust
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(x / y)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println!("结果: {}", result),
    Err(msg) => println!("错误: {}", msg),
}
```

### 9. 迭代器和闭包控制流
- map、filter、fold 等迭代器方法
- 闭包作为控制结构
- 链式操作
- collect 和消费适配器

**示例代码：**
```rust
let numbers = vec![1, 2, 3, 4, 5];

// 链式操作
let result: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .filter(|&x| x > 4)
    .collect();

// fold 操作
let sum = numbers.iter().fold(0, |acc, x| acc + x);
```

### 10. 高级控制流模式
- 函数式编程风格
- 状态机模式
- 递归控制结构
- 异步控制流基础

**示例代码：**
```rust
// 递归函数
fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

// 状态机模式
enum State {
    Start,
    Middle(i32),
    End,
}

fn process_state(state: State) -> State {
    match state {
        State::Start => State::Middle(1),
        State::Middle(x) if x < 5 => State::Middle(x + 1),
        State::Middle(_) => State::End,
        State::End => State::End,
    }
}
```

## 🔧 项目结构

```
08_flow-control_demo/
├── Cargo.toml          # 项目配置文件
├── Cargo.lock          # 依赖锁定文件（运行后生成）
├── README.md           # 项目说明文档
└── src/
    └── main.rs         # 主程序文件
```

## 📖 学习建议

1. **循序渐进**：按照代码中的顺序学习，从基础控制结构开始
2. **动手实践**：修改示例中的条件和循环参数，观察输出变化
3. **理解性能**：注意不同控制结构的性能影响
4. **模式匹配**：重点理解 match 和 if let 的使用场景
5. **错误处理**：掌握 Rust 特有的错误处理模式

## 🎯 学习目标

完成本项目后，你将能够：

- ✅ 熟练使用 Rust 的各种条件语句和循环结构
- ✅ 理解 match 模式匹配的强大功能
- ✅ 掌握 if let 和 while let 的使用方法
- ✅ 能够处理嵌套控制结构和复杂逻辑
- ✅ 了解函数式编程风格的控制流
- ✅ 掌握错误处理的控制流模式

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来改进这个项目！

## 📄 许可证

本项目采用 MIT 许可证。

## 🔗 相关资源

- [Rust 官方文档 - 控制流](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust 官方文档 - 模式匹配](https://doc.rust-lang.org/book/ch06-02-match.html)
- [Rust 官方文档 - if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)

---

**Happy Coding with Rust! 🦀**
