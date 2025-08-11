// Rust 切片（Slice）学习项目
// 本文件包含了从基础到高级的切片操作示例

fn main() {
    println!("=== Rust 切片（Slice）学习项目 ===\n");

    // 1. 切片基础概念
    basic_concepts();
    
    // 2. 切片语法和创建
    slice_syntax();
    
    // 3. 字符串切片操作
    string_slices();
    
    // 4. 数组和向量切片
    array_vector_slices();
    
    // 5. 切片迭代
    slice_iteration();
    
    // 6. 切片查找和搜索
    slice_search();
    
    // 7. 切片排序和操作
    slice_sorting();
    
    // 8. 切片函数参数
    slice_functions();
    
    // 9. 切片与集合类型
    slice_collections();
    
    // 10. 高级切片操作
    advanced_slice_operations();
    
    // 11. 切片安全性和错误处理
    slice_safety();
    
    // 12. 实际应用场景
    practical_applications();
}

// 1. 切片基础概念
fn basic_concepts() {
    println!("1. 切片基础概念");
    println!("{}", "=".repeat(30));
    
    // 字符串切片
    let s = String::from("Hello World");
    let slice = &s[0..5];
    println!("字符串: '{}'", s);
    println!("字符串切片 [0..5]: '{}'", slice);
    
    // 数组切片
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("数组: {:?}", arr);
    println!("数组切片 [1..4]: {:?}", slice);
    
    // 切片长度
    println!("切片长度: {}", slice.len());
    println!("切片是否为空: {}", slice.is_empty());
    
    println!();
}

// 2. 切片语法和创建
fn slice_syntax() {
    println!("2. 切片语法和创建");
    println!("{}", "=".repeat(30));
    
    let text = "Hello World";
    println!("原始字符串: '{}'", text);
    
    // 基本切片语法
    let slice1 = &text[0..5];
    println!("[0..5]: '{}'", slice1);
    
    // 省略起始索引
    let slice2 = &text[..5];
    println!("[..5]: '{}'", slice2);
    
    // 省略结束索引
    let slice3 = &text[6..];
    println!("[6..]: '{}'", slice3);
    
    // 完整范围
    let slice4 = &text[..];
    println!("[..]: '{}'", slice4);
    
    // 数组的不同切片方式
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("\n数组: {:?}", numbers);
    println!("[2..7]: {:?}", &numbers[2..7]);
    println!("[..5]: {:?}", &numbers[..5]);
    println!("[5..]: {:?}", &numbers[5..]);
    println!("[..]: {:?}", &numbers[..]);
    
    println!();
}

// 3. 字符串切片操作
fn string_slices() {
    println!("3. 字符串切片操作");
    println!("{}", "=".repeat(30));
    
    // 英文字符串切片
    let english = "Hello World";
    println!("英文字符串: '{}'", english);
    println!("前5个字符: '{}'", &english[0..5]);
    println!("后5个字符: '{}'", &english[6..]);
    
    // 中文字符串切片（按字节）
    let chinese = "你好世界";
    println!("\n中文字符串: '{}'", chinese);
    println!("前6个字节: '{}'", &chinese[0..6]); // "你好"
    println!("后6个字节: '{}'", &chinese[6..]);  // "世界"
    
    // 按字符切片（推荐方式）
    let chars: Vec<char> = chinese.chars().collect();
    println!("\n按字符切片:");
    println!("所有字符: {:?}", chars);
    println!("前2个字符: {:?}", &chars[0..2]);
    println!("后2个字符: {:?}", &chars[2..]);
    
    // 安全的字符串切片
    println!("\n安全的字符串切片:");
    if let Some(slice) = safe_string_slice(chinese, 0, 6) {
        println!("安全切片 [0..6]: '{}'", slice);
    }
    
    println!();
}

// 4. 数组和向量切片
fn array_vector_slices() {
    println!("4. 数组和向量切片");
    println!("{}", "=".repeat(30));
    
    // 整数数组切片
    let int_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("整数数组: {:?}", int_array);
    println!("中间部分 [3..7]: {:?}", &int_array[3..7]);
    
    // 浮点数数组切片
    let float_array = [1.1, 2.2, 3.3, 4.4, 5.5];
    println!("\n浮点数数组: {:?}", float_array);
    println!("前3个元素 [..3]: {:?}", &float_array[..3]);
    
    // 向量切片
    let vec = vec![10, 20, 30, 40, 50, 60, 70, 80];
    println!("\n向量: {:?}", vec);
    println!("向量切片 [2..6]: {:?}", &vec[2..6]);
    
    // 多维数组切片
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    println!("\n二维数组:");
    for row in &matrix {
        println!("行: {:?}", row);
    }
    println!("第一行切片 [0..2]: {:?}", &matrix[0][0..2]);
    
    println!();
}

// 5. 切片迭代
fn slice_iteration() {
    println!("5. 切片迭代");
    println!("{}", "=".repeat(30));
    
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &numbers[2..8];
    println!("切片: {:?}", slice);
    
    // 元素迭代
    println!("\n元素迭代:");
    for &num in slice {
        println!("数字: {}", num);
    }
    
    // 索引迭代
    println!("\n索引迭代:");
    for (i, &num) in slice.iter().enumerate() {
        println!("索引 {}: 值 {}", i, num);
    }
    
    // 反向迭代
    println!("\n反向迭代:");
    for &num in slice.iter().rev() {
        println!("反向数字: {}", num);
    }
    
    // 条件迭代
    println!("\n条件迭代（偶数）:");
    for &num in slice.iter().filter(|&&x| x % 2 == 0) {
        println!("偶数: {}", num);
    }
    
    // 映射迭代
    println!("\n映射迭代（平方）:");
    for num in slice.iter().map(|&x| x * x) {
        println!("平方: {}", num);
    }
    
    println!();
}

// 6. 切片查找和搜索
fn slice_search() {
    println!("6. 切片查找和搜索");
    println!("{}", "=".repeat(30));
    
    let numbers = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let slice = &numbers[2..8];
    println!("切片: {:?}", slice);
    
    // 线性搜索
    println!("\n线性搜索:");
    let target = 7;
    if let Some(pos) = slice.iter().position(|&x| x == target) {
        println!("找到 {}，位置: {}", target, pos);
    } else {
        println!("未找到 {}", target);
    }
    
    // 条件搜索
    println!("\n条件搜索（大于10的数）:");
    if let Some(pos) = slice.iter().position(|&x| x > 10) {
        println!("第一个大于10的数: {}，位置: {}", slice[pos], pos);
    }
    
    // 二分搜索（需要排序）
    let sorted_slice = &numbers[..];
    println!("\n二分搜索（在排序数组中）:");
    let search_value = 13;
    match sorted_slice.binary_search(&search_value) {
        Ok(pos) => println!("找到 {}，位置: {}", search_value, pos),
        Err(pos) => println!("未找到 {}，应该插入位置: {}", search_value, pos),
    }
    
    // 多值搜索
    println!("\n多值搜索:");
    let search_values = [5, 7, 15, 20];
    for &value in &search_values {
        if let Some(pos) = slice.iter().position(|&x| x == value) {
            println!("找到 {}，位置: {}", value, pos);
        } else {
            println!("未找到 {}", value);
        }
    }
    
    println!();
}

// 7. 切片排序和操作
fn slice_sorting() {
    println!("7. 切片排序和操作");
    println!("{}", "=".repeat(30));
    
    let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let slice = &mut numbers[1..9];
    println!("原始切片: {:?}", slice);
    
    // 排序
    slice.sort();
    println!("排序后: {:?}", slice);
    
    // 反转
    slice.reverse();
    println!("反转后: {:?}", slice);
    
    // 去重（需要先排序）
    slice.sort();
    let unique_count = slice.len();
    // 手动去重，因为切片没有dedup方法
    let mut unique_slice = Vec::new();
    for num in slice {
        if !unique_slice.contains(num) {
            unique_slice.push(*num);
        }
    }
    println!("去重后: {:?} (从{}个元素减少到{}个)", unique_slice, unique_count, unique_slice.len());
    
    // 部分排序
    let mut partial_numbers = [5, 2, 8, 1, 9, 3, 7, 4, 6];
    let partial_slice = &mut partial_numbers[1..7];
    println!("\n部分排序前: {:?}", partial_slice);
    partial_slice.sort();
    println!("部分排序后: {:?}", partial_slice);
    
    // 自定义排序
    let mut words = ["banana", "apple", "cherry", "date"];
    let word_slice = &mut words[..];
    println!("\n单词排序前: {:?}", word_slice);
    word_slice.sort_by(|a, b| a.len().cmp(&b.len())); // 按长度排序
    println!("按长度排序后: {:?}", word_slice);
    
    println!();
}

// 8. 切片函数参数
fn slice_functions() {
    println!("8. 切片函数参数");
    println!("{}", "=".repeat(30));
    
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &numbers[2..8];
    println!("切片: {:?}", slice);
    
    // 切片作为参数
    let sum = sum_slice(slice);
    println!("切片求和: {}", sum);
    
    // 可变切片参数
    let mut mutable_numbers = [1, 2, 3, 4, 5];
    let mutable_slice = &mut mutable_numbers[1..4];
    println!("\n可变切片修改前: {:?}", mutable_slice);
    double_slice(mutable_slice);
    println!("可变切片修改后: {:?}", mutable_slice);
    
    // 返回切片
    let middle = get_middle(slice);
    println!("\n中间部分: {:?}", middle);
    
    // 泛型切片函数
    let float_slice = &[1.1, 2.2, 3.3, 4.4, 5.5];
    let float_avg = average_slice(float_slice);
    println!("\n浮点数切片平均值: {:.2}", float_avg);
    
    println!();
}

// 9. 切片与集合类型
fn slice_collections() {
    println!("9. 切片与集合类型");
    println!("{}", "=".repeat(30));
    
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &vec[2..7];
    println!("向量: {:?}", vec);
    println!("向量切片: {:?}", slice);
    
    // 切片转向量
    let new_vec: Vec<i32> = slice.to_vec();
    println!("\n切片转向量: {:?}", new_vec);
    
    // 切片过滤
    let filtered: Vec<&i32> = slice.iter().filter(|&&x| x > 5).collect();
    println!("过滤后（大于5）: {:?}", filtered);
    
    // 切片映射
    let mapped: Vec<i32> = slice.iter().map(|&x| x * 2).collect();
    println!("映射后（乘以2）: {:?}", mapped);
    
    // 字符串切片操作
    let text = "Hello World Rust Programming";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("\n文本: '{}'", text);
    println!("单词: {:?}", words);
    
    // 字符串切片过滤
    let long_words: Vec<&str> = words.iter().filter(|&&word| word.len() > 4).copied().collect();
    println!("长单词（长度>4）: {:?}", long_words);
    
    println!();
}

// 10. 高级切片操作
fn advanced_slice_operations() {
    println!("10. 高级切片操作");
    println!("{}", "=".repeat(30));
    
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &numbers[..];
    println!("完整切片: {:?}", slice);
    
    // 窗口操作
    println!("\n窗口操作（窗口大小=3）:");
    for window in slice.windows(3) {
        println!("窗口: {:?}", window);
    }
    
    // 分块操作
    println!("\n分块操作（块大小=3）:");
    for chunk in slice.chunks(3) {
        println!("分块: {:?}", chunk);
    }
    
    // 分块操作（最后一块可能不完整）
    println!("\n分块操作（块大小=4）:");
    for chunk in slice.chunks(4) {
        println!("分块: {:?}", chunk);
    }
    
    // 重叠分块
    println!("\n重叠分块（块大小=3，步长=2）:");
    for chunk in slice.chunks_exact(3) {
        println!("分块: {:?}", chunk);
    }
    
    // 模式匹配
    println!("\n模式匹配:");
    match slice {
        [first, second, .., last] => {
            println!("第一个元素: {}", first);
            println!("第二个元素: {}", second);
            println!("最后一个元素: {}", last);
        }
        [single] => println!("只有一个元素: {}", single),
        [first, second] => println!("两个元素: {}, {}", first, second),
        [] => println!("空切片"),
    }
    
    println!();
}

// 11. 切片安全性和错误处理
fn slice_safety() {
    println!("11. 切片安全性和错误处理");
    println!("{}", "=".repeat(30));
    
    let numbers = [1, 2, 3, 4, 5];
    println!("数组: {:?}", numbers);
    
    // 安全的切片访问
    println!("\n安全的切片访问:");
    let safe_ranges = [(0, 3), (1, 5), (2, 10), (5, 6)];
    
    for (start, end) in safe_ranges {
        match safe_slice(&numbers, start, end) {
            Some(slice) => println!("安全切片 [{}, {}): {:?}", start, end, slice),
            None => println!("无效范围 [{}, {})", start, end),
        }
    }
    
    // 运行时错误处理
    println!("\n运行时错误处理:");
    let text = "Hello World";
    
    // 安全的字符串切片
    if let Ok(slice) = safe_string_slice_result(text, 0, 5) {
        println!("安全字符串切片 [0, 5): '{}'", slice);
    }
    
    if let Ok(slice) = safe_string_slice_result(text, 0, 20) {
        println!("安全字符串切片 [0, 20): '{}'", slice);
    } else {
        println!("字符串切片 [0, 20) 超出范围");
    }
    
    // 自定义切片验证
    println!("\n自定义切片验证:");
    let data = [10, 20, 30, 40, 50];
    let validation_result = validate_slice(&data, 1, 4);
    match validation_result {
        Ok(slice) => println!("验证通过: {:?}", slice),
        Err(e) => println!("验证失败: {}", e),
    }
    
    println!();
}

// 12. 实际应用场景
fn practical_applications() {
    println!("12. 实际应用场景");
    println!("{}", "=".repeat(30));
    
    // 文本处理
    println!("文本处理:");
    let text = "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.";
    let words = extract_words(text);
    println!("文本: '{}'", text);
    println!("单词数量: {}", words.len());
    println!("前5个单词: {:?}", &words[..5.min(words.len())]);
    
    // 数据分析
    println!("\n数据分析:");
    let scores = [85.5, 92.3, 78.9, 95.1, 88.7, 91.2, 76.4, 89.8];
    let slice = &scores[..];
    let avg = calculate_average(slice);
    let max = slice.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min = slice.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    
    println!("成绩: {:?}", slice);
    println!("平均分: {:.2}", avg);
    println!("最高分: {:.1}", max);
    println!("最低分: {:.1}", min);
    
    // 图像处理模拟
    println!("\n图像处理模拟:");
    let image_data = [255, 128, 64, 32, 16, 8, 4, 2, 1, 0];
    let processed = process_image_slice(&image_data);
    println!("原始数据: {:?}", image_data);
    println!("处理后: {:?}", processed);
    
    // 网络协议模拟
    println!("\n网络协议模拟:");
    let packet = [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64];
    let header = &packet[0..4];
    let payload = &packet[4..];
    println!("数据包: {:?}", packet);
    println!("头部: {:?}", header);
    println!("载荷: {:?}", payload);
    
    println!("\n=== 学习完成！===");
}

// 辅助函数

// 安全的字符串切片
fn safe_string_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    if start <= end && end <= s.len() {
        Some(&s[start..end])
    } else {
        None
    }
}

// 安全的字符串切片（返回Result）
fn safe_string_slice_result(s: &str, start: usize, end: usize) -> Result<&str, &'static str> {
    if start > end {
        return Err("起始位置大于结束位置");
    }
    if end > s.len() {
        return Err("结束位置超出字符串长度");
    }
    Ok(&s[start..end])
}

// 安全的切片访问
fn safe_slice<T>(data: &[T], start: usize, end: usize) -> Option<&[T]> {
    if start <= end && end <= data.len() {
        Some(&data[start..end])
    } else {
        None
    }
}

// 切片验证
fn validate_slice<T>(data: &[T], start: usize, end: usize) -> Result<&[T], String> {
    if start > end {
        return Err(format!("起始位置 {} 大于结束位置 {}", start, end));
    }
    if end > data.len() {
        return Err(format!("结束位置 {} 超出数组长度 {}", end, data.len()));
    }
    Ok(&data[start..end])
}

// 切片求和
fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

// 切片翻倍
fn double_slice(slice: &mut [i32]) {
    for num in slice.iter_mut() {
        *num *= 2;
    }
}

// 获取中间部分
fn get_middle(slice: &[i32]) -> &[i32] {
    let mid = slice.len() / 2;
    if slice.len() >= 2 {
        &slice[mid-1..mid+1]
    } else {
        slice
    }
}

// 泛型切片平均值
fn average_slice(slice: &[f64]) -> f64 {
    if slice.is_empty() {
        return 0.0;
    }
    slice.iter().sum::<f64>() / slice.len() as f64
}

// 提取单词
fn extract_words(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

// 计算平均值
fn calculate_average(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

// 图像处理模拟
fn process_image_slice(data: &[u8]) -> Vec<u8> {
    data.iter().map(|&pixel| {
        if pixel > 128 {
            255
        } else {
            0
        }
    }).collect()
}
