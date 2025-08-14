fn main() {
    println!("=== Rust 结构体学习项目 ===\n");

    // 1. 基本结构体定义和使用
    basic_structs();
    
    // 2. 结构体字段访问和修改
    struct_field_access();
    
    // 3. 结构体方法定义
    struct_methods();
    
    // 4. 关联函数（静态方法）
    associated_functions();
    
    // 5. 结构体更新语法
    struct_update_syntax();
    
    // 6. 元组结构体
    tuple_structs();
    
    // 7. 单元结构体
    unit_structs();
    
    // 8. 结构体所有权和借用
    struct_ownership();
    
    // 9. 结构体实现特征（trait）
    struct_traits();
    
    // 10. 高级结构体操作
    advanced_structs();
    
    // 总结
    main_end();
}

// 1. 基本结构体定义和使用
fn basic_structs() {
    println!("1. 基本结构体定义和使用:");
    
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
    
    println!("   创建的结构体实例:");
    println!("   姓名: {}", person1.name);
    println!("   年龄: {}", person1.age);
    println!("   邮箱: {}", person1.email);
    
    // 使用字段初始化简写语法
    let name = String::from("李四");
    let age = 30;
    let person2 = Person {
        name,  // 字段名和变量名相同时可以简写
        age,
        email: String::from("lisi@example.com"),
    };
    
    println!("   使用简写语法创建:");
    println!("   姓名: {}, 年龄: {}", person2.name, person2.age);
    
    // 结构体作为函数参数
    fn print_person(person: &Person) {
        println!("   打印人员信息: {} ({}岁) - {}", person.name, person.age, person.email);
    }
    
    print_person(&person1);
    print_person(&person2);
    
    println!();
}

// 2. 结构体字段访问和修改
fn struct_field_access() {
    println!("2. 结构体字段访问和修改:");
    
    struct Point {
        x: f64,
        y: f64,
    }
    
    // 创建可变结构体实例
    let mut point = Point {
        x: 10.0,
        y: 20.0,
    };
    
    println!("   原始坐标: ({}, {})", point.x, point.y);
    
    // 修改字段值
    point.x = 15.0;
    point.y = 25.0;
    println!("   修改后坐标: ({}, {})", point.x, point.y);
    
    // 结构体字段的引用
    let x_ref = &point.x;
    let y_ref = &point.y;
    println!("   x坐标引用: {}", x_ref); // 15.0
    println!("   y坐标引用: {}", y_ref); // 25.0
    
    // 结构体字段的可变引用
    let x_mut_ref = &mut point.x;
    *x_mut_ref = 30.0;
    println!("   通过可变引用修改x: {}", point.x); // 30.0
    
    // 结构体解构
    let Point { x, y } = point;
    println!("   解构后的值: x = {}, y = {}", x, y);
    
    // 部分解构
    let point2 = Point { x: 5.0, y: 10.0 };
    let Point { x, .. } = point2;
    println!("   部分解构，只获取x: {}", x);
    
    println!();
}

// 3. 结构体方法定义
fn struct_methods() {
    println!("3. 结构体方法定义:");
    
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    // 为结构体实现方法
    impl Rectangle {
        // 实例方法 - 计算面积
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        // 实例方法 - 检查是否为正方形
        fn is_square(&self) -> bool {
            self.width == self.height
        }
        
        // 实例方法 - 调整大小
        fn resize(&mut self, width: u32, height: u32) {
            self.width = width;
            self.height = height;
        }
        
        // 实例方法 - 检查是否能包含另一个矩形
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        
        // 实例方法 - 获取周长
        fn perimeter(&self) -> u32 {
            2 * (self.width + self.height)
        }
    }
    
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("   矩形尺寸: {} x {}", rect.width, rect.height);
    println!("   面积: {}", rect.area());
    println!("   周长: {}", rect.perimeter());
    println!("   是否为正方形: {}", rect.is_square());
    
    // 调整大小
    rect.resize(40, 40);
    println!("   调整后尺寸: {} x {}", rect.width, rect.height);
    println!("   调整后是否为正方形: {}", rect.is_square());
    
    // 检查包含关系
    let small_rect = Rectangle {
        width: 20,
        height: 30,
    };
    println!("   小矩形尺寸: {} x {}", small_rect.width, small_rect.height);
    println!("   大矩形能否包含小矩形: {}", rect.can_hold(&small_rect));
    
    println!();
}

// 4. 关联函数（静态方法）
fn associated_functions() {
    println!("4. 关联函数（静态方法）:");
    
    struct Circle {
        radius: f64,
    }
    
    impl Circle {
        // 关联函数 - 创建圆形
        fn new(radius: f64) -> Circle {
            Circle { radius }
        }
        
        // 关联函数 - 创建单位圆
        fn unit() -> Circle {
            Circle { radius: 1.0 }
        }
        
        // 关联函数 - 从直径创建圆形
        fn from_diameter(diameter: f64) -> Circle {
            Circle { radius: diameter / 2.0 }
        }
        
        // 实例方法 - 计算面积
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
        
        // 实例方法 - 计算周长
        fn circumference(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }
    
    // 使用关联函数创建实例
    let circle1 = Circle::new(5.0);
    let circle2 = Circle::unit();
    let circle3 = Circle::from_diameter(10.0);
    
    println!("   圆形1 (半径5): 面积 = {:.2}, 周长 = {:.2}", 
             circle1.area(), circle1.circumference());
    println!("   单位圆: 面积 = {:.2}, 周长 = {:.2}", 
             circle2.area(), circle2.circumference());
    println!("   圆形3 (直径10): 面积 = {:.2}, 周长 = {:.2}", 
             circle3.area(), circle3.circumference());
    
    println!();
}

// 5. 结构体更新语法
fn struct_update_syntax() {
    println!("5. 结构体更新语法:");
    
    struct User {
        username: String,
        email: String,
        active: bool,
        sign_in_count: u64,
    }
    
    let user1 = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("   原始用户: {} ({})", user1.username, user1.email);
    
    // 使用更新语法创建新用户
    let user2 = User {
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        ..user1  // 从user1复制其他字段
    };
    
    println!("   新用户: {} ({})", user2.username, user2.email);
    println!("   新用户状态: active={}, sign_in_count={}", 
             user2.active, user2.sign_in_count);
    
    // 部分更新
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user1
    };
    
    println!("   部分更新用户: {} ({})", user3.username, user3.email);
    
    // 链式更新
    let user4 = User {
        username: String::from("user4"),
        ..User {
            username: String::from("temp"),
            email: String::from("user4@example.com"),
            active: false,
            sign_in_count: 0,
        }
    };
    
    println!("   链式更新用户: {} ({})", user4.username, user4.email);
    
    println!();
}

// 6. 元组结构体
fn tuple_structs() {
    println!("6. 元组结构体:");
    
    // 定义元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    struct Kilometers(i32);
    
    // 创建元组结构体实例
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let distance = Kilometers(10);
    
    println!("   颜色: RGB({}, {}, {})", black.0, black.1, black.2);
    println!("   原点: ({}, {}, {})", origin.0, origin.1, origin.2);
    println!("   距离: {} 公里", distance.0);
    
    // 元组结构体的方法
    impl Color {
        fn new(r: i32, g: i32, b: i32) -> Color {
            Color(r, g, b)
        }
        
        fn is_black(&self) -> bool {
            self.0 == 0 && self.1 == 0 && self.2 == 0
        }
        
        fn is_white(&self) -> bool {
            self.0 == 255 && self.1 == 255 && self.2 == 255
        }
        
        fn brightness(&self) -> f64 {
            (self.0 as f64 * 0.299 + self.1 as f64 * 0.587 + self.2 as f64 * 0.114) / 255.0
        }
    }
    
    let white = Color::new(255, 255, 255);
    let red = Color::new(255, 0, 0);
    
    println!("   黑色: {}", black.is_black());
    println!("   白色: {}", white.is_white());
    println!("   红色亮度: {:.2}", red.brightness());
    
    // 元组结构体解构
    let Color(r, g, b) = red;
    println!("   红色分量: R={}, G={}, B={}", r, g, b);
    
    println!();
}

// 7. 单元结构体
fn unit_structs() {
    println!("7. 单元结构体:");
    
    // 定义单元结构体
    struct AlwaysEqual;
    struct Empty;
    struct Unit;
    
    // 创建单元结构体实例
    let subject = AlwaysEqual;
    let empty = Empty;
    let unit = Unit;
    
    println!("   单元结构体实例已创建");
    
    // 单元结构体的方法
    impl AlwaysEqual {
        fn new() -> AlwaysEqual {
            AlwaysEqual
        }
        
        fn is_equal(&self, _other: &AlwaysEqual) -> bool {
            true  // 总是返回true
        }
    }
    
    impl Empty {
        fn do_nothing(&self) {
            println!("   空结构体什么都不做");
        }
    }
    
    impl Unit {
        fn value(&self) -> () {
            ()
        }
    }
    
    let subject2 = AlwaysEqual::new();
    println!("   两个AlwaysEqual是否相等: {}", subject.is_equal(&subject2));
    
    empty.do_nothing();
    println!("   Unit结构体的值: {:?}", unit.value());
    
    // 单元结构体在特征实现中的用途
    struct Counter;
    
    impl Counter {
        fn new() -> Counter {
            Counter
        }
        
        fn count(&self, items: &[i32]) -> usize {
            items.len()
        }
    }
    
    let counter = Counter::new();
    let numbers = vec![1, 2, 3, 4, 5];
    println!("   计数器统计结果: {}", counter.count(&numbers));
    
    println!();
}

// 8. 结构体所有权和借用
fn struct_ownership() {
    println!("8. 结构体所有权和借用:");
    
    struct Book {
        title: String,
        author: String,
        pages: u32,
    }
    
    // 创建结构体
    let book1 = Book {
        title: String::from("Rust编程"),
        author: String::from("张三"),
        pages: 300,
    };
    
    println!("   原始书籍: {} 作者: {}", book1.title, book1.author);
    
    // 借用结构体
    fn print_book_info(book: &Book) {
        println!("   书籍信息: {} 作者: {} 页数: {}", 
                 book.title, book.author, book.pages);
    }
    
    print_book_info(&book1);
    
    // 可变借用
    let mut book2 = Book {
        title: String::from("Python编程"),
        author: String::from("李四"),
        pages: 250,
    };
    
    fn update_book_pages(book: &mut Book, new_pages: u32) {
        book.pages = new_pages;
        println!("   更新页数: {} 页", book.pages);
    }
    
    update_book_pages(&mut book2, 280);
    
    // 结构体字段的所有权
    struct Library {
        books: Vec<Book>,
        name: String,
    }
    
    impl Library {
        fn new(name: String) -> Library {
            Library {
                books: Vec::new(),
                name,
            }
        }
        
        fn add_book(&mut self, book: Book) {
            self.books.push(book);
        }
        
        fn get_book_count(&self) -> usize {
            self.books.len()
        }
        
        fn find_book_by_title(&self, title: &str) -> Option<&Book> {
            self.books.iter().find(|book| book.title == title)
        }
    }
    
    let mut library = Library::new(String::from("我的图书馆"));
    
    // 移动所有权到图书馆
    library.add_book(book1);
    library.add_book(book2);
    
    println!("   图书馆: {} 共有 {} 本书", library.name, library.get_book_count());
    
    // 查找书籍（借用）
    if let Some(book) = library.find_book_by_title("Rust编程") {
        println!("   找到书籍: {} 作者: {}", book.title, book.author);
    }
    
    println!();
}

// 9. 结构体实现特征（trait）
fn struct_traits() {
    println!("9. 结构体实现特征（trait）:");
    
    // 定义特征
    trait Printable {
        fn print(&self);
        fn get_info(&self) -> String;
    }
    
    trait Movable {
        fn move_to(&mut self, x: f64, y: f64);
        fn get_position(&self) -> (f64, f64);
    }
    
    trait Drawable {
        fn draw(&self);
    }
    
    // 结构体
    struct Shape {
        x: f64,
        y: f64,
        name: String,
    }
    
    // 为Shape实现Printable特征
    impl Printable for Shape {
        fn print(&self) {
            println!("   形状: {} 位置: ({}, {})", self.name, self.x, self.y);
        }
        
        fn get_info(&self) -> String {
            format!("{} at ({}, {})", self.name, self.x, self.y)
        }
    }
    
    // 为Shape实现Movable特征
    impl Movable for Shape {
        fn move_to(&mut self, x: f64, y: f64) {
            self.x = x;
            self.y = y;
        }
        
        fn get_position(&self) -> (f64, f64) {
            (self.x, self.y)
        }
    }
    
    // 为Shape实现Drawable特征
    impl Drawable for Shape {
        fn draw(&self) {
            println!("   绘制形状: {}", self.name);
        }
    }
    
    let mut shape = Shape {
        x: 0.0,
        y: 0.0,
        name: String::from("圆形"),
    };
    
    // 使用特征方法
    shape.print();
    println!("   信息: {}", shape.get_info());
    
    shape.move_to(10.0, 20.0);
    let (x, y) = shape.get_position();
    println!("   移动后位置: ({}, {})", x, y);
    
    shape.draw();
    
    // 特征对象
    fn print_any_printable(item: &dyn Printable) {
        item.print();
    }
    
    fn move_any_movable(item: &mut dyn Movable, x: f64, y: f64) {
        item.move_to(x, y);
    }
    
    print_any_printable(&shape);
    move_any_movable(&mut shape, 50.0, 60.0);
    let (x, y) = shape.get_position();
    println!("   通过特征对象移动后位置: ({}, {})", x, y);
    
    println!();
}

// 10. 高级结构体操作
fn advanced_structs() {
    println!("10. 高级结构体操作:");
    
    // 泛型结构体
    struct Pair<T, U> {
        first: T,
        second: U,
    }
    
    impl<T, U> Pair<T, U> {
        fn new(first: T, second: U) -> Pair<T, U> {
            Pair { first, second }
        }
        
        fn get_first(&self) -> &T {
            &self.first
        }
        
        fn get_second(&self) -> &U {
            &self.second
        }
    }
    
    let int_pair = Pair::new(1, 2);
    let mixed_pair = Pair::new("Hello", 42);
    
    println!("   整数对: ({}, {})", int_pair.get_first(), int_pair.get_second());
    println!("   混合对: ({}, {})", mixed_pair.get_first(), mixed_pair.get_second());
    
    // 嵌套结构体
    struct Address {
        street: String,
        city: String,
        country: String,
    }
    
    struct Employee {
        name: String,
        age: u32,
        address: Address,
        skills: Vec<String>,
    }
    
    let employee = Employee {
        name: String::from("王五"),
        age: 28,
        address: Address {
            street: String::from("中关村大街"),
            city: String::from("北京"),
            country: String::from("中国"),
        },
        skills: vec![String::from("Rust"), String::from("Python"), String::from("JavaScript")],
    };
    
    println!("   员工: {} ({}岁)", employee.name, employee.age);
    println!("   地址: {}, {}, {}", employee.address.street, employee.address.city, employee.address.country);
    println!("   技能: {}", employee.skills.join(", "));
    
    // 结构体实现Clone和Debug
    #[derive(Clone, Debug)]
    struct Point3D {
        x: f64,
        y: f64,
        z: f64,
    }
    
    let point1 = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    let point2 = point1.clone();
    
    println!("   点1: {:?}", point1);
    println!("   点2 (克隆): {:?}", point2);
    
    // 结构体实现PartialEq和Eq
    #[derive(PartialEq, Eq, Debug)]
    struct SimplePoint {
        x: i32,
        y: i32,
    }
    
    let sp1 = SimplePoint { x: 1, y: 2 };
    let sp2 = SimplePoint { x: 1, y: 2 };
    let sp3 = SimplePoint { x: 2, y: 1 };
    
    println!("   sp1 == sp2: {}", sp1 == sp2);
    println!("   sp1 == sp3: {}", sp1 == sp3);
    
    // 结构体实现Default
    #[derive(Debug)]
    struct Config {
        host: String,
        port: u16,
        timeout: u64,
    }
    
    // 自定义Default实现
    impl Default for Config {
        fn default() -> Config {
            Config {
                host: String::from("localhost"),
                port: 8080,
                timeout: 30,
            }
        }
    }
    
    let default_config = Config::default();
    println!("   默认配置: {:?}", default_config);
    
    // 另一个配置结构体，使用派生Default
    #[derive(Default, Debug)]
    struct SimpleConfig {
        host: String,
        port: u16,
    }
    
    let simple_config = SimpleConfig::default();
    println!("   简单配置: {:?}", simple_config);
    
    println!();
}

// 主函数结束
fn main_end() {
    println!("=== 学习完成！===");
    println!("这些示例涵盖了 Rust 中结构体的主要概念:");
    println!("- 基本结构体定义和使用");
    println!("- 结构体字段访问和修改");
    println!("- 结构体方法定义");
    println!("- 关联函数（静态方法）");
    println!("- 结构体更新语法");
    println!("- 元组结构体");
    println!("- 单元结构体");
    println!("- 结构体所有权和借用");
    println!("- 结构体实现特征（trait）");
    println!("- 高级结构体操作");
    println!();
    println!("你可以运行 'cargo run' 来查看所有示例的执行结果。");
    println!("尝试修改参数和返回值来更好地理解这些概念！");
}
