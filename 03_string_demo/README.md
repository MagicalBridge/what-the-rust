# Rust 字符串操作学习项目

这个项目是一个全面的 Rust 字符串操作学习指南，包含了常见的字符串处理方法和实际应用示例。

## 📋 项目概述

本项目旨在通过实际代码示例来学习 Rust 中的字符串操作，涵盖了从基础到高级的各种字符串处理技巧。每个示例都有详细的注释和输出说明，帮助理解 Rust 字符串的工作原理。

## 🚀 快速开始

### 环境要求
- Rust 1.70+ 
- Cargo

### 运行项目
```bash
# 克隆项目（如果是从远程仓库）
git clone <repository-url>
cd 03_string_demo

# 运行项目
cargo run
```

### 编译项目
```bash
cargo build
```

## 📚 学习内容

### 1. 字符串创建和基本操作
- 空字符串创建
- 从字面量创建字符串
- 预分配容量优化
- 字符串长度计算（字节长度 vs 字符长度）
- 空字符串检查
- 字符串清空操作

**示例代码：**
```rust
// 创建空字符串
let mut empty_string = String::new();

// 从字面量创建
let hello = String::from("你好，世界！");

// 预分配容量
let mut preallocated = String::with_capacity(100);
```

### 2. 字符串连接和拼接
- 使用 `+` 操作符
- `push_str` 和 `push` 方法
- `format!` 宏
- `concat!` 宏
- `join` 方法

**示例代码：**
```rust
// 使用 + 操作符
let s1 = String::from("Hello");
let s2 = String::from(" World");
let result = s1 + &s2;

// 使用 format! 宏
let formatted = format!("我叫 {}，今年 {} 岁", name, age);
```

### 3. 字符串分割和切片
- 字符串切片操作
- 按分隔符分割
- 限制分割数量
- 按空白字符分割
- 按行分割
- 中文字符串处理

**示例代码：**
```rust
let text = "Hello,World,Rust,Programming";
let parts: Vec<&str> = text.split(',').collect();

// 字符串切片
println!("前5个字符: '{}'", &text[0..5]);
```

### 4. 字符串查找和替换
- 子字符串查找
- 位置查找（正向和反向）
- 字符串替换
- 条件替换
- 开始/结束检查
- 空白字符处理

**示例代码：**
```rust
let text = "Hello World Hello Rust";
println!("包含 'Hello': {}", text.contains("Hello"));

// 替换字符串
let replaced = text.replace("Hello", "Hi");
```

### 5. 字符串格式化
- 基本格式化
- 数字格式化（十进制、十六进制、八进制、二进制）
- 对齐和填充
- 精度控制
- 条件格式化

**示例代码：**
```rust
// 数字格式化
let number = 12345;
println!("十六进制: {:x}", number);
println!("二进制: {:b}", number);

// 对齐和填充
let text = "Hello";
println!("居中对齐: '{:^10}'", text);
```

### 6. 字符串转换
- 数字与字符串互转
- 大小写转换
- UTF-8 编码处理
- 字节数组转换
- 字符数组转换

**示例代码：**
```rust
// 数字转字符串
let number = 42;
let number_str = number.to_string();

// 大小写转换
let mixed = "Hello World";
println!("转小写: '{}'", mixed.to_lowercase());
```

### 7. 字符串迭代
- 按字符迭代
- 按字节迭代
- 按行迭代
- 按单词迭代
- 字符位置迭代

**示例代码：**
```rust
let text = "Hello世界";
for (i, ch) in text.chars().enumerate() {
    println!("字符 {}: '{}' (Unicode: {})", i, ch, ch as u32);
}
```

### 8. 字符串验证
- 空字符串检查
- 数字字符串验证
- 字母字符串验证
- 回文检查
- 邮箱格式验证
- 长度验证

**示例代码：**
```rust
// 回文检查
fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}
```

### 9. 字符串编码和字节操作
- UTF-8 编码处理
- 字节数组转换
- 无效 UTF-8 处理
- 字符编码转换

**示例代码：**
```rust
let text = "Hello世界";
let bytes = text.as_bytes();
println!("UTF-8 字节: {:?}", bytes);

// 处理无效 UTF-8
let lossy_string = String::from_utf8_lossy(&invalid_bytes);
```

### 10. 高级字符串操作
- 字符串去重
- 字符串反转
- 字符串排序
- 字符串统计
- 字符串压缩/展开
- 模式匹配分离

**示例代码：**
```rust
// 字符串反转
let text = "Hello世界";
let reversed: String = text.chars().rev().collect();

// 字符串压缩
fn compress_string(s: &str) -> String {
    // 实现字符串压缩算法
}
```

## 🔧 项目结构

```
03_string_demo/
├── Cargo.toml          # 项目配置文件
├── Cargo.lock          # 依赖锁定文件
├── README.md           # 项目说明文档
└── src/
    └── main.rs         # 主程序文件
```

## 📖 学习建议

1. **循序渐进**：按照代码中的顺序学习，从基础操作开始
2. **动手实践**：修改示例中的参数，观察输出变化
3. **理解原理**：注意字节长度和字符长度的区别
4. **中文支持**：特别关注中文字符串的处理方式
5. **性能考虑**：了解预分配容量等性能优化技巧

## 🎯 学习目标

完成本项目后，你将能够：

- ✅ 熟练使用 Rust 的字符串类型和方法
- ✅ 理解 UTF-8 编码和字符串处理
- ✅ 掌握字符串的常见操作和优化技巧
- ✅ 处理中文字符串和国际化文本
- ✅ 实现自定义的字符串处理算法

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来改进这个项目！

## 📄 许可证

本项目采用 MIT 许可证。

## 🔗 相关资源

- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust 字符串文档](https://doc.rust-lang.org/std/string/struct.String.html)
- [Rust 字符串切片文档](https://doc.rust-lang.org/std/primitive.str.html)

---

**Happy Coding with Rust! 🦀**
