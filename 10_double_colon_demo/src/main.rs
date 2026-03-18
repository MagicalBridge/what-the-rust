// ============================================================
// Rust 双冒号 :: (Path Separator) 全面学习
// ============================================================
// :: 是 Rust 中的路径分隔符，用于访问模块、类型、函数等命名空间中的项。
// 类似于其他语言中的 . 或 /，但在 Rust 中有更多的语义。

fn main() {
    println!("=== Rust 双冒号 :: 路径语法学习 ===\n");

    // 1. 关联函数调用（最常见的用法）
    associated_function_demo();

    // 2. 模块路径访问
    module_path_demo();

    // 3. 枚举变体访问
    enum_variant_demo();

    // 4. 标准库中的 :: 用法
    std_lib_demo();

    // 5. Trait 限定调用（完全限定语法）
    trait_qualified_demo();

    // 6. 泛型中的 Turbofish 语法 ::<>
    turbofish_demo();

    // 7. use 语句中的路径
    use_path_demo();

    // 总结
    summary();
}

// ============================================================
// 1. 关联函数调用 — Type::function()
// ============================================================
// 关联函数是定义在 impl 块中但没有 self 参数的函数。
// 通过 Type::function() 调用，类似其他语言的"静态方法"。
fn associated_function_demo() {
    println!("1. 关联函数调用 (Type::function):");

    struct ServerConfig {
        host: String,
        port: u16,
        max_connections: u32,
    }

    impl ServerConfig {
        // 关联函数：没有 &self 参数，用 :: 调用
        fn new(host: &str, port: u16) -> ServerConfig {
            ServerConfig {
                host: host.to_string(),
                port,
                max_connections: 100,
            }
        }

        fn load_from_yaml(path: &str) -> Result<ServerConfig, String> {
            // 模拟从 YAML 加载配置
            println!("   [模拟] 从 {} 加载配置...", path);
            Ok(ServerConfig::new("0.0.0.0", 8080))
        }

        fn default_local() -> ServerConfig {
            ServerConfig::new("127.0.0.1", 3000)
        }

        // 实例方法：有 &self 参数，用 . 调用
        fn display(&self) {
            println!(
                "   {}:{} (最大连接: {})",
                self.host, self.port, self.max_connections
            );
        }
    }

    // :: 调用关联函数
    let config1 = ServerConfig::new("0.0.0.0", 8080); // :: 调用
    let config2 = ServerConfig::default_local(); // :: 调用
    let config3 = ServerConfig::load_from_yaml("config.yaml"); // :: 调用

    // . 调用实例方法
    config1.display(); // . 调用
    config2.display(); // . 调用

    if let Ok(config) = config3 {
        config.display();
    }

    // 常见的标准库关联函数
    let s = String::new(); // String::new()
    let s2 = String::from("hello"); // String::from()
    let v: Vec<i32> = Vec::new(); // Vec::new()
    let v2 = Vec::from([1, 2, 3]); // Vec::from()
    let boxed = Box::new(42); // Box::new()

    println!("   String::new() => {:?}", s);
    println!("   String::from(\"hello\") => {:?}", s2);
    println!("   Vec::new() => {:?}", v);
    println!("   Vec::from([1,2,3]) => {:?}", v2);
    println!("   Box::new(42) => {:?}", boxed);

    println!();
}

// ============================================================
// 2. 模块路径访问 — module::item
// ============================================================
mod network {
    pub fn connect() -> String {
        "已连接".to_string()
    }

    pub mod http {
        pub fn get(url: &str) -> String {
            format!("GET {}", url)
        }

        pub fn post(url: &str) -> String {
            format!("POST {}", url)
        }
    }

    pub mod websocket {
        pub struct WsClient {
            pub url: String,
        }

        impl WsClient {
            pub fn new(url: &str) -> WsClient {
                WsClient {
                    url: url.to_string(),
                }
            }

            pub fn send(&self, msg: &str) -> String {
                format!("发送到 {}: {}", self.url, msg)
            }
        }
    }
}

fn module_path_demo() {
    println!("2. 模块路径访问 (module::item):");

    // 一级模块访问
    let status = network::connect();
    println!("   network::connect() => {}", status);

    // 嵌套模块访问
    let response = network::http::get("https://api.example.com");
    println!("   network::http::get() => {}", response);

    let response = network::http::post("https://api.example.com/data");
    println!("   network::http::post() => {}", response);

    // 模块中的类型 + 关联函数（组合使用 ::）
    let client = network::websocket::WsClient::new("wss://stream.example.com");
    //           ^^^^^^^ ^^^^^^^^^ ^^^^^^^^ ^^^
    //           模块      子模块     类型    关联函数
    //              ::        ::       ::
    let msg = client.send("ping");
    println!("   network::websocket::WsClient::new() => {}", msg);

    println!();
}

// ============================================================
// 3. 枚举变体访问 — Enum::Variant
// ============================================================
fn enum_variant_demo() {
    println!("3. 枚举变体访问 (Enum::Variant):");

    #[derive(Debug)]
    enum OrderSide {
        Buy,
        Sell,
    }

    #[derive(Debug)]
    enum OrderType {
        Market,
        Limit { price: f64 },
        StopLoss { trigger: f64, price: f64 },
    }

    #[derive(Debug)]
    enum OrderStatus {
        Pending,
        Filled(f64),       // 包含成交价
        Cancelled(String), // 包含原因
    }

    // 用 :: 访问枚举变体
    let side = OrderSide::Buy;
    let order_type = OrderType::Limit { price: 50000.0 };
    let status = OrderStatus::Filled(50001.5);

    println!("   OrderSide::Buy => {:?}", side);
    println!(
        "   OrderType::Limit {{ price: 50000.0 }} => {:?}",
        order_type
    );
    println!("   OrderStatus::Filled(50001.5) => {:?}", status);

    // match 中也使用 :: 匹配变体
    match &status {
        OrderStatus::Pending => println!("   状态: 待处理"),
        OrderStatus::Filled(price) => println!("   状态: 已成交，价格 {}", price),
        OrderStatus::Cancelled(reason) => println!("   状态: 已取消，原因: {}", reason),
    }

    // Option 和 Result 也是枚举，用 :: 访问
    let some_value: Option<i32> = Some(42); // Some 是 Option::Some 的简写
    let none_value: Option<i32> = None; // None 是 Option::None 的简写
    let ok_value: Result<i32, String> = Ok(100); // Ok 是 Result::Ok 的简写

    // 完整写法（不常用但合法）
    let _full_some: Option<i32> = Option::Some(42);
    let _full_none: Option<i32> = Option::None;
    let _full_ok: Result<i32, String> = Result::Ok(100);

    println!("   Option::Some(42) => {:?}", some_value);
    println!("   Option::None => {:?}", none_value);
    println!("   Result::Ok(100) => {:?}", ok_value);

    println!();
}

// ============================================================
// 4. 标准库中的 :: 用法
// ============================================================
fn std_lib_demo() {
    println!("4. 标准库常见 :: 用法:");

    // std::collections::HashMap
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("BTC", 50000.0);
    map.insert("ETH", 3000.0);
    println!("   std::collections::HashMap::new() => {:?}", map);

    // std::f64::consts
    let pi = std::f64::consts::PI;
    let e = std::f64::consts::E;
    println!("   std::f64::consts::PI => {}", pi);
    println!("   std::f64::consts::E  => {}", e);

    // std::cmp
    let max = std::cmp::max(10, 20);
    let min = std::cmp::min(10, 20);
    println!("   std::cmp::max(10, 20) => {}", max);
    println!("   std::cmp::min(10, 20) => {}", min);

    // std::mem
    let size = std::mem::size_of::<i32>();
    let size_str = std::mem::size_of::<String>();
    println!("   std::mem::size_of::<i32>()    => {} bytes", size);
    println!("   std::mem::size_of::<String>() => {} bytes", size_str);

    // i32/u64 等原始类型的关联常量和函数
    println!("   i32::MAX  => {}", i32::MAX);
    println!("   i32::MIN  => {}", i32::MIN);
    println!("   u64::MAX  => {}", u64::MAX);
    println!("   f64::INFINITY  => {}", f64::INFINITY);
    println!("   f64::NAN       => {}", f64::NAN);

    // str 的关联方法
    let parsed: i32 = "42".parse().unwrap();
    let from_str = i32::from_str_radix("FF", 16).unwrap();
    println!("   \"42\".parse::<i32>()           => {}", parsed);
    println!("   i32::from_str_radix(\"FF\", 16) => {}", from_str);

    println!();
}

// ============================================================
// 5. Trait 限定调用（完全限定语法）— <Type as Trait>::method()
// ============================================================
fn trait_qualified_demo() {
    println!("5. Trait 完全限定调用 (<Type as Trait>::method):");

    trait Summary {
        fn summarize(&self) -> String;
    }

    trait Preview {
        fn summarize(&self) -> String; // 注意：方法名和 Summary 一样！
    }

    struct Article {
        title: String,
        content: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            let preview: String = self.content.chars().take(5).collect();
            format!("[摘要] {}: {}...", self.title, preview)
        }
    }

    impl Preview for Article {
        fn summarize(&self) -> String {
            format!("[预览] {} - {}", self.title, self.content)
        }
    }

    let article = Article {
        title: "Rust双冒号".to_string(),
        content: "双冒号是路径分隔符".to_string(),
    };

    // 当两个 trait 有同名方法时，必须使用完全限定语法
    let s1 = <Article as Summary>::summarize(&article);
    let s2 = <Article as Preview>::summarize(&article);
    println!("   <Article as Summary>::summarize() => {}", s1);
    println!("   <Article as Preview>::summarize() => {}", s2);

    println!();
}

// ============================================================
// 6. Turbofish 语法 ::<> — 显式指定泛型类型
// ============================================================
fn turbofish_demo() {
    println!("6. Turbofish 语法 (function::<Type>):");

    // 解析字符串时指定目标类型
    let x = "42".parse::<i32>().unwrap();
    let y = "3.14".parse::<f64>().unwrap();
    let z = "true".parse::<bool>().unwrap();
    println!("   \"42\".parse::<i32>()     => {}", x);
    println!("   \"3.14\".parse::<f64>()   => {}", y);
    println!("   \"true\".parse::<bool>()  => {}", z);

    // collect 时指定集合类型
    let numbers: Vec<i32> = (1..=5).collect(); // 方式1: 类型注解
    let numbers2 = (1..=5).collect::<Vec<i32>>(); // 方式2: turbofish
    println!("   (1..=5).collect::<Vec<i32>>() => {:?}", numbers);
    println!("   两种方式结果相同: {}", numbers == numbers2);

    // 泛型函数调用
    fn convert<T: std::str::FromStr>(s: &str) -> Option<T> {
        s.parse::<T>().ok()
    }

    let val = convert::<i32>("123");
    let val2 = convert::<f64>("45.6");
    println!("   convert::<i32>(\"123\")  => {:?}", val);
    println!("   convert::<f64>(\"45.6\") => {:?}", val2);

    // std::mem::size_of 中的 turbofish
    println!(
        "   size_of::<u8>()  = {} byte",
        std::mem::size_of::<u8>()
    );
    println!(
        "   size_of::<u32>() = {} bytes",
        std::mem::size_of::<u32>()
    );
    println!(
        "   size_of::<u64>() = {} bytes",
        std::mem::size_of::<u64>()
    );

    println!();
}

// ============================================================
// 7. use 语句中的路径
// ============================================================
fn use_path_demo() {
    println!("7. use 语句中的 :: 路径:");

    // use 引入后可以直接使用短名称
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert("BTC");
    set.insert("ETH");
    println!("   use std::collections::HashSet => {:?}", set);

    // 嵌套 use（一次引入多个）
    // use std::collections::{HashMap, BTreeMap};
    // use std::io::{self, Read, Write};

    // self 关键字 — 引入模块本身
    // use std::io::{self, Read};  // 等价于 use std::io; use std::io::Read;

    // 重命名引入
    use std::collections::HashMap as Map;
    let mut m = Map::new();
    m.insert("key", "value");
    println!("   use HashMap as Map => {:?}", m);

    // crate:: — 从当前 crate 的根路径开始
    let resp = crate::network::http::get("https://example.com");
    println!("   crate::network::http::get() => {}", resp);

    // self:: — 从当前模块开始（通常在模块内部使用）
    // super:: — 从父模块开始

    println!();
}

// ============================================================
// 总结对比表
// ============================================================
fn summary() {
    println!("=== :: 双冒号用法总结 ===\n");
    println!("  用法                          | 示例");
    println!("  ------------------------------|--------------------------------------------");
    println!("  关联函数调用                   | String::from(\"hello\")");
    println!("  关联常量访问                   | i32::MAX");
    println!("  模块路径访问                   | std::collections::HashMap");
    println!("  嵌套模块                       | network::http::get()");
    println!("  枚举变体                       | Option::Some(42)");
    println!("  完全限定调用                   | <Type as Trait>::method()");
    println!("  Turbofish 泛型                | \"42\".parse::<i32>()");
    println!("  use 引入                      | use std::io::Read");
    println!("  crate 根路径                  | crate::module::function()");
    println!();
    println!("  对比: . (点号) 用于实例方法调用");
    println!("  config.display()   — 通过实例调用");
    println!("  Config::new()      — 通过类型调用");
    println!();
    println!("  记忆口诀: :: 找\"名字\"，. 找\"东西\"");
    println!("  :: 是在命名空间中定位（类型/模块/枚举）");
    println!("  .  是对已有实例进行操作");
    println!();
    println!("你可以运行 'cargo run' 来查看所有示例的执行结果。");
}
