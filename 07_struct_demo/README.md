# Rust 结构体学习项目

这个项目是一个全面的 Rust 结构体学习指南，包含了结构体的各种用法和实际应用示例。

## 📋 项目概述

本项目旨在通过实际代码示例来学习 Rust 中的结构体（Struct），涵盖了从基础到高级的各种结构体概念。每个示例都有详细的注释和输出说明，帮助理解 Rust 结构体的工作原理和使用方法。

## 🚀 快速开始

### 环境要求
- Rust 1.70+ 
- Cargo

### 运行项目
```bash
# 克隆项目（如果是从远程仓库）
git clone <repository-url>
cd 07_struct_demo

# 运行项目
cargo run
```

### 编译项目
```bash
cargo build
```

## 📚 学习内容

### 1. 基本结构体定义和使用
- 结构体定义语法
- 创建结构体实例
- 字段初始化简写语法
- 结构体作为函数参数

**示例代码：**
```rust
// 定义结构体
struct Person {
    name: String,
    age: u32,
    email: String,
}

// 创建结构体实例
let person1 = Person {
    name: String::from("张三"),
    age: 25,
    email: String::from("zhangsan@example.com"),
};
```

### 2. 结构体字段访问和修改
- 字段访问和修改
- 可变结构体实例
- 字段引用和可变引用
- 结构体解构
- 部分解构

**示例代码：**
```rust
let mut point = Point { x: 10.0, y: 20.0 };
point.x = 15.0;  // 修改字段

// 结构体解构
let Point { x, y } = point;
```

### 3. 结构体方法定义
- 实例方法定义
- `&self` 参数
- 可变方法（`&mut self`）
- 方法调用语法

**示例代码：**
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
```

### 4. 关联函数（静态方法）
- 关联函数定义
- 构造函数模式
- 工厂方法
- 与实例方法的区别

**示例代码：**
```rust
impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    
    fn unit() -> Circle {
        Circle { radius: 1.0 }
    }
}
```

### 5. 结构体更新语法
- 使用 `..` 语法
- 从现有实例创建新实例
- 部分字段更新
- 链式更新

**示例代码：**
```rust
let user2 = User {
    username: String::from("user2"),
    email: String::from("user2@example.com"),
    ..user1  // 从user1复制其他字段
};
```

### 6. 元组结构体
- 元组结构体定义
- 字段访问（使用索引）
- 元组结构体方法
- 解构元组结构体

**示例代码：**
```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
println!("RGB({}, {}, {})", black.0, black.1, black.2);
```

### 7. 单元结构体
- 单元结构体定义
- 零大小类型
- 特征实现载体
- 状态标记

**示例代码：**
```rust
struct AlwaysEqual;
let subject = AlwaysEqual;

impl AlwaysEqual {
    fn is_equal(&self, _other: &AlwaysEqual) -> bool {
        true
    }
}
```

### 8. 结构体所有权和借用
- 结构体字段的所有权
- 借用和可变借用
- 结构体作为函数参数
- 所有权转移

**示例代码：**
```rust
fn print_book_info(book: &Book) {
    println!("书籍信息: {} 作者: {}", book.title, book.author);
}

fn update_book_pages(book: &mut Book, new_pages: u32) {
    book.pages = new_pages;
}
```

### 9. 结构体实现特征（trait）
- 特征定义和实现
- 多个特征实现
- 特征对象
- 特征约束

**示例代码：**
```rust
trait Printable {
    fn print(&self);
    fn get_info(&self) -> String;
}

impl Printable for Shape {
    fn print(&self) {
        println!("形状: {} 位置: ({}, {})", self.name, self.x, self.y);
    }
    
    fn get_info(&self) -> String {
        format!("{} at ({}, {})", self.name, self.x, self.y)
    }
}
```

### 10. 高级结构体操作
- 泛型结构体
- 嵌套结构体
- 派生特征（Clone, Debug, PartialEq等）
- 自定义特征实现

**示例代码：**
```rust
// 泛型结构体
struct Pair<T, U> {
    first: T,
    second: U,
}

// 派生特征
#[derive(Clone, Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}
```

## 🔧 项目结构

```
07_struct_demo/
├── Cargo.toml          # 项目配置文件
├── Cargo.lock          # 依赖锁定文件
├── README.md           # 项目说明文档
└── src/
    └── main.rs         # 主程序文件
```

## 📖 学习建议

1. **循序渐进**：按照代码中的顺序学习，从基本结构体开始
2. **动手实践**：修改示例中的字段和方法，观察输出变化
3. **理解所有权**：特别注意结构体字段的所有权规则
4. **方法调用**：理解 `&self`、`&mut self` 和关联函数的区别
5. **特征实现**：掌握如何为结构体实现自定义特征

## 🎯 学习目标

完成本项目后，你将能够：

- ✅ 熟练定义和使用 Rust 结构体
- ✅ 理解结构体字段的所有权和借用规则
- ✅ 掌握结构体方法的定义和调用
- ✅ 使用关联函数和构造函数模式
- ✅ 实现自定义特征和特征对象
- ✅ 使用泛型结构体和高级特性

## 🔍 关键概念

### 结构体类型
- **普通结构体**：有命名字段的结构体
- **元组结构体**：使用索引访问字段的结构体
- **单元结构体**：没有字段的结构体

### 方法类型
- **实例方法**：使用 `&self` 或 `&mut self` 参数
- **关联函数**：不使用 `self` 参数的函数

### 所有权规则
- 结构体字段遵循 Rust 的所有权规则
- 可以借用整个结构体或单个字段
- 结构体可以包含拥有数据或引用数据

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来改进这个项目！

## 📄 许可证

本项目采用 MIT 许可证。

## 🔗 相关资源

- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust 结构体文档](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust 特征文档](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

**Happy Coding with Rust! 🦀**
