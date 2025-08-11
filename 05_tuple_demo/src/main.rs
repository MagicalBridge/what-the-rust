use std::collections::HashMap;

// 元组宏
macro_rules! tuple {
    ($($x:expr),*) => {
        ($($x),*)
    };
}

fn main() {
    println!("=== Rust 元组类型学习项目 ===\n");

    // 1. 基本元组定义和访问
    basic_tuple_definition();
    
    // 2. 元组解构
    tuple_destructuring();
    
    // 3. 元组作为函数参数和返回值
    tuple_as_parameters_and_return();
    
    // 4. 元组比较和操作
    tuple_comparison_and_operations();
    
    // 5. 元组与结构体
    tuple_structs();
    
    // 6. 元组在集合中的应用
    tuples_in_collections();
    
    // 7. 元组类型别名
    tuple_type_aliases();
    
    // 8. 元组与迭代器
    tuples_with_iterators();
    
    // 9. 元组在错误处理中的应用
    tuples_in_error_handling();
    
    // 10. 高级元组技巧
    advanced_tuple_tricks();
    
    // 总结
    main_end();
}

// 1. 基本元组定义和访问
fn basic_tuple_definition() {
    println!("1. 基本元组定义和访问:");
    
    // 创建元组
    let tuple1 = (1, 2, 3);
    let tuple2 = ("hello", 42, true);
    let tuple3 = (3.14, "pi", 22);
    
    println!("   元组1: {:?}", tuple1);
    println!("   元组2: {:?}", tuple2);
    println!("   元组3: {:?}", tuple3);
    
    // 访问元组元素
    println!("   元组1的第一个元素: {}", tuple1.0);
    println!("   元组1的第二个元素: {}", tuple1.1);
    println!("   元组1的第三个元素: {}", tuple1.2);
    
    // 类型注解
    let tuple4: (i32, f64, &str) = (10, 3.14, "test");
    println!("   带类型注解的元组: {:?}", tuple4);
    
    // 空元组 (单元类型)
    let empty_tuple = ();
    println!("   空元组: {:?}", empty_tuple);
    
    // 单元素元组 (注意逗号)
    let single_element = (42,);
    println!("   单元素元组: {:?}", single_element);
    
    println!();
}

// 2. 元组解构
fn tuple_destructuring() {
    println!("2. 元组解构:");
    
    // 基本解构
    let point = (10, 20);
    let (x, y) = point;
    println!("   点坐标: x = {}, y = {}", x, y);
    
    // 忽略某些元素
    let rgb = (255, 128, 64);
    let (red, _, blue) = rgb;
    println!("   RGB颜色: 红 = {}, 蓝 = {}", red, blue);
    
    // 部分解构
    let person = ("Alice", 25, "Engineer");
    let (name, age, _) = person;
    println!("   人员信息: 姓名 = {}, 年龄 = {}", name, age);
    
    // 嵌套元组解构
    let complex = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = complex;
    println!("   嵌套元组: a = {}, b = {}, c = {}, d = {}", a, b, c, d);
    
    // 在函数参数中解构
    let coordinates = [(1, 2), (3, 4), (5, 6)];
    for (x, y) in coordinates.iter() {
        println!("   坐标点: ({}, {})", x, y);
    }
    
    println!();
}

// 3. 元组作为函数参数和返回值
fn tuple_as_parameters_and_return() {
    println!("3. 元组作为函数参数和返回值:");
    
    // 元组作为函数参数
    let point1 = (3.0, 4.0);
    let point2 = (6.0, 8.0);
    let distance = calculate_distance(point1, point2);
    println!("   两点距离: {}", distance);
    
    // 函数返回元组
    let numbers = [1, 2, 3, 4, 5];
    let (min, max, sum) = get_stats(&numbers);
    println!("   数组统计: 最小值 = {}, 最大值 = {}, 总和 = {}", min, max, sum);
    
    // 多值返回
    let dividend = 17;
    let divisor = 5;
    let (quotient, remainder) = divide_with_remainder(dividend, divisor);
    println!("   {} ÷ {} = {} 余 {}", dividend, divisor, quotient, remainder);
    
    // 返回元组和错误信息
    let result = safe_divide(10, 2);
    match result {
        Ok((quotient, remainder)) => {
            println!("   安全除法: 商 = {}, 余数 = {}", quotient, remainder);
        }
        Err(e) => println!("   错误: {}", e),
    }
    
    println!();
}

// 计算两点距离
fn calculate_distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    (dx * dx + dy * dy).sqrt()
}

// 获取数组统计信息
fn get_stats(numbers: &[i32]) -> (i32, i32, i32) {
    let min = numbers.iter().min().unwrap_or(&0);
    let max = numbers.iter().max().unwrap_or(&0);
    let sum = numbers.iter().sum();
    (*min, *max, sum)
}

// 带余数的除法
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

// 安全除法
fn safe_divide(dividend: i32, divisor: i32) -> Result<(i32, i32), String> {
    if divisor == 0 {
        Err("除数不能为零".to_string())
    } else {
        let quotient = dividend / divisor;
        let remainder = dividend % divisor;
        Ok((quotient, remainder))
    }
}

// 4. 元组比较和操作
fn tuple_comparison_and_operations() {
    println!("4. 元组比较和操作:");
    
    // 元组比较
    let tuple1 = (1, 2, 3);
    let tuple2 = (1, 2, 3);
    let tuple3 = (1, 2, 4);
    
    println!("   tuple1 == tuple2: {}", tuple1 == tuple2);
    println!("   tuple1 == tuple3: {}", tuple1 == tuple3);
    println!("   tuple1 < tuple3: {}", tuple1 < tuple3);
    
    // 元组嵌套
    let nested = ((1, 2), (3, 4));
    println!("   嵌套元组: {:?}", nested);
    println!("   访问嵌套元素: {:?}", nested.0);
    
    // 元组数组
    let points = [(1, 2), (3, 4), (5, 6)];
    println!("   元组数组: {:?}", points);
    
    // 元组向量
    let mut tuple_vec = vec![(1, "one"), (2, "two"), (3, "three")];
    tuple_vec.push((4, "four"));
    println!("   元组向量: {:?}", tuple_vec);
    
    // 元组排序
    let mut scores = [(85, "Alice"), (92, "Bob"), (78, "Charlie")];
    scores.sort_by(|a, b| b.0.cmp(&a.0)); // 按分数降序
    println!("   排序后的成绩: {:?}", scores);
    
    println!();
}

// 5. 元组与结构体
fn tuple_structs() {
    println!("5. 元组与结构体:");
    
    // 定义元组结构体
    struct Point(i32, i32);
    struct Color(u8, u8, u8);
    struct Person(String, u32, String);
    
    // 创建实例
    let point = Point(10, 20);
    let color = Color(255, 128, 64);
    let person = Person("张三".to_string(), 25, "工程师".to_string());
    
    println!("   点: ({}, {})", point.0, point.1);
    println!("   颜色: RGB({}, {}, {})", color.0, color.1, color.2);
    println!("   人员: {} - {}岁 - {}", person.0, person.1, person.2);
    
    // 元组结构体解构
    let Point(x, y) = point;
    let Color(r, g, b) = color;
    let Person(name, age, job) = person;
    
    println!("   解构后 - 点: ({}, {})", x, y);
    println!("   解构后 - 颜色: RGB({}, {}, {})", r, g, b);
    println!("   解构后 - 人员: {} - {}岁 - {}", name, age, job);
    
    // 为元组结构体实现方法
    impl Point {
        fn new(x: i32, y: i32) -> Point {
            Point(x, y)
        }
        
        fn distance_to_origin(&self) -> f64 {
            ((self.0 * self.0 + self.1 * self.1) as f64).sqrt()
        }
        
        fn add(&self, other: &Point) -> Point {
            Point(self.0 + other.0, self.1 + other.1)
        }
    }
    
    let p1 = Point::new(3, 4);
    let p2 = Point::new(1, 2);
    let p3 = p1.add(&p2);
    
    println!("   点1到原点距离: {:.2}", p1.distance_to_origin());
    println!("   点1 + 点2 = ({}, {})", p3.0, p3.1);
    
    println!();
}

// 6. 元组在集合中的应用
fn tuples_in_collections() {
    println!("6. 元组在集合中的应用:");
    
    // 元组向量
    let mut students = vec![
        ("Alice", 85),
        ("Bob", 92),
        ("Charlie", 78),
        ("David", 96),
    ];
    
    println!("   学生成绩: {:?}", students);
    
    // 添加新学生
    students.push(("Eve", 88));
    println!("   添加后: {:?}", students);
    
    // 查找最高分
    if let Some((name, score)) = students.iter().max_by_key(|&(_, score)| score) {
        println!("   最高分: {} - {}", name, score);
    }
    
    // HashMap中的元组
    let mut scores = HashMap::new();
    scores.insert("Alice", (85, "A"));
    scores.insert("Bob", (92, "A"));
    scores.insert("Charlie", (78, "C"));
    
    println!("   成绩映射: {:?}", scores);
    
    // 遍历HashMap中的元组
    for (name, (score, grade)) in &scores {
        println!("   {}: {}分 ({})", name, score, grade);
    }
    
    // 元组数组
    let coordinates = [(1, 2), (3, 4), (5, 6), (7, 8)];
    let distances: Vec<f64> = coordinates
        .iter()
        .map(|&(x, y)| ((x * x + y * y) as f64).sqrt())
        .collect();
    
    println!("   坐标到原点距离: {:?}", distances);
    
    println!();
}

// 7. 元组类型别名
fn tuple_type_aliases() {
    println!("7. 元组类型别名:");
    
    // 定义类型别名
    type Point2D = (f64, f64);
    type RGB = (u8, u8, u8);
    type PersonInfo = (String, u32, String);
    type DatabaseRecord = (i32, String, String, bool);
    
    // 使用类型别名
    let point: Point2D = (3.14, 2.71);
    let color: RGB = (255, 128, 64);
    let person: PersonInfo = ("李四".to_string(), 30, "设计师".to_string());
    let record: DatabaseRecord = (1, "用户1".to_string(), "user1@example.com".to_string(), true);
    
    println!("   2D点: {:?}", point);
    println!("   RGB颜色: {:?}", color);
    println!("   人员信息: {:?}", person);
    println!("   数据库记录: {:?}", record);
    
    // 函数使用类型别名
    let distance = calculate_distance_2d(point, (0.0, 0.0));
    println!("   到原点距离: {:.2}", distance);
    
    // 复杂类型别名
    type Point3D = (f64, f64, f64);
    type Line = (Point2D, Point2D);
    type Triangle = (Point2D, Point2D, Point2D);
    
    let line: Line = ((0.0, 0.0), (3.0, 4.0));
    let triangle: Triangle = ((0.0, 0.0), (3.0, 0.0), (0.0, 4.0));
    
    println!("   线段: {:?}", line);
    println!("   三角形: {:?}", triangle);
    
    println!();
}

// 计算2D点距离
fn calculate_distance_2d(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    (dx * dx + dy * dy).sqrt()
}

// 8. 元组与迭代器
fn tuples_with_iterators() {
    println!("8. 元组与迭代器:");
    
    // enumerate - 获取索引和值
    let fruits = ["苹果", "香蕉", "橙子", "葡萄"];
    println!("   水果列表:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("     {}: {}", index, fruit);
    }
    
    // zip - 组合两个迭代器
    let names = ["Alice", "Bob", "Charlie"];
    let ages = [25, 30, 35];
    println!("   人员信息:");
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("     {}: {}岁", name, age);
    }
    
    // 元组迭代器
    let points = [(1, 2), (3, 4), (5, 6)];
    let distances: Vec<f64> = points
        .iter()
        .map(|&(x, y)| ((x * x + y * y) as f64).sqrt())
        .collect();
    
    println!("   各点到原点距离: {:?}", distances);
    
    // 过滤和映射
    let scores = [85, 92, 78, 96, 88, 75];
    let high_scores: Vec<(usize, i32)> = scores
        .iter()
        .enumerate()
        .filter(|&(_, &score)| score >= 85)
        .map(|(index, &score)| (index, score))
        .collect();
    
    println!("   高分学生 (>=85): {:?}", high_scores);
    
    // 折叠操作
    let numbers = [1, 2, 3, 4, 5];
    let (sum, count) = numbers
        .iter()
        .fold((0, 0), |(sum, count), &num| (sum + num, count + 1));
    
    println!("   总和: {}, 数量: {}", sum, count);
    
    println!();
}

// 9. 元组在错误处理中的应用
fn tuples_in_error_handling() {
    println!("9. 元组在错误处理中的应用:");
    
    // Result类型本身就是元组
    let result1: Result<i32, String> = Ok(42);
    let result2: Result<i32, String> = Err("发生错误".to_string());
    
    match result1 {
        Ok(value) => println!("   成功: {}", value),
        Err(e) => println!("   错误: {}", e),
    }
    
    match result2 {
        Ok(value) => println!("   成功: {}", value),
        Err(e) => println!("   错误: {}", e),
    }
    
    // Option类型
    let option1: Option<i32> = Some(100);
    let option2: Option<i32> = None;
    
    match option1 {
        Some(value) => println!("   有值: {}", value),
        None => println!("   无值"),
    }
    
    match option2 {
        Some(value) => println!("   有值: {}", value),
        None => println!("   无值"),
    }
    
    // 返回多个错误信息
    let validation_result = validate_user("", -5);
    match validation_result {
        Ok(user) => println!("   用户验证成功: {:?}", user),
        Err((field, message)) => println!("   验证失败 - {}: {}", field, message),
    }
    
    // 链式错误处理
    let operation_result = perform_operation(10, 0);
    match operation_result {
        Ok((result, message)) => println!("   操作成功: {} - {}", result, message),
        Err((error_code, error_msg)) => println!("   操作失败: {} - {}", error_code, error_msg),
    }
    
    println!();
}

// 用户验证函数
fn validate_user(name: &str, age: i32) -> Result<(String, i32), (String, String)> {
    if name.is_empty() {
        return Err(("姓名".to_string(), "姓名不能为空".to_string()));
    }
    if age < 0 || age > 150 {
        return Err(("年龄".to_string(), "年龄必须在0-150之间".to_string()));
    }
    Ok((name.to_string(), age))
}

// 执行操作函数
fn perform_operation(a: i32, b: i32) -> Result<(i32, String), (i32, String)> {
    if b == 0 {
        return Err((1001, "除数不能为零".to_string()));
    }
    let result = a / b;
    Ok((result, "操作成功完成".to_string()))
}

// 10. 高级元组技巧
fn advanced_tuple_tricks() {
    println!("10. 高级元组技巧:");
    
    // 元组函数
    let add = |a: i32, b: i32| (a + b, "加法".to_string());
    let multiply = |a: i32, b: i32| (a * b, "乘法".to_string());
    
    let (sum, op1) = add(5, 3);
    let (product, op2) = multiply(4, 6);
    
    println!("   {}: {}", op1, sum);
    println!("   {}: {}", op2, product);
    
    // 元组宏
    let tuple_macro = tuple!(1, "hello", true);
    println!("   元组宏结果: {:?}", tuple_macro);
    
    // 元组转换
    let array = [1, 2, 3, 4, 5];
    let tuple_from_array = convert_array_to_tuple(&array);
    println!("   数组转元组: {:?}", tuple_from_array);
    
    // 元组切片（通过解构）
    let long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    let (first, second, third, ..) = long_tuple;
    println!("   前三个元素: {}, {}, {}", first, second, third);
    
    // 条件元组
    let condition = true;
    let conditional_tuple = if condition {
        (1, "true")
    } else {
        (0, "false")
    };
    println!("   条件元组: {:?}", conditional_tuple);
    
    // 元组作为键（需要实现Hash和Eq）
    let mut tuple_map = HashMap::new();
    tuple_map.insert((1, "one"), "第一个");
    tuple_map.insert((2, "two"), "第二个");
    
    println!("   元组作为键的映射:");
    for ((num, word), meaning) in &tuple_map {
        println!("     ({}, '{}') -> {}", num, word, meaning);
    }
    
    println!();
}

// 数组转元组函数（这里展示5个元素的转换）
fn convert_array_to_tuple(arr: &[i32; 5]) -> (i32, i32, i32, i32, i32) {
    (arr[0], arr[1], arr[2], arr[3], arr[4])
}

// 总结函数
fn main_end() {
    println!("=== 学习总结 ===");
    println!("通过这个demo，你已经学习了:");
    println!("✅ 元组的基本定义和访问");
    println!("✅ 元组解构和模式匹配");
    println!("✅ 元组作为函数参数和返回值");
    println!("✅ 元组比较和操作");
    println!("✅ 元组结构体");
    println!("✅ 元组在集合中的应用");
    println!("✅ 元组类型别名");
    println!("✅ 元组与迭代器");
    println!("✅ 元组在错误处理中的应用");
    println!("✅ 高级元组技巧");
    println!();
    println!("🎉 恭喜你完成了Rust元组类型的学习！");
    println!("继续探索Rust的其他特性吧！");
}
