fn main() {
    println!("=== Rust 字符串学习项目 ===\n");

    // 1. 字符串创建和基本操作
    string_creation();
    
    // 2. 字符串连接和拼接
    string_concatenation();
    
    // 3. 字符串分割和切片
    string_slicing();
    
    // 4. 字符串查找和替换
    string_search_replace();
    
    // 5. 字符串格式化
    string_formatting();
    
    // 6. 字符串转换
    string_conversion();
    
    // 7. 字符串迭代
    string_iteration();
    
    // 8. 字符串验证
    string_validation();
    
    // 9. 字符串编码和字节操作
    string_encoding();
    
    // 10. 高级字符串操作
    advanced_string_operations();
    
    // 总结
    main_end();
}

// 1. 字符串创建和基本操作
fn string_creation() {
    println!("1. 字符串创建和基本操作:");
    
    // 创建空字符串
    let mut empty_string = String::new();
    println!("   空字符串: '{}'", empty_string);
    
    // 从字符串字面量创建
    let hello = String::from("你好，世界！");
    println!("   从字面量创建: '{}'", hello);
    
    // 使用 to_string() 方法
    let world = "世界".to_string();
    println!("   使用 to_string(): '{}'", world);
    
    // 使用 with_capacity 预分配容量
    let mut preallocated = String::with_capacity(100);
    preallocated.push_str("预分配的字符串");
    println!("   预分配字符串: '{}' (容量: {})", preallocated, preallocated.capacity());
    
    // 字符串长度
    let text = "Hello, 世界!";
    println!("   字符串: '{}'", text);
    println!("   字节长度: {}", text.len());
    println!("   字符长度: {}", text.chars().count());
    
    // 字符串是否为空
    println!("   空字符串是否为空: {}", empty_string.is_empty());
    println!("   hello 是否为空: {}", hello.is_empty());
    
    // 清空字符串
    empty_string.push_str("临时内容");
    println!("   添加内容后: '{}'", empty_string);
    empty_string.clear();
    println!("   清空后: '{}'", empty_string);
    
    println!();
}

// 2. 字符串连接和拼接
fn string_concatenation() {
    println!("2. 字符串连接和拼接:");
    
    // 使用 + 操作符
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let result = s1 + &s2;
    println!("   使用 + 操作符: '{}'", result);
    
    // 使用 push_str 方法
    let mut text = String::from("你好");
    text.push_str("，世界！");
    println!("   使用 push_str: '{}'", text);
    
    // 使用 push 方法添加单个字符
    let mut greeting = String::from("Hello");
    greeting.push(' ');
    greeting.push('W');
    greeting.push('o');
    greeting.push('r');
    greeting.push('l');
    greeting.push('d');
    greeting.push('!');
    println!("   使用 push: '{}'", greeting);
    
    // 使用 format! 宏
    let name = "Alice";
    let age = 25;
    let formatted = format!("我叫 {}，今年 {} 岁", name, age);
    println!("   使用 format!: '{}'", formatted);
    
    // 使用 concat! 宏
    let concatenated = concat!("Hello", " ", "World", "!");
    println!("   使用 concat!: '{}'", concatenated);
    
    // 使用 join 方法
    let words = vec!["Hello", "World", "Rust"];
    let joined = words.join(" ");
    println!("   使用 join: '{}'", joined);
    
    println!();
}

// 3. 字符串分割和切片
fn string_slicing() {
    println!("3. 字符串分割和切片:");
    
    let text = "Hello,World,Rust,Programming";
    
    // 字符串切片
    println!("   原始字符串: '{}'", text);
    println!("   前5个字符: '{}'", &text[0..5]);
    println!("   从第7个字符开始: '{}'", &text[6..]);
    println!("   最后5个字符: '{}'", &text[text.len()-5..]);
    
    // 按分隔符分割
    let parts: Vec<&str> = text.split(',').collect();
    println!("   按逗号分割: {:?}", parts);
    
    // 分割并限制数量
    let parts_limited: Vec<&str> = text.splitn(2, ',').collect();
    println!("   限制分割数量为2: {:?}", parts_limited);
    
    // 按空白字符分割
    let sentence = "Hello   World   Rust";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("   按空白字符分割: {:?}", words);
    
    // 按行分割
    let multiline = "第一行\n第二行\n第三行";
    let lines: Vec<&str> = multiline.lines().collect();
    println!("   按行分割: {:?}", lines);
    
    // 字符串切片（注意：只能按字节边界切片）
    let chinese = "你好世界";
    println!("   中文字符串: '{}'", chinese);
    println!("   前6个字节: '{}'", &chinese[0..6]); // "你好"
    // println!("   前3个字节: '{}'", &chinese[0..3]); // 这会panic！
    
    println!();
}

// 4. 字符串查找和替换
fn string_search_replace() {
    println!("4. 字符串查找和替换:");
    
    let text = "Hello World Hello Rust Hello Programming";
    
    // 查找子字符串
    println!("   原始字符串: '{}'", text);
    println!("   包含 'Hello': {}", text.contains("Hello"));
    println!("   包含 'Python': {}", text.contains("Python"));
    
    // 查找位置
    if let Some(pos) = text.find("World") {
        println!("   'World' 的位置: {}", pos);
    }
    
    if let Some(pos) = text.rfind("Hello") {
        println!("   最后一个 'Hello' 的位置: {}", pos);
    }
    
    // 替换字符串
    let replaced = text.replace("Hello", "Hi");
    println!("   替换 'Hello' 为 'Hi': '{}'", replaced);
    
    // 替换指定次数
    let replaced_once = text.replacen("Hello", "Hi", 1);
    println!("   只替换第一个 'Hello': '{}'", replaced_once);
    
    // 使用正则表达式替换（需要添加 regex 依赖）
    // let re = Regex::new(r"Hello").unwrap();
    // let regex_replaced = re.replace_all(text, "Hi");
    // println!("   正则表达式替换: '{}'", regex_replaced);
    
    // 字符串开始和结束检查
    println!("   以 'Hello' 开始: {}", text.starts_with("Hello"));
    println!("   以 'Programming' 结束: {}", text.ends_with("Programming"));
    
    // 去除空白字符
    let whitespace_text = "  Hello World  ";
    println!("   原始: '{}'", whitespace_text);
    println!("   去除前导空白: '{}'", whitespace_text.trim_start());
    println!("   去除尾随空白: '{}'", whitespace_text.trim_end());
    println!("   去除两端空白: '{}'", whitespace_text.trim());
    
    println!();
}

// 5. 字符串格式化
fn string_formatting() {
    println!("5. 字符串格式化:");
    
    // 基本格式化
    let name = "Alice";
    let age = 25;
    let height = 165.5;
    
    let formatted = format!("姓名: {}, 年龄: {}, 身高: {:.1}cm", name, age, height);
    println!("   基本格式化: '{}'", formatted);
    
    // 数字格式化
    let number = 12345;
    println!("   数字格式化:");
    println!("   十进制: {}", number);
    println!("   十六进制: {:x}", number);
    println!("   八进制: {:o}", number);
    println!("   二进制: {:b}", number);
    
    // 对齐和填充
    let text = "Hello";
    println!("   对齐和填充:");
    println!("   左对齐: '{:<10}'", text);
    println!("   右对齐: '{:>10}'", text);
    println!("   居中对齐: '{:^10}'", text);
    println!("   用*填充: '{:*^10}'", text);
    
    // 精度控制
    let pi = 3.14159265359;
    println!("   精度控制:");
    println!("   默认精度: {}", pi);
    println!("   2位小数: {:.2}", pi);
    println!("   科学计数法: {:.2e}", pi);
    
    // 条件格式化
    let value = 42;
    let formatted = if value > 50 {
        format!("值 {} 大于50", value)
    } else {
        format!("值 {} 小于等于50", value)
    };
    println!("   条件格式化: '{}'", formatted);
    
    println!();
}

// 6. 字符串转换
fn string_conversion() {
    println!("6. 字符串转换:");
    
    // 数字转字符串
    let number = 42;
    let number_str = number.to_string();
    println!("   数字转字符串: {} -> '{}'", number, number_str);
    
    // 字符串转数字
    let str_number = "123";
    match str_number.parse::<i32>() {
        Ok(n) => println!("   字符串转数字: '{}' -> {}", str_number, n),
        Err(e) => println!("   转换失败: {}", e),
    }
    
    // 大小写转换
    let mixed = "Hello World";
    println!("   原始字符串: '{}'", mixed);
    println!("   转小写: '{}'", mixed.to_lowercase());
    println!("   转大写: '{}'", mixed.to_uppercase());
    
    // 字符编码转换
    let chinese = "你好世界";
    let bytes = chinese.as_bytes();
    println!("   中文字符串: '{}'", chinese);
    println!("   UTF-8字节: {:?}", bytes);
    
    // 从字节创建字符串
    let bytes = [72, 101, 108, 108, 111]; // "Hello"
    match String::from_utf8(bytes.to_vec()) {
        Ok(s) => println!("   从字节创建: {:?} -> '{}'", bytes, s),
        Err(e) => println!("   转换失败: {}", e),
    }
    
    // 字符串转字符数组
    let text = "Hello";
    let chars: Vec<char> = text.chars().collect();
    println!("   字符串转字符数组: '{}' -> {:?}", text, chars);
    
    println!();
}

// 7. 字符串迭代
fn string_iteration() {
    println!("7. 字符串迭代:");
    
    let text = "Hello世界";
    
    // 按字符迭代
    println!("   按字符迭代:");
    for (i, ch) in text.chars().enumerate() {
        println!("     字符 {}: '{}' (Unicode: {})", i, ch, ch as u32);
    }
    
    // 按字节迭代
    println!("   按字节迭代:");
    for (i, byte) in text.bytes().enumerate() {
        println!("     字节 {}: {} (0x{:02x})", i, byte, byte);
    }
    
    // 按行迭代
    let multiline = "第一行\n第二行\n第三行";
    println!("   按行迭代:");
    for (i, line) in multiline.lines().enumerate() {
        println!("     行 {}: '{}'", i + 1, line);
    }
    
    // 按单词迭代
    let sentence = "Hello World Rust Programming";
    println!("   按单词迭代:");
    for (i, word) in sentence.split_whitespace().enumerate() {
        println!("     单词 {}: '{}'", i + 1, word);
    }
    
    // 字符位置迭代
    println!("   字符位置迭代:");
    for (i, ch) in text.char_indices() {
        println!("     位置 {}: '{}'", i, ch);
    }
    
    println!();
}

// 8. 字符串验证
fn string_validation() {
    println!("8. 字符串验证:");
    
    // 空字符串检查
    let empty = "";
    let non_empty = "Hello";
    println!("   空字符串检查:");
    println!("   '{}' 是否为空: {}", empty, empty.is_empty());
    println!("   '{}' 是否为空: {}", non_empty, non_empty.is_empty());
    
    // 数字字符串检查
    let numeric = "12345";
    let non_numeric = "123abc";
    println!("   数字字符串检查:");
    println!("   '{}' 是否为数字: {}", numeric, numeric.chars().all(|c| c.is_numeric()));
    println!("   '{}' 是否为数字: {}", non_numeric, non_numeric.chars().all(|c| c.is_numeric()));
    
    // 字母字符串检查
    let alphabetic = "Hello";
    let mixed = "Hello123";
    println!("   字母字符串检查:");
    println!("   '{}' 是否为字母: {}", alphabetic, alphabetic.chars().all(|c| c.is_alphabetic()));
    println!("   '{}' 是否为字母: {}", mixed, mixed.chars().all(|c| c.is_alphabetic()));
    
    // 回文检查
    let palindrome = "racecar";
    let non_palindrome = "hello";
    println!("   回文检查:");
    println!("   '{}' 是否为回文: {}", palindrome, is_palindrome(palindrome));
    println!("   '{}' 是否为回文: {}", non_palindrome, is_palindrome(non_palindrome));
    
    // 邮箱格式检查
    let valid_email = "user@example.com";
    let invalid_email = "invalid-email";
    println!("   邮箱格式检查:");
    println!("   '{}' 是否为有效邮箱: {}", valid_email, is_valid_email(valid_email));
    println!("   '{}' 是否为有效邮箱: {}", invalid_email, is_valid_email(invalid_email));
    
    // 长度验证
    let short = "Hi";
    let long = "This is a very long string that exceeds the limit";
    println!("   长度验证:");
    println!("   '{}' 长度是否在3-20之间: {}", short, is_length_valid(short, 3, 20));
    println!("   '{}' 长度是否在3-20之间: {}", long, is_length_valid(long, 3, 20));
    
    println!();
}

// 回文检查函数
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

// 邮箱格式检查函数
fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && !email.starts_with('@') && !email.ends_with('@')
}

// 长度验证函数
fn is_length_valid(s: &str, min: usize, max: usize) -> bool {
    let len = s.chars().count();
    len >= min && len <= max
}

// 9. 字符串编码和字节操作
fn string_encoding() {
    println!("9. 字符串编码和字节操作:");
    
    let text = "Hello世界";
    
    // UTF-8 编码
    println!("   原始字符串: '{}'", text);
    let bytes = text.as_bytes();
    println!("   UTF-8 字节: {:?}", bytes);
    println!("   字节长度: {}", bytes.len());
    
    // 从字节创建字符串
    let hello_bytes = [72, 101, 108, 108, 111]; // "Hello"
    match String::from_utf8(hello_bytes.to_vec()) {
        Ok(s) => println!("   从字节创建: {:?} -> '{}'", hello_bytes, s),
        Err(e) => println!("   转换失败: {}", e),
    }
    
    // 处理无效 UTF-8
    let invalid_bytes = [72, 101, 108, 108, 111, 0xFF]; // 包含无效字节
    match String::from_utf8(invalid_bytes.to_vec()) {
        Ok(s) => println!("   从无效字节创建: {:?} -> '{}'", invalid_bytes, s),
        Err(e) => println!("   转换失败: {}", e),
    }
    
    // 使用 from_utf8_lossy 处理无效字节
    let lossy_string = String::from_utf8_lossy(&invalid_bytes);
    println!("   使用 from_utf8_lossy: {:?} -> '{}'", invalid_bytes, lossy_string);
    
    // 字符编码转换
    let chinese = "你好";
    println!("   中文字符串: '{}'", chinese);
    for ch in chinese.chars() {
        println!("     字符 '{}': Unicode {}, UTF-8 {:?}", ch, ch as u32, ch.encode_utf8(&mut [0; 4]));
    }
    
    println!();
}

// 10. 高级字符串操作
fn advanced_string_operations() {
    println!("10. 高级字符串操作:");
    
    // 字符串去重
    let text = "hello world hello rust";
    let unique_chars: String = text.chars().collect::<std::collections::HashSet<_>>().into_iter().collect();
    println!("   原始字符串: '{}'", text);
    println!("   去重后: '{}'", unique_chars);
    
    // 字符串反转
    let text = "Hello世界";
    let reversed: String = text.chars().rev().collect();
    println!("   原始字符串: '{}'", text);
    println!("   反转后: '{}'", reversed);
    
    // 字符串排序
    let text = "hello";
    let sorted: String = {
        let mut chars: Vec<char> = text.chars().collect();
        chars.sort();
        chars.into_iter().collect()
    };
    println!("   原始字符串: '{}'", text);
    println!("   排序后: '{}'", sorted);
    
    // 字符串统计
    let text = "hello world";
    let char_count = text.chars().count();
    let word_count = text.split_whitespace().count();
    let byte_count = text.len();
    println!("   字符串统计:");
    println!("   字符数: {}", char_count);
    println!("   单词数: {}", word_count);
    println!("   字节数: {}", byte_count);
    
    // 字符串压缩（简单实现）
    let text = "aaabbbcc";
    let compressed = compress_string(text);
    println!("   原始字符串: '{}'", text);
    println!("   压缩后: '{}'", compressed);
    
    // 字符串展开
    let text = "a3b2c1";
    let expanded = expand_string(text);
    println!("   压缩字符串: '{}'", text);
    println!("   展开后: '{}'", expanded);
    
    // 字符串模式匹配
    let text = "Hello123World456";
    let (letters, numbers) = separate_letters_numbers(text);
    println!("   原始字符串: '{}'", text);
    println!("   字母部分: '{}'", letters);
    println!("   数字部分: '{}'", numbers);
    
    println!();
}

// 字符串压缩函数
fn compress_string(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    
    let mut result = String::new();
    let mut current_char = s.chars().next().unwrap();
    let mut count = 1;
    
    for ch in s.chars().skip(1) {
        if ch == current_char {
            count += 1;
        } else {
            result.push(current_char);
            result.push_str(&count.to_string());
            current_char = ch;
            count = 1;
        }
    }
    
    result.push(current_char);
    result.push_str(&count.to_string());
    result
}

// 字符串展开函数
fn expand_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch.is_alphabetic() {
            let mut count = 0;
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_numeric() {
                    count = count * 10 + next_ch.to_digit(10).unwrap() as usize;
                    chars.next();
                } else {
                    break;
                }
            }
            for _ in 0..count {
                result.push(ch);
            }
        }
    }
    
    result
}

// 分离字母和数字函数
fn separate_letters_numbers(s: &str) -> (String, String) {
    let mut letters = String::new();
    let mut numbers = String::new();
    
    for ch in s.chars() {
        if ch.is_alphabetic() {
            letters.push(ch);
        } else if ch.is_numeric() {
            numbers.push(ch);
        }
    }
    
    (letters, numbers)
}

// 主函数结束
fn main_end() {
    println!("=== 学习完成！===");
    println!("这些示例涵盖了 Rust 中字符串操作的主要概念:");
    println!("- 字符串创建和基本操作");
    println!("- 字符串连接和拼接");
    println!("- 字符串分割和切片");
    println!("- 字符串查找和替换");
    println!("- 字符串格式化");
    println!("- 字符串转换");
    println!("- 字符串迭代");
    println!("- 字符串验证");
    println!("- 字符串编码和字节操作");
    println!("- 高级字符串操作");
    println!();
    println!("你可以运行 'cargo run' 来查看所有示例的执行结果。");
    println!("尝试修改参数和返回值来更好地理解这些概念！");
}