fn main() {
    println!("=== Rust 函数学习项目 ===\n");

    // 1. 基本函数定义和调用
    basic_functions();
    
    // 2. 函数参数
    function_parameters();
    
    // 3. 函数返回值
    function_return_values();
    
    // 4. 函数类型
    function_types();
    
    // 5. 闭包 (Closures)
    closures_demo();
    
    // 6. 高阶函数
    higher_order_functions();
    
    // 7. 方法 (Methods)
    methods_demo();
    
    // 8. 关联函数 (Associated Functions)
    associated_functions();
    
    // 9. 函数指针
    function_pointers();
    
    // 10. 递归函数
    recursive_functions();
    
    // 总结
    main_end();
}

// 1. 基本函数定义和调用
fn basic_functions() {
    println!("1. 基本函数定义和调用:");
    
    // 调用无参数函数
    greet();
    
    // 调用有参数函数
    greet_person("Alice");
    greet_person("Bob");
    
    // 调用有多个参数的函数
    print_sum(10, 20);
    
    println!();
}

// 无参数函数
fn greet() {
    println!("   你好，Rust!");
}

// 有参数函数
fn greet_person(name: &str) {
    println!("   你好，{}!", name);
}

// 多个参数函数
fn print_sum(a: i32, b: i32) {
    println!("   {} + {} = {}", a, b, a + b);
}

// 2. 函数参数
fn function_parameters() {
    println!("2. 函数参数:");
    
    // 值传递
    let x = 5;
    pass_by_value(x);
    println!("   原始值: x = {}", x);
    
    // 引用传递
    let mut y = 10;
    pass_by_reference(&mut y);
    println!("   修改后的值: y = {}", y);
    
    // 不可变引用
    let z = 15;
    pass_by_immutable_reference(&z);
    println!("   原始值: z = {}", z);
    
    // 数组参数
    let numbers = [1, 2, 3, 4, 5];
    print_array(&numbers);
    
    // 可变数组参数
    let mut scores = [85, 92, 78, 96, 88];
    add_bonus(&mut scores, 5);
    println!("   加分后的成绩: {:?}", scores);
    
    println!();
}

fn pass_by_value(value: i32) {
    println!("   值传递: 收到值 {}", value);
    // 这里无法修改原始值
}

fn pass_by_reference(value: &mut i32) {
    println!("   引用传递: 原始值 {}", *value);
    *value += 5;
    println!("   引用传递: 修改后 {}", *value);
}

fn pass_by_immutable_reference(value: &i32) {
    println!("   不可变引用: 值 {}", *value);
    // 无法修改值
}

fn print_array(arr: &[i32]) {
    println!("   数组内容: {:?}", arr);
}

fn add_bonus(scores: &mut [i32], bonus: i32) {
    for score in scores.iter_mut() {
        *score += bonus;
    }
}

// 3. 函数返回值
fn function_return_values() {
    println!("3. 函数返回值:");
    
    // 基本返回值
    let result = add(5, 3);
    println!("   5 + 3 = {}", result);
    
    // 多个返回值 (使用元组)
    let (sum, product) = calculate(4, 6);
    println!("   4 和 6 的和: {}, 积: {}", sum, product);
    
    // 条件返回值
    let max = get_max(10, 20);
    println!("   10 和 20 中的最大值: {}", max);
    
    // 提前返回
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("   10 / 2 = {}", value),
        Err(e) => println!("   错误: {}", e),
    }
    
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("   10 / 0 = {}", value),
        Err(e) => println!("   错误: {}", e),
    }
    
    println!();
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // 隐式返回
}

fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)  // 返回元组
}

fn get_max(a: i32, b: i32) -> i32 {
    if a > b {
        a  // 返回 a
    } else {
        b  // 返回 b
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("除数不能为零".to_string());  // 提前返回错误
    }
    Ok(a / b)  // 返回成功结果
}

// 4. 函数类型
fn function_types() {
    println!("4. 函数类型:");
    
    // 函数作为变量
    let func: fn(i32, i32) -> i32 = multiply;
    let result = func(3, 4);
    println!("   函数变量调用: 3 * 4 = {}", result);
    
    // 函数数组
    let operations: [fn(i32, i32) -> i32; 3] = [add, subtract, multiply];
    let a = 10;
    let b = 5;
    
    println!("   函数数组调用:");
    println!("   {} + {} = {}", a, b, operations[0](a, b));
    println!("   {} - {} = {}", a, b, operations[1](a, b));
    println!("   {} * {} = {}", a, b, operations[2](a, b));
    
    // 函数作为参数
    apply_operation(15, 3, add);
    apply_operation(15, 3, multiply);
    
    println!();
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn apply_operation(a: i32, b: i32, operation: fn(i32, i32) -> i32) {
    let result = operation(a, b);
    println!("   应用操作: {}({}, {}) = {}", 
             std::any::type_name_of_val(&operation), a, b, result);
}

// 5. 闭包 (Closures)
fn closures_demo() {
    println!("5. 闭包 (Closures):");
    
    // 基本闭包
    let add_one = |x: i32| x + 1;
    println!("   基本闭包: add_one(5) = {}", add_one(5));
    
    // 捕获环境变量
    let factor = 3;
    let multiply_by_factor = |x: i32| x * factor;
    println!("   捕获环境变量: multiply_by_factor(4) = {}", multiply_by_factor(4));
    
    // 可变闭包
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        counter
    };
    
    println!("   可变闭包:");
    println!("   第一次调用: {}", increment());
    println!("   第二次调用: {}", increment());
    println!("   第三次调用: {}", increment());
    
    // 移动语义
    let numbers = vec![1, 2, 3, 4, 5];
    let print_numbers = move || {
        println!("   移动闭包中的数字: {:?}", numbers);
    };
    print_numbers();
    // numbers 在这里已经不可用了
    
    // 闭包作为参数
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("   使用闭包映射: {:?}", doubled);
    
    println!();
}

// 6. 高阶函数
fn higher_order_functions() {
    println!("6. 高阶函数:");
    
    // 返回函数的函数
    let operation = get_operation("add");
    let result = operation(10, 5);
    println!("   高阶函数返回加法: {}", result);
    
    let operation = get_operation("multiply");
    let result = operation(10, 5);
    println!("   高阶函数返回乘法: {}", result);
    
    // 函数组合
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    let combined = compose(add_one, double);
    let result = combined(5);
    println!("   函数组合: (add_one ∘ double)(5) = {}", result);
    
    println!();
}

fn get_operation(op: &str) -> fn(i32, i32) -> i32 {
    match op {
        "add" => add,
        "multiply" => multiply,
        "subtract" => subtract,
        _ => add,
    }
}

fn compose<F, G, T>(f: F, g: G) -> impl Fn(T) -> T
where
    F: Fn(T) -> T + 'static,
    G: Fn(T) -> T + 'static,
    T: Copy + 'static,
{
    move |x| f(g(x))
}

// 7. 方法 (Methods)
fn methods_demo() {
    println!("7. 方法 (Methods):");
    
    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    
    println!("   矩形: {:?}", rect);
    println!("   面积: {}", rect.area());
    println!("   周长: {}", rect.perimeter());
    println!("   是否为正方形: {}", rect.is_square());
    
    let mut rect = Rectangle {
        width: 5,
        height: 15,
    };
    rect.resize(2.0);
    println!("   调整大小后: {:?}", rect);
    
    // 关联函数调用
    let square = Rectangle::square(10);
    println!("   创建正方形: {:?}", square);
    
    println!();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 实例方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    // 可变引用方法
    fn resize(&mut self, factor: f64) {
        self.width = (self.width as f64 * factor) as u32;
        self.height = (self.height as f64 * factor) as u32;
    }
    
    // 关联函数 (类似静态方法)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 8. 关联函数 (Associated Functions)
fn associated_functions() {
    println!("8. 关联函数 (Associated Functions):");
    
    // 调用关联函数
    let point = Point::new(3.0, 4.0);
    println!("   创建点: {:?}", point);
    
    let distance = Point::distance_between(&point, &Point::new(0.0, 0.0));
    println!("   到原点的距离: {:.2}", distance);
    
    let origin = Point::origin();
    println!("   原点: {:?}", origin);
    
    println!();
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // 构造函数
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    
    // 原点
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    
    // 计算两点间距离
    fn distance_between(p1: &Point, p2: &Point) -> f64 {
        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// 9. 函数指针
fn function_pointers() {
    println!("9. 函数指针:");
    
    // 函数指针类型
    let func_ptr: fn(i32) -> i32 = square;
    let result = func_ptr(5);
    println!("   函数指针调用: square(5) = {}", result);
    
    // 函数指针数组
    let functions: [fn(i32) -> i32; 3] = [square, cube, double];
    let x = 3;
    
    println!("   函数指针数组:");
    for (i, func) in functions.iter().enumerate() {
        let result = func(x);
        let names = ["square", "cube", "double"];
        println!("   {}({}) = {}", names[i], x, result);
    }
    
    // 函数指针作为参数
    apply_function(4, square);
    apply_function(4, cube);
    
    println!();
}

fn square(x: i32) -> i32 {
    x * x
}

fn cube(x: i32) -> i32 {
    x * x * x
}

fn double(x: i32) -> i32 {
    x * 2
}

fn apply_function(x: i32, f: fn(i32) -> i32) {
    let result = f(x);
    println!("   应用函数: f({}) = {}", x, result);
}

// 10. 递归函数
fn recursive_functions() {
    println!("10. 递归函数:");
    
    // 阶乘
    let n = 5;
    let factorial_result = factorial(n);
    println!("   阶乘: {}! = {}", n, factorial_result);
    
    // 斐波那契数列
    let n = 10;
    let fib_result = fibonacci(n);
    println!("   斐波那契数列: F({}) = {}", n, fib_result);
    
    // 递归打印
    println!("   递归打印数字:");
    print_numbers_recursive(5);
    
    // 尾递归优化示例
    let n = 20; // 使用较小的数字避免溢出
    let factorial_tail = factorial_tail_recursive(n, 1);
    println!("   尾递归阶乘: {}! = {}", n, factorial_tail);
    
    println!();
}

fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn print_numbers_recursive(n: i32) {
    if n > 0 {
        print!("   {}", n);
        if n > 1 {
            print!(" -> ");
        }
        print_numbers_recursive(n - 1);
    } else {
        println!();
    }
}

fn factorial_tail_recursive(n: u64, acc: u64) -> u64 {
    if n <= 1 {
        acc
    } else {
        factorial_tail_recursive(n - 1, n * acc)
    }
}

// 主函数结束
fn main_end() {
    println!("=== 学习完成！===");
    println!("这些示例涵盖了 Rust 中函数的主要概念:");
    println!("- 基本函数定义和调用");
    println!("- 函数参数（值传递、引用传递）");
    println!("- 函数返回值");
    println!("- 函数类型和函数指针");
    println!("- 闭包和高阶函数");
    println!("- 方法和关联函数");
    println!("- 递归函数");
    println!();
    println!("你可以运行 'cargo run' 来查看所有示例的执行结果。");
    println!("尝试修改参数和返回值来更好地理解这些概念！");
}
