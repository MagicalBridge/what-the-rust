fn main() {
    println!("=== Rust 枚举学习项目 ===\n");

    // 1. 基本枚举定义和使用
    basic_enum_demo();
    
    // 2. 带数据的枚举
    enum_with_data_demo();
    
    // 3. 枚举模式匹配
    enum_pattern_matching_demo();
    
    // 4. 枚举方法实现
    enum_methods_demo();
    
    // 5. 枚举和Option类型
    option_enum_demo();
    
    // 6. 枚举和Result类型
    result_enum_demo();
    
    // 7. 枚举和集合
    enum_collections_demo();
    
    // 8. 枚举和错误处理
    enum_error_handling_demo();
    
    // 9. 枚举和状态机
    enum_state_machine_demo();
    
    // 10. 高级枚举用法
    advanced_enum_demo();
    
    // 总结
    main_end();
}

// 1. 基本枚举定义和使用
fn basic_enum_demo() {
    println!("1. 基本枚举定义和使用:");
    
    // 定义基本枚举
    #[derive(Debug, Clone)]
    enum Direction {
        North,
        South,
        East,
        West,
    }
    
    // 使用枚举
    let current_direction = Direction::North;
    println!("   当前方向: {:?}", current_direction);
    
    // 枚举比较
    let directions = vec![Direction::North, Direction::South, Direction::East, Direction::West];
    println!("   所有方向:");
    for (i, dir) in directions.iter().enumerate() {
        println!("     {}: {:?}", i + 1, dir);
    }
    
    // 枚举作为函数参数
    fn get_direction_name(direction: Direction) -> &'static str {
        match direction {
            Direction::North => "北",
            Direction::South => "南",
            Direction::East => "东",
            Direction::West => "西",
        }
    }
    
    println!("   方向名称:");
    for dir in &directions {
        println!("     {:?} -> {}", dir, get_direction_name(dir.clone()));
    }
    
    // 枚举作为返回值
    fn get_opposite_direction(direction: Direction) -> Direction {
        match direction {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    
    println!("   相反方向:");
    for dir in &directions {
        let opposite = get_opposite_direction(dir.clone());
        println!("     {:?} 的相反方向是 {:?}", dir, opposite);
    }
    
    println!();
}

// 2. 带数据的枚举
fn enum_with_data_demo() {
    println!("2. 带数据的枚举:");
    
    // 定义带数据的枚举
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    // 创建不同类型的消息
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, World!")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    println!("   消息类型:");
    for (i, msg) in messages.iter().enumerate() {
        println!("     {}: {:?}", i + 1, msg);
    }
    
    // 处理带数据的枚举
    fn process_message(message: &Message) {
        match message {
            Message::Quit => {
                println!("     收到退出消息");
            }
            Message::Move { x, y } => {
                println!("     移动到坐标: ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("     写入文本: '{}'", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("     改变颜色为: RGB({}, {}, {})", r, g, b);
            }
        }
    }
    
    println!("   处理消息:");
    for msg in &messages {
        process_message(msg);
    }
    
    // 带数据的枚举在结构体中的使用
    #[derive(Debug)]
    struct GameState {
        player_position: (i32, i32),
        current_action: Option<Message>,
    }
    
    let mut game = GameState {
        player_position: (0, 0),
        current_action: Some(Message::Move { x: 5, y: 5 }),
    };
    
    println!("   游戏状态: {:?}", game);
    
    // 更新游戏状态
    if let Some(Message::Move { x, y }) = game.current_action {
        game.player_position = (x, y);
        println!("   玩家移动到: {:?}", game.player_position);
    }
    
    println!();
}

// 3. 枚举模式匹配
fn enum_pattern_matching_demo() {
    println!("3. 枚举模式匹配:");
    
    #[derive(Debug)]
    enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
        Triangle(f64, f64, f64),
    }
    
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    
    // 基本模式匹配
    fn calculate_area(shape: &Shape) -> f64 {
        match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                // 使用海伦公式计算三角形面积
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
    
    println!("   形状面积计算:");
    for shape in &shapes {
        let area = calculate_area(shape);
        println!("     {:?} -> 面积: {:.2}", shape, area);
    }
    
    // 使用 if let 进行模式匹配
    fn describe_shape(shape: &Shape) {
        if let Shape::Circle(radius) = shape {
            println!("     这是一个半径为 {:.1} 的圆形", radius);
        } else if let Shape::Rectangle(width, height) = shape {
            println!("     这是一个 {}x{} 的矩形", width, height);
        } else if let Shape::Triangle(a, b, c) = shape {
            println!("     这是一个三边长为 {:.1}, {:.1}, {:.1} 的三角形", a, b, c);
        }
    }
    
    println!("   形状描述:");
    for shape in &shapes {
        describe_shape(shape);
    }
    
    // 使用 match 进行复杂模式匹配
    fn analyze_shape(shape: &Shape) {
        match shape {
            Shape::Circle(radius) => {
                if *radius > 10.0 {
                    println!("     这是一个大圆形");
                } else if *radius > 5.0 {
                    println!("     这是一个中等圆形");
                } else {
                    println!("     这是一个小圆形");
                }
            }
            Shape::Rectangle(width, height) => {
                if (width - height).abs() < 0.1 {
                    println!("     这是一个正方形");
                } else {
                    println!("     这是一个矩形");
                }
            }
            Shape::Triangle(a, b, c) => {
                if (a - b).abs() < 0.1 && (b - c).abs() < 0.1 {
                    println!("     这是一个等边三角形");
                } else if (a - b).abs() < 0.1 || (b - c).abs() < 0.1 || (a - c).abs() < 0.1 {
                    println!("     这是一个等腰三角形");
                } else {
                    println!("     这是一个不等边三角形");
                }
            }
        }
    }
    
    println!("   形状分析:");
    for shape in &shapes {
        analyze_shape(shape);
    }
    
    println!();
}

// 4. 枚举方法实现
fn enum_methods_demo() {
    println!("4. 枚举方法实现:");
    
    #[derive(Debug)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }
    
    impl TrafficLight {
        // 基本方法
        fn color(&self) -> &'static str {
            match self {
                TrafficLight::Red => "红色",
                TrafficLight::Yellow => "黄色",
                TrafficLight::Green => "绿色",
            }
        }
        
        // 返回下一个状态
        fn next(&self) -> TrafficLight {
            match self {
                TrafficLight::Red => TrafficLight::Green,
                TrafficLight::Yellow => TrafficLight::Red,
                TrafficLight::Green => TrafficLight::Yellow,
            }
        }
        
        // 检查是否可以通行
        fn can_go(&self) -> bool {
            matches!(self, TrafficLight::Green)
        }
        
        // 获取等待时间（秒）
        fn wait_time(&self) -> u32 {
            match self {
                TrafficLight::Red => 30,
                TrafficLight::Yellow => 3,
                TrafficLight::Green => 25,
            }
        }
        
        // 静态方法
        fn new() -> TrafficLight {
            TrafficLight::Red
        }
        
        // 从字符串创建
        fn from_str(s: &str) -> Option<TrafficLight> {
            match s.to_lowercase().as_str() {
                "red" | "红色" => Some(TrafficLight::Red),
                "yellow" | "黄色" => Some(TrafficLight::Yellow),
                "green" | "绿色" => Some(TrafficLight::Green),
                _ => None,
            }
        }
    }
    
    // 测试枚举方法
    let lights = vec![TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];
    
    println!("   交通灯信息:");
    for light in &lights {
        println!("     {:?}: 颜色={}, 可通行={}, 等待时间={}秒", 
                light, light.color(), light.can_go(), light.wait_time());
    }
    
    println!("   交通灯状态转换:");
    let mut current = TrafficLight::new();
    for i in 0..6 {
        println!("     步骤 {}: {:?} ({})", i, current, current.color());
        current = current.next();
    }
    
    // 测试从字符串创建
    let color_strings = vec!["red", "yellow", "green", "蓝色"];
    println!("   从字符串创建交通灯:");
    for color in color_strings {
        match TrafficLight::from_str(color) {
            Some(light) => println!("     '{}' -> {:?}", color, light),
            None => println!("     '{}' -> 无效颜色", color),
        }
    }
    
    println!();
}

// 5. 枚举和Option类型
fn option_enum_demo() {
    println!("5. 枚举和Option类型:");
    
    // Option 是 Rust 标准库中的枚举
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    
    // 创建 Option 值
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;
    
    println!("   Option 值:");
    println!("     some_number: {:?}", some_number);
    println!("     no_number: {:?}", no_number);
    
    // 使用 match 处理 Option
    fn process_option(opt: Option<i32>) {
        match opt {
            Some(value) => println!("     有值: {}", value),
            None => println!("     没有值"),
        }
    }
    
    println!("   处理 Option:");
    process_option(some_number);
    process_option(no_number);
    
    // 使用 if let 处理 Option
    fn process_option_if_let(opt: Option<i32>) {
        if let Some(value) = opt {
            println!("     使用 if let 处理: {}", value);
        } else {
            println!("     使用 if let 处理: 没有值");
        }
    }
    
    println!("   使用 if let 处理:");
    process_option_if_let(some_number);
    process_option_if_let(no_number);
    
    // Option 的常用方法
    let numbers = vec![Some(1), None, Some(3), Some(4)];
    
    println!("   Option 方法演示:");
    for (i, opt) in numbers.iter().enumerate() {
        println!("     索引 {}:", i);
        println!("       原始值: {:?}", opt);
        println!("       是否为 Some: {}", opt.is_some());
        println!("       是否为 None: {}", opt.is_none());
        println!("       使用 unwrap_or: {}", opt.unwrap_or(0));
        println!("       使用 map: {:?}", opt.map(|x| x * 2));
        println!("       使用 and_then: {:?}", opt.and_then(|x| if x > 2 { Some(x * 2) } else { None }));
    }
    
    // 安全的 unwrap 方法
    fn safe_unwrap(opt: Option<i32>) -> i32 {
        opt.unwrap_or_else(|| {
            println!("     警告: 使用默认值 0");
            0
        })
    }
    
    println!("   安全解包:");
    println!("     safe_unwrap(Some(42)) = {}", safe_unwrap(Some(42)));
    println!("     safe_unwrap(None) = {}", safe_unwrap(None));
    
    // Option 在函数返回值中的使用
    fn find_number(numbers: &[i32], target: i32) -> Option<usize> {
        for (i, &num) in numbers.iter().enumerate() {
            if num == target {
                return Some(i);
            }
        }
        None
    }
    
    let numbers = vec![1, 3, 5, 7, 9];
    let targets = vec![3, 6, 9];
    
    println!("   在数组中查找数字:");
    for target in targets {
        match find_number(&numbers, target) {
            Some(index) => println!("     找到 {} 在索引 {}", target, index),
            None => println!("     没有找到 {}", target),
        }
    }
    
    println!();
}

// 6. 枚举和Result类型
fn result_enum_demo() {
    println!("6. 枚举和Result类型:");
    
    // Result 是 Rust 标准库中的枚举
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    
    // 定义自定义错误类型
    #[derive(Debug, Clone)]
    enum MathError {
        DivisionByZero,
        NegativeNumber,
        Overflow,
    }
    
    // 创建 Result 值
    let success_result: Result<i32, MathError> = Ok(42);
    let error_result: Result<i32, MathError> = Err(MathError::DivisionByZero);
    
    println!("   Result 值:");
    println!("     success_result: {:?}", success_result);
    println!("     error_result: {:?}", error_result);
    
    // 使用 match 处理 Result
    fn process_result(result: Result<i32, MathError>) {
        match result {
            Ok(value) => println!("     成功: {}", value),
            Err(error) => println!("     错误: {:?}", error),
        }
    }
    
    println!("   处理 Result:");
    process_result(success_result);
    process_result(error_result);
    
    // 数学运算函数返回 Result
    fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
        if b == 0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
    
    fn safe_sqrt(n: i32) -> Result<f64, MathError> {
        if n < 0 {
            Err(MathError::NegativeNumber)
        } else {
            Ok((n as f64).sqrt())
        }
    }
    
    fn safe_multiply(a: i32, b: i32) -> Result<i32, MathError> {
        a.checked_mul(b).ok_or(MathError::Overflow)
    }
    
    // 测试数学运算
    let operations = vec![
        (10, 2, "除法"),
        (10, 0, "除零"),
        (16, 1, "平方根"),
        (-4, 1, "负数平方根"),
        (1000, 1000, "乘法"),
        (i32::MAX, 2, "溢出乘法"),
    ];
    
    println!("   数学运算结果:");
    for (a, b, op) in operations {
        match op {
            "除法" => {
                let result = safe_divide(a, b);
                println!("     {} / {} = {:?}", a, b, result);
            }
            "除零" => {
                let result = safe_divide(a, b);
                println!("     {} / {} = {:?}", a, b, result);
            }
            "平方根" => {
                let result = safe_sqrt(a);
                println!("     sqrt({}) = {:?}", a, result);
            }
            "负数平方根" => {
                let result = safe_sqrt(a);
                println!("     sqrt({}) = {:?}", a, result);
            }
            "乘法" => {
                let result = safe_multiply(a, b);
                println!("     {} * {} = {:?}", a, b, result);
            }
            "溢出乘法" => {
                let result = safe_multiply(a, b);
                println!("     {} * {} = {:?}", a, b, result);
            }
            _ => {}
        }
    }
    
    // Result 的常用方法
    println!("   Result 方法演示:");
    
    let results = vec![
        Ok(42),
        Err(MathError::DivisionByZero),
        Ok(100),
    ];
    
    for result in results {
        println!("     原始值: {:?}", result);
        println!("       是否为 Ok: {}", result.is_ok());
        println!("       是否为 Err: {}", result.is_err());
        println!("       使用 unwrap_or: {}", result.clone().unwrap_or(0));
        println!("       使用 map: {:?}", result.clone().map(|x| x * 2));
        println!("       使用 map_err: {:?}", result.map_err(|_| "自定义错误"));
    }
    
    // 错误传播
    fn complex_calculation(a: i32, b: i32) -> Result<f64, MathError> {
        // 使用 ? 操作符进行错误传播
        let division = safe_divide(a, b)?;
        let sqrt_result = safe_sqrt(division)?;
        Ok(sqrt_result)
    }
    
    println!("   复杂计算:");
    let test_cases = vec![(16, 4), (16, 0), (-16, 4)];
    
    for (a, b) in test_cases {
        match complex_calculation(a, b) {
            Ok(result) => println!("     complex_calculation({}, {}) = {:.2}", a, b, result),
            Err(error) => println!("     complex_calculation({}, {}) = 错误: {:?}", a, b, error),
        }
    }
    
    println!();
}

// 7. 枚举和集合
fn enum_collections_demo() {
    println!("7. 枚举和集合:");
    
    #[derive(Debug, Clone)]
    enum Item {
        Book(String),
        Tool(String),
        Food(String),
        Treasure(i32),
    }
    
    // 创建包含枚举的向量
    let inventory = vec![
        Item::Book("Rust编程指南".to_string()),
        Item::Tool("锤子".to_string()),
        Item::Food("苹果".to_string()),
        Item::Treasure(100),
        Item::Book("算法导论".to_string()),
        Item::Tool("螺丝刀".to_string()),
    ];
    
    println!("   背包物品:");
    for (i, item) in inventory.iter().enumerate() {
        println!("     {}: {:?}", i + 1, item);
    }
    
    // 按类型统计物品
    fn count_by_type(items: &[Item]) -> (usize, usize, usize, usize) {
        let mut books = 0;
        let mut tools = 0;
        let mut foods = 0;
        let mut treasures = 0;
        
        for item in items {
            match item {
                Item::Book(_) => books += 1,
                Item::Tool(_) => tools += 1,
                Item::Food(_) => foods += 1,
                Item::Treasure(_) => treasures += 1,
            }
        }
        
        (books, tools, foods, treasures)
    }
    
    let (books, tools, foods, treasures) = count_by_type(&inventory);
    println!("   物品统计:");
    println!("     书籍: {} 本", books);
    println!("     工具: {} 个", tools);
    println!("     食物: {} 个", foods);
    println!("     宝藏: {} 个", treasures);
    
    // 过滤特定类型的物品
    fn filter_books(items: &[Item]) -> Vec<&Item> {
        items.iter().filter(|item| matches!(item, Item::Book(_))).collect()
    }
    
    fn filter_treasures(items: &[Item]) -> Vec<&Item> {
        items.iter().filter(|item| matches!(item, Item::Treasure(_))).collect()
    }
    
    println!("   书籍列表:");
    for book in filter_books(&inventory) {
        println!("     {:?}", book);
    }
    
    println!("   宝藏列表:");
    for treasure in filter_treasures(&inventory) {
        println!("     {:?}", treasure);
    }
    
    // 使用 HashMap 存储枚举
    use std::collections::HashMap;
    
    #[derive(Debug, Hash, Eq, PartialEq)]
    enum Category {
        Technology,
        Literature,
        Science,
        History,
    }
    
    let mut category_map: HashMap<Category, Vec<String>> = HashMap::new();
    
    category_map.insert(Category::Technology, vec!["Rust".to_string(), "Python".to_string()]);
    category_map.insert(Category::Literature, vec!["小说".to_string(), "诗歌".to_string()]);
    category_map.insert(Category::Science, vec!["物理".to_string(), "化学".to_string()]);
    
    println!("   分类映射:");
    for (category, items) in &category_map {
        println!("     {:?}: {:?}", category, items);
    }
    
    // 使用 HashSet 存储枚举
    use std::collections::HashSet;
    
    #[derive(Debug, Hash, Eq, PartialEq)]
    enum Permission {
        Read,
        Write,
        Execute,
        Delete,
    }
    
    let mut user_permissions: HashSet<Permission> = HashSet::new();
    user_permissions.insert(Permission::Read);
    user_permissions.insert(Permission::Write);
    
    println!("   用户权限:");
    for permission in &user_permissions {
        println!("     {:?}", permission);
    }
    
    // 检查权限
    fn has_permission(permissions: &HashSet<Permission>, required: Permission) -> bool {
        permissions.contains(&required)
    }
    
    println!("   权限检查:");
    println!("     有读取权限: {}", has_permission(&user_permissions, Permission::Read));
    println!("     有执行权限: {}", has_permission(&user_permissions, Permission::Execute));
    
    println!();
}

// 8. 枚举和错误处理
fn enum_error_handling_demo() {
    println!("8. 枚举和错误处理:");
    
    // 定义自定义错误类型
    #[derive(Debug)]
    enum DatabaseError {
        ConnectionFailed(String),
        QueryFailed(String),
        DataNotFound,
        InvalidData(String),
        PermissionDenied,
    }
    
    // 实现 Display trait
    impl std::fmt::Display for DatabaseError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                DatabaseError::ConnectionFailed(msg) => write!(f, "连接失败: {}", msg),
                DatabaseError::QueryFailed(msg) => write!(f, "查询失败: {}", msg),
                DatabaseError::DataNotFound => write!(f, "数据未找到"),
                DatabaseError::InvalidData(msg) => write!(f, "无效数据: {}", msg),
                DatabaseError::PermissionDenied => write!(f, "权限被拒绝"),
            }
        }
    }
    
    // 模拟数据库操作
    fn connect_database(url: &str) -> Result<(), DatabaseError> {
        if url.is_empty() {
            Err(DatabaseError::ConnectionFailed("URL不能为空".to_string()))
        } else if url.contains("invalid") {
            Err(DatabaseError::ConnectionFailed("无效的URL".to_string()))
        } else {
            println!("     成功连接到数据库: {}", url);
            Ok(())
        }
    }
    
    fn query_user(id: i32) -> Result<String, DatabaseError> {
        match id {
            1 => Ok("Alice".to_string()),
            2 => Ok("Bob".to_string()),
            3 => Err(DatabaseError::DataNotFound),
            _ => Err(DatabaseError::QueryFailed("无效的用户ID".to_string())),
        }
    }
    
    fn update_user(id: i32, data: &str) -> Result<(), DatabaseError> {
        if data.is_empty() {
            Err(DatabaseError::InvalidData("数据不能为空".to_string()))
        } else if id == 999 {
            Err(DatabaseError::PermissionDenied)
        } else {
            println!("     成功更新用户 {}: {}", id, data);
            Ok(())
        }
    }
    
    // 测试数据库操作
    println!("   数据库连接测试:");
    let urls = vec!["localhost:5432", "", "invalid://url"];
    
    for url in urls {
        match connect_database(url) {
            Ok(()) => println!("     连接成功"),
            Err(e) => println!("     连接失败: {}", e),
        }
    }
    
    println!("   用户查询测试:");
    let user_ids = vec![1, 2, 3, 999];
    
    for id in user_ids {
        match query_user(id) {
            Ok(name) => println!("     用户 {}: {}", id, name),
            Err(e) => println!("     查询用户 {} 失败: {}", id, e),
        }
    }
    
    println!("   用户更新测试:");
    let updates = vec![(1, "Alice Smith"), (2, ""), (999, "Test")];
    
    for (id, data) in updates {
        match update_user(id, data) {
            Ok(()) => println!("     更新成功"),
            Err(e) => println!("     更新失败: {}", e),
        }
    }
    
    // 错误转换
    #[derive(Debug)]
    enum AppError {
        Database(DatabaseError),
        Network(String),
        Validation(String),
    }
    
    impl From<DatabaseError> for AppError {
        fn from(error: DatabaseError) -> Self {
            AppError::Database(error)
        }
    }
    
    // 高级错误处理函数
    fn process_user_request(user_id: i32, action: &str) -> Result<String, AppError> {
        // 连接数据库
        connect_database("localhost:5432")?;
        
        match action {
            "query" => {
                let user = query_user(user_id)?;
                Ok(format!("查询结果: {}", user))
            }
            "update" => {
                update_user(user_id, "新数据")?;
                Ok("更新成功".to_string())
            }
            _ => Err(AppError::Validation("无效的操作".to_string())),
        }
    }
    
    println!("   高级错误处理:");
    let requests = vec![(1, "query"), (2, "update"), (999, "invalid")];
    
    for (user_id, action) in requests {
        match process_user_request(user_id, action) {
            Ok(result) => println!("     请求成功: {}", result),
            Err(e) => println!("     请求失败: {:?}", e),
        }
    }
    
    println!();
}

// 9. 枚举和状态机
fn enum_state_machine_demo() {
    println!("9. 枚举和状态机:");
    
    // 定义状态机状态
    #[derive(Debug, Clone)]
    enum VendingMachineState {
        Idle,
        WaitingForMoney { item: String, price: u32 },
        Dispensing { item: String },
        OutOfStock,
    }
    
    // 定义事件
    #[derive(Debug)]
    enum VendingMachineEvent {
        SelectItem { item: String, price: u32 },
        InsertMoney { amount: u32 },
        Cancel,
        Dispense,
    }
    
    // 自动售货机
    struct VendingMachine {
        state: VendingMachineState,
        money_inserted: u32,
        inventory: std::collections::HashMap<String, u32>,
    }
    
    impl VendingMachine {
        fn new() -> Self {
            let mut inventory = std::collections::HashMap::new();
            inventory.insert("可乐".to_string(), 5);
            inventory.insert("薯片".to_string(), 3);
            inventory.insert("巧克力".to_string(), 2);
            
            Self {
                state: VendingMachineState::Idle,
                money_inserted: 0,
                inventory,
            }
        }
        
        fn handle_event(&mut self, event: VendingMachineEvent) -> Result<String, String> {
            match (self.state.clone(), event) {
                (VendingMachineState::Idle, VendingMachineEvent::SelectItem { item, price }) => {
                    if let Some(&stock) = self.inventory.get(&item) {
                        if stock > 0 {
                            self.state = VendingMachineState::WaitingForMoney { item: item.clone(), price };
                            Ok(format!("请投入 {} 元购买 {}", price, item))
                        } else {
                            self.state = VendingMachineState::OutOfStock;
                            Err("商品缺货".to_string())
                        }
                    } else {
                        Err("商品不存在".to_string())
                    }
                }
                
                (VendingMachineState::WaitingForMoney { item, price }, VendingMachineEvent::InsertMoney { amount }) => {
                    self.money_inserted += amount;
                    if self.money_inserted >= price {
                        self.state = VendingMachineState::Dispensing { item: item.clone() };
                        Ok(format!("投入足够金额，正在出货 {}", item))
                    } else {
                        let remaining = price - self.money_inserted;
                        Ok(format!("还需要投入 {} 元", remaining))
                    }
                }
                
                (VendingMachineState::WaitingForMoney { .. }, VendingMachineEvent::Cancel) => {
                    let refund = self.money_inserted;
                    self.money_inserted = 0;
                    self.state = VendingMachineState::Idle;
                    Ok(format!("取消购买，退还 {} 元", refund))
                }
                
                (VendingMachineState::Dispensing { item }, VendingMachineEvent::Dispense) => {
                    // 更新库存
                    if let Some(stock) = self.inventory.get_mut(&item) {
                        *stock -= 1;
                    }
                    
                    let change = self.money_inserted - 5; // 假设价格是5元
                    self.money_inserted = 0;
                    self.state = VendingMachineState::Idle;
                    
                    if change > 0 {
                        Ok(format!("出货完成，找零 {} 元", change))
                    } else {
                        Ok("出货完成".to_string())
                    }
                }
                
                (VendingMachineState::OutOfStock, _) => {
                    Err("机器缺货，请联系管理员".to_string())
                }
                
                _ => {
                    Err("无效的操作".to_string())
                }
            }
        }
        
        fn get_state(&self) -> &VendingMachineState {
            &self.state
        }
        
        fn get_inventory(&self) -> &std::collections::HashMap<String, u32> {
            &self.inventory
        }
    }
    
    // 测试状态机
    let mut machine = VendingMachine::new();
    
    println!("   自动售货机状态机演示:");
    println!("   初始状态: {:?}", machine.get_state());
    println!("   初始库存: {:?}", machine.get_inventory());
    
    // 测试正常购买流程
    println!("   正常购买流程:");
    
    // 1. 选择商品
    match machine.handle_event(VendingMachineEvent::SelectItem { 
        item: "可乐".to_string(), 
        price: 5 
    }) {
        Ok(msg) => println!("     {}", msg),
        Err(e) => println!("     错误: {}", e),
    }
    println!("     当前状态: {:?}", machine.get_state());
    
    // 2. 投入金钱
    match machine.handle_event(VendingMachineEvent::InsertMoney { amount: 3 }) {
        Ok(msg) => println!("     {}", msg),
        Err(e) => println!("     错误: {}", e),
    }
    println!("     当前状态: {:?}", machine.get_state());
    
    // 3. 继续投入金钱
    match machine.handle_event(VendingMachineEvent::InsertMoney { amount: 2 }) {
        Ok(msg) => println!("     {}", msg),
        Err(e) => println!("     错误: {}", e),
    }
    println!("     当前状态: {:?}", machine.get_state());
    
    // 4. 出货
    match machine.handle_event(VendingMachineEvent::Dispense) {
        Ok(msg) => println!("     {}", msg),
        Err(e) => println!("     错误: {}", e),
    }
    println!("     当前状态: {:?}", machine.get_state());
    println!("     剩余库存: {:?}", machine.get_inventory());
    
    // 测试取消购买
    println!("   取消购买流程:");
    match machine.handle_event(VendingMachineEvent::SelectItem { 
        item: "薯片".to_string(), 
        price: 3 
    }) {
        Ok(msg) => println!("     {}", msg),
        Err(e) => println!("     错误: {}", e),
    }
    
    match machine.handle_event(VendingMachineEvent::InsertMoney { amount: 1 }) {
        Ok(msg) => println!("     {}", msg),
        Err(e) => println!("     错误: {}", e),
    }
    
    match machine.handle_event(VendingMachineEvent::Cancel) {
        Ok(msg) => println!("     {}", msg),
        Err(e) => println!("     错误: {}", e),
    }
    println!("     当前状态: {:?}", machine.get_state());
    
    println!();
}

// 10. 高级枚举用法
fn advanced_enum_demo() {
    println!("10. 高级枚举用法:");
    
    // 递归枚举
    #[derive(Debug)]
    enum Expression {
        Number(i32),
        Add(Box<Expression>, Box<Expression>),
        Subtract(Box<Expression>, Box<Expression>),
        Multiply(Box<Expression>, Box<Expression>),
        Divide(Box<Expression>, Box<Expression>),
    }
    
    impl Expression {
        fn evaluate(&self) -> Result<i32, String> {
            match self {
                Expression::Number(n) => Ok(*n),
                Expression::Add(a, b) => {
                    Ok(a.evaluate()? + b.evaluate()?)
                }
                Expression::Subtract(a, b) => {
                    Ok(a.evaluate()? - b.evaluate()?)
                }
                Expression::Multiply(a, b) => {
                    Ok(a.evaluate()? * b.evaluate()?)
                }
                Expression::Divide(a, b) => {
                    let b_val = b.evaluate()?;
                    if b_val == 0 {
                        Err("除零错误".to_string())
                    } else {
                        Ok(a.evaluate()? / b_val)
                    }
                }
            }
        }
        
        fn to_string(&self) -> String {
            match self {
                Expression::Number(n) => n.to_string(),
                Expression::Add(a, b) => format!("({} + {})", a.to_string(), b.to_string()),
                Expression::Subtract(a, b) => format!("({} - {})", a.to_string(), b.to_string()),
                Expression::Multiply(a, b) => format!("({} * {})", a.to_string(), b.to_string()),
                Expression::Divide(a, b) => format!("({} / {})", a.to_string(), b.to_string()),
            }
        }
    }
    
    // 测试表达式求值
    println!("   表达式求值:");
    
    // 创建表达式: (2 + 3)
    let expr1 = Expression::Add(
        Box::new(Expression::Number(2)),
        Box::new(Expression::Number(3))
    );
    
    // 创建表达式: ((2 + 3) * 4)
    let expr2 = Expression::Multiply(
        Box::new(Expression::Add(
            Box::new(Expression::Number(2)),
            Box::new(Expression::Number(3))
        )),
        Box::new(Expression::Number(4))
    );
    
    // 创建表达式: (10 / (2 + 3))
    let expr3 = Expression::Divide(
        Box::new(Expression::Number(10)),
        Box::new(Expression::Add(
            Box::new(Expression::Number(2)),
            Box::new(Expression::Number(3))
        ))
    );
    
    let expressions = vec![expr1, expr2, expr3];
    
    for expr in expressions {
        println!("     表达式: {}", expr.to_string());
        match expr.evaluate() {
            Ok(result) => println!("     结果: {}", result),
            Err(e) => println!("     错误: {}", e),
        }
    }
    
    // 枚举实现 trait
    #[derive(Debug, Clone, PartialEq)]
    enum Color {
        Red,
        Green,
        Blue,
        RGB(u8, u8, u8),
    }
    
    impl Color {
        fn to_hex(&self) -> String {
            match self {
                Color::Red => "#FF0000".to_string(),
                Color::Green => "#00FF00".to_string(),
                Color::Blue => "#0000FF".to_string(),
                Color::RGB(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b),
            }
        }
        
        fn brightness(&self) -> u8 {
            match self {
                Color::Red => 255,
                Color::Green => 255,
                Color::Blue => 255,
                Color::RGB(r, g, b) => {
                    // 计算亮度 (0.299*R + 0.587*G + 0.114*B)
                    (*r as f32 * 0.299 + *g as f32 * 0.587 + *b as f32 * 0.114) as u8
                }
            }
        }
    }
    
    // 实现自定义 trait
    trait Colorful {
        fn is_dark(&self) -> bool;
        fn mix(&self, other: &Color) -> Color;
    }
    
    impl Colorful for Color {
        fn is_dark(&self) -> bool {
            self.brightness() < 128
        }
        
        fn mix(&self, other: &Color) -> Color {
            match (self, other) {
                (Color::RGB(r1, g1, b1), Color::RGB(r2, g2, b2)) => {
                    Color::RGB(
                        ((*r1 as u16 + *r2 as u16) / 2) as u8,
                        ((*g1 as u16 + *g2 as u16) / 2) as u8,
                        ((*b1 as u16 + *b2 as u16) / 2) as u8,
                    )
                }
                _ => Color::RGB(128, 128, 128), // 默认灰色
            }
        }
    }
    
    println!("   颜色处理:");
    let colors = vec![
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::RGB(100, 150, 200),
        Color::RGB(50, 50, 50),
    ];
    
    for color in &colors {
        println!("     {:?}:", color);
        println!("       十六进制: {}", color.to_hex());
        println!("       亮度: {}", color.brightness());
        println!("       是否暗色: {}", color.is_dark());
    }
    
    // 颜色混合
    if let (Some(color1), Some(color2)) = (colors.get(0), colors.get(3)) {
        let mixed = color1.mix(color2);
        println!("     颜色混合: {:?} + {:?} = {:?}", color1, color2, mixed);
    }
    
    // 枚举和迭代器
    #[derive(Debug)]
    enum Tree<T> {
        Empty,
        Node(T, Box<Tree<T>>, Box<Tree<T>>),
    }
    
    impl<T> Tree<T> {
        fn new() -> Self {
            Tree::Empty
        }
        
        fn insert(self, value: T) -> Self 
        where T: Ord + Clone
        {
            match self {
                Tree::Empty => Tree::Node(value, Box::new(Tree::Empty), Box::new(Tree::Empty)),
                Tree::Node(v, left, right) => {
                    if value < v {
                        Tree::Node(v, Box::new(left.insert(value)), right)
                    } else {
                        Tree::Node(v, left, Box::new(right.insert(value)))
                    }
                }
            }
        }
        
        fn inorder(&self) -> Vec<&T> {
            match self {
                Tree::Empty => vec![],
                Tree::Node(value, left, right) => {
                    let mut result = left.inorder();
                    result.push(value);
                    result.extend(right.inorder());
                    result
                }
            }
        }
    }
    
    println!("   二叉树演示:");
    let mut tree = Tree::new();
    tree = tree.insert(5);
    tree = tree.insert(3);
    tree = tree.insert(7);
    tree = tree.insert(1);
    tree = tree.insert(9);
    
    println!("     中序遍历: {:?}", tree.inorder());
    
    // 枚举和泛型
    #[derive(Debug)]
    enum Container<T> {
        Empty,
        Single(T),
        Multiple(Vec<T>),
    }
    
    impl<T: Clone> Container<T> {
        fn new() -> Self {
            Container::Empty
        }
        
        fn add(&mut self, item: T) {
            match self {
                Container::Empty => *self = Container::Single(item),
                Container::Single(existing) => {
                    let items = vec![existing.clone(), item];
                    *self = Container::Multiple(items);
                }
                Container::Multiple(items) => {
                    items.push(item);
                }
            }
        }
        
        fn len(&self) -> usize {
            match self {
                Container::Empty => 0,
                Container::Single(_) => 1,
                Container::Multiple(items) => items.len(),
            }
        }
        
        fn is_empty(&self) -> bool {
            matches!(self, Container::Empty)
        }
    }
    
    println!("   泛型容器演示:");
    let mut container: Container<i32> = Container::new();
    println!("     初始状态: {:?}, 长度: {}", container, container.len());
    
    container.add(42);
    println!("     添加一个元素: {:?}, 长度: {}", container, container.len());
    
    container.add(100);
    println!("     添加第二个元素: {:?}, 长度: {}", container, container.len());
    
    container.add(200);
    println!("     添加第三个元素: {:?}, 长度: {}", container, container.len());
    
    // 枚举和生命周期
    #[derive(Debug)]
    enum Reference<'a> {
        Owned(String),
        Borrowed(&'a str),
    }
    
    impl<'a> Reference<'a> {
        fn as_str(&self) -> &str {
            match self {
                Reference::Owned(s) => s,
                Reference::Borrowed(s) => s,
            }
        }
        
        fn to_owned(self) -> String {
            match self {
                Reference::Owned(s) => s,
                Reference::Borrowed(s) => s.to_string(),
            }
        }
    }
    
    println!("   引用枚举演示:");
    let owned = Reference::Owned("Hello".to_string());
    let borrowed = Reference::Borrowed("World");
    
    println!("     拥有的引用: {:?}", owned);
    println!("     借用的引用: {:?}", borrowed);
    println!("     转换为字符串: '{}'", owned.as_str());
    println!("     转换为字符串: '{}'", borrowed.as_str());
    
    println!();
}

// 主函数结束
fn main_end() {
    println!("=== 学习完成！===");
    println!("这些示例涵盖了 Rust 中枚举的主要概念:");
    println!("- 基本枚举定义和使用");
    println!("- 带数据的枚举");
    println!("- 枚举模式匹配");
    println!("- 枚举方法实现");
    println!("- 枚举和Option类型");
    println!("- 枚举和Result类型");
    println!("- 枚举和集合");
    println!("- 枚举和错误处理");
    println!("- 枚举和状态机");
    println!("- 高级枚举用法");
    println!();
    println!("你可以运行 'cargo run' 来查看所有示例的执行结果。");
    println!("尝试修改参数和返回值来更好地理解这些概念！");
}