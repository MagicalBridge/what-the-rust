fn main() {
    println!("=== Rust Trait 学习项目 ===\n");

    // 1. 基本 trait 定义与实现
    basic_trait();

    // 2. 默认方法实现
    default_methods();

    // 3. trait 作为参数（静态分发 vs 动态分发）
    trait_as_parameter();

    // 4. trait 作为返回值
    trait_as_return();

    // 5. trait bound（特征约束）
    trait_bounds();

    // 6. 多个 trait 约束
    multiple_trait_bounds();

    // 7. derive 宏自动实现 trait
    derive_traits();

    // 8. 常用标准库 trait（Display, From, Into）
    std_traits();

    // 9. trait 继承（supertrait）
    trait_inheritance();

    // 10. 关联类型（Associated Types）
    associated_types();

    // 总结
    main_end();
}

// ============================================================
// 1. 基本 trait 定义与实现
// ============================================================
fn basic_trait() {
    println!("1. 基本 trait 定义与实现:");

    // 定义一个 trait —— 就像一份"合同"，规定了要实现哪些方法
    trait Animal {
        fn name(&self) -> &str;
        fn sound(&self) -> &str;
        fn info(&self) -> String {
            format!("{} 的叫声是: {}", self.name(), self.sound())
        }
    }

    struct Dog {
        name: String,
    }

    struct Cat {
        name: String,
    }

    // 为 Dog 实现 Animal trait
    impl Animal for Dog {
        fn name(&self) -> &str {
            &self.name
        }
        fn sound(&self) -> &str {
            "汪汪"
        }
    }

    // 为 Cat 实现 Animal trait
    impl Animal for Cat {
        fn name(&self) -> &str {
            &self.name
        }
        fn sound(&self) -> &str {
            "喵喵"
        }
    }

    let dog = Dog { name: String::from("旺财") };
    let cat = Cat { name: String::from("小花") };

    // 调用 trait 方法
    println!("   {}", dog.info());
    println!("   {}", cat.info());

    println!();
}

// ============================================================
// 2. 默认方法实现
// ============================================================
fn default_methods() {
    println!("2. 默认方法实现:");

    // trait 中可以提供默认实现，实现者可以选择覆盖或直接使用
    trait Greeter {
        fn name(&self) -> &str;

        // 默认实现 —— 实现者可以不写这个方法
        fn greet(&self) -> String {
            format!("你好，我是 {}", self.name())
        }

        // 另一个默认实现
        fn farewell(&self) -> String {
            format!("{} 说: 再见！", self.name())
        }
    }

    struct Student {
        name: String,
    }

    struct Teacher {
        name: String,
    }

    // Student 使用默认的 greet 和 farewell
    impl Greeter for Student {
        fn name(&self) -> &str {
            &self.name
        }
    }

    // Teacher 覆盖了 greet，但保留 farewell 的默认实现
    impl Greeter for Teacher {
        fn name(&self) -> &str {
            &self.name
        }
        fn greet(&self) -> String {
            format!("同学们好，我是 {} 老师", self.name())
        }
    }

    let student = Student { name: String::from("小明") };
    let teacher = Teacher { name: String::from("王") };

    println!("   {}", student.greet());   // 使用默认实现
    println!("   {}", teacher.greet());   // 使用自定义实现
    println!("   {}", student.farewell()); // 默认实现
    println!("   {}", teacher.farewell()); // 默认实现

    println!();
}

// ============================================================
// 3. trait 作为参数（静态分发 vs 动态分发）
// ============================================================
fn trait_as_parameter() {
    println!("3. trait 作为参数:");

    trait Summary {
        fn summarize(&self) -> String;
    }

    struct Article {
        title: String,
        content: String,
    }

    struct Tweet {
        username: String,
        message: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("[文章] {}: {}...", self.title, &self.content[..self.content.len().min(20)])
        }
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("[推文] @{}: {}", self.username, self.message)
        }
    }

    // 方式1: impl Trait 语法（静态分发，编译期确定类型，性能更好）
    fn notify_static(item: &impl Summary) {
        println!("   [静态分发] 摘要: {}", item.summarize());
    }

    // 方式2: dyn Trait 语法（动态分发，运行时确定类型，更灵活）
    fn notify_dynamic(item: &dyn Summary) {
        println!("   [动态分发] 摘要: {}", item.summarize());
    }

    let article = Article {
        title: String::from("Rust入门"),
        content: String::from("Rust 是一门系统编程语言，注重安全性和性能"),
    };
    let tweet = Tweet {
        username: String::from("rustacean"),
        message: String::from("Rust 太棒了！"),
    };

    // 静态分发
    notify_static(&article);
    notify_static(&tweet);

    // 动态分发
    notify_dynamic(&article);
    notify_dynamic(&tweet);

    // 动态分发的优势：可以放进同一个集合
    let items: Vec<&dyn Summary> = vec![&article, &tweet];
    println!("   [集合遍历] 共 {} 条内容:", items.len());
    for item in &items {
        println!("     - {}", item.summarize());
    }

    println!();
}

// ============================================================
// 4. trait 作为返回值
// ============================================================
fn trait_as_return() {
    println!("4. trait 作为返回值:");

    trait Drawable {
        fn draw(&self) -> String;
    }

    struct Circle {
        radius: f64,
    }

    struct Square {
        side: f64,
    }

    impl Drawable for Circle {
        fn draw(&self) -> String {
            format!("绘制圆形 (半径={})", self.radius)
        }
    }

    impl Drawable for Square {
        fn draw(&self) -> String {
            format!("绘制正方形 (边长={})", self.side)
        }
    }

    // impl Trait 作为返回值 —— 但只能返回同一种具体类型
    fn create_circle() -> impl Drawable {
        Circle { radius: 5.0 }
    }

    // 如果需要返回不同类型，用 Box<dyn Trait>
    fn create_shape(shape_type: &str) -> Box<dyn Drawable> {
        match shape_type {
            "circle" => Box::new(Circle { radius: 3.0 }),
            "square" => Box::new(Square { side: 4.0 }),
            _ => Box::new(Circle { radius: 1.0 }),
        }
    }

    let circle = create_circle();
    println!("   {}", circle.draw());

    let shape1 = create_shape("circle");
    let shape2 = create_shape("square");
    println!("   {}", shape1.draw());
    println!("   {}", shape2.draw());

    println!();
}

// ============================================================
// 5. trait bound（特征约束）
// ============================================================
fn trait_bounds() {
    println!("5. trait bound（特征约束）:");

    use std::fmt;

    trait Printable: fmt::Display {
        fn print(&self) {
            println!("   打印: {}", self);
        }
    }

    struct Temperature {
        celsius: f64,
    }

    // 先实现 Display（因为 Printable 要求它）
    impl fmt::Display for Temperature {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:.1}°C", self.celsius)
        }
    }

    impl Printable for Temperature {}

    // 泛型函数 + trait bound: T 必须实现 Display + Clone
    fn print_twice<T: fmt::Display + Clone>(item: &T) {
        let copy = item.clone();
        println!("   第一次: {}", item);
        println!("   第二次: {}", copy);
    }

    // where 子句写法 —— 当约束很多时更清晰
    fn compare_and_print<T>(a: &T, b: &T)
    where
        T: fmt::Display + PartialOrd,
    {
        if a > b {
            println!("   {} > {}", a, b);
        } else {
            println!("   {} <= {}", a, b);
        }
    }

    let temp = Temperature { celsius: 36.5 };
    temp.print();

    print_twice(&"Hello Trait");
    compare_and_print(&10, &20);
    compare_and_print(&3.14, &2.71);

    println!();
}

// ============================================================
// 6. 多个 trait 约束
// ============================================================
fn multiple_trait_bounds() {
    println!("6. 多个 trait 约束:");

    use std::fmt;

    trait Serializable {
        fn serialize(&self) -> String;
    }

    trait Deserializable {
        fn type_name(&self) -> &str;
    }

    #[derive(Debug, Clone)]
    struct Config {
        key: String,
        value: String,
    }

    impl fmt::Display for Config {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}={}", self.key, self.value)
        }
    }

    impl Serializable for Config {
        fn serialize(&self) -> String {
            format!("{{\"{}\":\"{}\"}}", self.key, self.value)
        }
    }

    impl Deserializable for Config {
        fn type_name(&self) -> &str {
            "Config"
        }
    }

    // 要求同时满足多个 trait
    fn process<T>(item: &T)
    where
        T: Serializable + Deserializable + fmt::Display,
    {
        println!("   类型: {}", item.type_name());
        println!("   显示: {}", item);
        println!("   序列化: {}", item.serialize());
    }

    let config = Config {
        key: String::from("database_url"),
        value: String::from("localhost:5432"),
    };

    process(&config);

    println!();
}

// ============================================================
// 7. derive 宏自动实现 trait
// ============================================================
fn derive_traits() {
    println!("7. derive 宏自动实现 trait:");

    // derive 让编译器自动帮你实现常用 trait
    #[derive(Debug, Clone, PartialEq)]
    struct Point {
        x: f64,
        y: f64,
    }

    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone();        // Clone trait
    let p3 = Point { x: 3.0, y: 4.0 };

    println!("   Debug:    {:?}", p1);           // Debug trait
    println!("   相等判断: p1 == p2 → {}", p1 == p2);  // PartialEq trait
    println!("   相等判断: p1 == p3 → {}", p1 == p3);

    // 常用 derive trait 说明
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    struct Pixel {
        r: u8,
        g: u8,
        b: u8,
    }

    let black = Pixel::default();  // Default: 所有字段为零值
    let red = Pixel { r: 255, g: 0, b: 0 };
    let red_copy = red;            // Copy: 赋值时复制而非移动

    println!("   Default:  {:?}", black);
    println!("   Copy:     red={:?}, red_copy={:?}", red, red_copy);
    println!("   两者仍可使用 (因为 Copy): red.r={}", red.r);

    // 用 HashMap 验证 Hash trait
    use std::collections::HashMap;
    let mut color_names: HashMap<Pixel, &str> = HashMap::new();
    color_names.insert(Pixel { r: 255, g: 0, b: 0 }, "红色");
    color_names.insert(Pixel { r: 0, g: 255, b: 0 }, "绿色");
    color_names.insert(Pixel { r: 0, g: 0, b: 255 }, "蓝色");

    println!("   Hash:     红色像素查找 = {:?}",
        color_names.get(&Pixel { r: 255, g: 0, b: 0 }));

    println!();
}

// ============================================================
// 8. 常用标准库 trait（Display, From, Into）
// ============================================================
fn std_traits() {
    println!("8. 常用标准库 trait:");

    use std::fmt;

    // --- Display trait: 自定义打印格式 ---
    struct Money {
        amount: f64,
        currency: String,
    }

    impl fmt::Display for Money {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:.2} {}", self.amount, self.currency)
        }
    }

    let price = Money { amount: 99.5, currency: String::from("USDT") };
    println!("   Display:  {}", price);  // 自动调用 Display

    // --- From / Into trait: 类型转换 ---
    struct Celsius(f64);
    struct Fahrenheit(f64);

    impl From<Celsius> for Fahrenheit {
        fn from(c: Celsius) -> Self {
            Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
        }
    }

    impl fmt::Display for Celsius {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:.1}°C", self.0)
        }
    }

    impl fmt::Display for Fahrenheit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:.1}°F", self.0)
        }
    }

    let boiling = Celsius(100.0);
    println!("   From:     {} = {}", boiling, Fahrenheit::from(Celsius(100.0)));

    // 实现 From 后自动获得 Into
    let body_temp = Celsius(37.0);
    let f: Fahrenheit = body_temp.into();  // 自动获得的 Into
    println!("   Into:     37°C = {}", f);

    // --- Iterator trait 简单展示 ---
    struct Countdown {
        value: i32,
    }

    impl Iterator for Countdown {
        type Item = i32;  // 关联类型（下一节详细讲）

        fn next(&mut self) -> Option<Self::Item> {
            if self.value > 0 {
                let current = self.value;
                self.value -= 1;
                Some(current)
            } else {
                None
            }
        }
    }

    let countdown = Countdown { value: 5 };
    let nums: Vec<i32> = countdown.collect();
    println!("   Iterator: 倒计时 = {:?}", nums);

    println!();
}

// ============================================================
// 9. trait 继承（supertrait）
// ============================================================
fn trait_inheritance() {
    println!("9. trait 继承（supertrait）:");

    use std::fmt;

    // 基础 trait
    trait Named {
        fn name(&self) -> &str;
    }

    // Describable 继承 Named + Display
    // 意味着实现 Describable 之前，必须先实现 Named 和 Display
    trait Describable: Named + fmt::Display {
        fn describe(&self) -> String {
            format!("[{}] {}", self.name(), self)
        }
    }

    struct Server {
        hostname: String,
        ip: String,
        port: u16,
    }

    impl Named for Server {
        fn name(&self) -> &str {
            &self.hostname
        }
    }

    impl fmt::Display for Server {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}:{}", self.ip, self.port)
        }
    }

    // 现在才能实现 Describable（因为前置条件已满足）
    impl Describable for Server {}

    let server = Server {
        hostname: String::from("api-gateway"),
        ip: String::from("192.168.1.100"),
        port: 8080,
    };

    println!("   Named:       {}", server.name());
    println!("   Display:     {}", server);
    println!("   Describable: {}", server.describe());

    println!();
}

// ============================================================
// 10. 关联类型（Associated Types）
// ============================================================
fn associated_types() {
    println!("10. 关联类型（Associated Types）:");

    // 关联类型让 trait 的实现者决定某些类型
    // 对比泛型: trait Container<T> 允许多个实现
    // 关联类型: trait Container { type Item; } 每个类型只有一个实现

    trait Container {
        type Item;  // 关联类型 —— 由实现者决定

        fn add(&mut self, item: Self::Item);
        fn get(&self, index: usize) -> Option<&Self::Item>;
        fn len(&self) -> usize;
        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    // 一个简单的栈
    struct Stack<T> {
        items: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Self {
            Stack { items: Vec::new() }
        }
    }

    // 为 Stack<T> 实现 Container，关联类型 Item = T
    impl<T> Container for Stack<T> {
        type Item = T;

        fn add(&mut self, item: T) {
            self.items.push(item);
        }

        fn get(&self, index: usize) -> Option<&T> {
            self.items.get(index)
        }

        fn len(&self) -> usize {
            self.items.len()
        }
    }

    let mut stack: Stack<String> = Stack::new();
    stack.add(String::from("第一个"));
    stack.add(String::from("第二个"));
    stack.add(String::from("第三个"));

    println!("   栈大小: {}", stack.len());
    println!("   是否为空: {}", stack.is_empty());
    println!("   索引0: {:?}", stack.get(0));
    println!("   索引2: {:?}", stack.get(2));

    // 关联类型 vs 泛型的区别
    // 用关联类型时，使用处不需要指定类型参数:
    fn print_first<C: Container>(container: &C)
    where
        C::Item: std::fmt::Debug,
    {
        if let Some(first) = container.get(0) {
            println!("   第一个元素: {:?}", first);
        }
    }

    print_first(&stack);

    println!();
}

// ============================================================
// 总结
// ============================================================
fn main_end() {
    println!("=== 学习完成！===");
    println!("这些示例涵盖了 Rust Trait 的主要概念:");
    println!("  1.  基本定义与实现 —— trait 就是行为的合同");
    println!("  2.  默认方法 —— 提供默认逻辑，实现者可选择覆盖");
    println!("  3.  作为参数 —— impl Trait（静态）vs dyn Trait（动态）");
    println!("  4.  作为返回值 —— impl Trait 或 Box<dyn Trait>");
    println!("  5.  trait bound —— 泛型约束，限定类型能力");
    println!("  6.  多 trait 约束 —— 用 + 组合多个 trait");
    println!("  7.  derive 宏 —— 自动实现 Debug/Clone/PartialEq 等");
    println!("  8.  标准库 trait —— Display, From/Into, Iterator");
    println!("  9.  trait 继承 —— supertrait 实现层级关系");
    println!(" 10.  关联类型 —— 让实现者决定 trait 中的类型");
    println!();
    println!("运行 'cargo run' 查看所有示例的执行结果！");
}
