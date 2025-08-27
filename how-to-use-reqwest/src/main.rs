use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// 定义用于JSON操作的数据结构
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<u32>,
    title: String,
    body: String,
    user_id: u32,
}

#[derive(Debug, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("🚀 Reqwest 用法示例大全");
    println!("========================\n");

    // 1. 基础GET请求示例
    basic_get_request().await?;
    
    // 2. 基础POST请求示例
    basic_post_request().await?;
    
    // 3. JSON数据处理示例
    json_data_examples().await?;
    
    // 4. 表单数据示例
    form_data_examples().await?;
    
    // 5. 请求头处理示例
    header_examples().await?;
    
    // 6. 错误处理示例
    error_handling_examples().await?;
    
    // 7. 客户端配置示例
    client_configuration_examples().await?;
    
    // 8. 认证示例
    authentication_examples().await?;
    
    // 9. 文件上传示例
    file_upload_examples().await?;
    
    println!("\n✅ 所有示例执行完成！");
    Ok(())
}

/// 1. 基础GET请求示例
async fn basic_get_request() -> Result<(), Error> {
    println!("1️⃣ 基础GET请求示例");
    println!("---------------------");
    
    // 简单的GET请求
    let response = reqwest::get("https://httpbin.org/get").await?;
    println!("状态码: {}", response.status());
    
    // 获取响应文本
    let body = response.text().await?;
    println!("响应长度: {} 字符", body.len());
    
    // 使用Client进行GET请求（推荐方式）
    let client = reqwest::Client::new();
    let response = client
        .get("https://httpbin.org/get")
        .query(&[("param1", "value1"), ("param2", "value2")])
        .send()
        .await?;
    
    println!("带查询参数的GET请求状态: {}", response.status());
    println!();
    
    Ok(())
}

/// 2. 基础POST请求示例
async fn basic_post_request() -> Result<(), Error> {
    println!("2️⃣ 基础POST请求示例");
    println!("---------------------");
    
    let client = reqwest::Client::new();
    
    // POST请求发送文本数据
    let response = client
        .post("https://httpbin.org/post")
        .body("这是POST请求的文本数据")
        .header("Content-Type", "text/plain; charset=utf-8")
        .send()
        .await?;
    
    println!("POST请求状态: {}", response.status());
    
    // POST请求发送字节数据
    let data = b"Binary data example";
    let response = client
        .post("https://httpbin.org/post")
        .body(data.to_vec())
        .send()
        .await?;
    
    println!("二进制数据POST状态: {}", response.status());
    println!();
    
    Ok(())
}

/// 3. JSON数据处理示例
async fn json_data_examples() -> Result<(), Error> {
    println!("3️⃣ JSON数据处理示例");
    println!("--------------------");
    
    let client = reqwest::Client::new();
    
    // 发送JSON数据
    let new_post = Post {
        id: None,
        title: "我的新文章".to_string(),
        body: "这是文章内容".to_string(),
        user_id: 1,
    };
    
    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?;
    
    println!("创建文章响应状态: {}", response.status());
    
    // 解析JSON响应
    if response.status().is_success() {
        let created_post: Post = response.json().await?;
        println!("创建的文章ID: {:?}", created_post.id);
        println!("文章标题: {}", created_post.title);
    }
    
    // 获取JSON数据
    let response = client
        .get("https://jsonplaceholder.typicode.com/users/1")
        .send()
        .await?;
    
    if response.status().is_success() {
        let user: User = response.json().await?;
        println!("用户信息: {} ({})", user.name, user.email);
    }
    
    println!();
    Ok(())
}

/// 4. 表单数据示例
async fn form_data_examples() -> Result<(), Error> {
    println!("4️⃣ 表单数据示例");
    println!("------------------");
    
    let client = reqwest::Client::new();
    
    // 发送URL编码的表单数据
    let mut form_data = HashMap::new();
    form_data.insert("name", "张三");
    form_data.insert("email", "zhangsan@example.com");
    form_data.insert("message", "这是一条测试消息");
    
    let response = client
        .post("https://httpbin.org/post")
        .form(&form_data)
        .send()
        .await?;
    
    println!("表单提交状态: {}", response.status());
    
    // 使用multipart表单（通常用于文件上传）
    let form = reqwest::multipart::Form::new()
        .text("field1", "value1")
        .text("field2", "value2")
        .text("description", "这是一个多部分表单示例");
    
    let response = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;
    
    println!("多部分表单状态: {}", response.status());
    println!();
    
    Ok(())
}

/// 5. 请求头处理示例
async fn header_examples() -> Result<(), Error> {
    println!("5️⃣ 请求头处理示例");
    println!("-------------------");
    
    let client = reqwest::Client::new();
    
    // 添加自定义请求头
    let response = client
        .get("https://httpbin.org/headers")
        .header("User-Agent", "Reqwest示例客户端/1.0")
        .header("X-Custom-Header", "自定义头部值")
        .header("Accept", "application/json")
        .send()
        .await?;
    
    println!("带自定义头部的请求状态: {}", response.status());
    
    // 检查响应头
    if let Some(content_type) = response.headers().get("content-type") {
        println!("响应内容类型: {:?}", content_type);
    }
    
    if let Some(server) = response.headers().get("server") {
        println!("服务器信息: {:?}", server);
    }
    
    println!();
    Ok(())
}

/// 6. 错误处理示例
async fn error_handling_examples() -> Result<(), Error> {
    println!("6️⃣ 错误处理示例");
    println!("------------------");
    
    let client = reqwest::Client::new();
    
    // 处理404错误
    match client.get("https://httpbin.org/status/404").send().await {
        Ok(response) => {
            println!("404请求状态: {}", response.status());
            if !response.status().is_success() {
                println!("请求失败: {}", response.status());
            }
        }
        Err(e) => {
            println!("网络错误: {}", e);
        }
    }
    
    // 处理超时
    match client
        .get("https://httpbin.org/delay/10")
        .timeout(Duration::from_secs(2))
        .send()
        .await
    {
        Ok(response) => {
            println!("超时测试响应状态: {}", response.status());
        }
        Err(e) => {
            if e.is_timeout() {
                println!("请求超时了！");
            } else {
                println!("其他错误: {}", e);
            }
        }
    }
    
    println!();
    Ok(())
}

/// 7. 客户端配置示例
async fn client_configuration_examples() -> Result<(), Error> {
    println!("7️⃣ 客户端配置示例");
    println!("-------------------");
    
    // 创建具有自定义配置的客户端
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))  // 设置全局超时
        .user_agent("我的应用程序/1.0")     // 设置User-Agent
        .redirect(reqwest::redirect::Policy::limited(10))  // 限制重定向次数
        .build()?;
    
    // 使用配置好的客户端
    let response = client
        .get("https://httpbin.org/user-agent")
        .send()
        .await?;
    
    println!("自定义客户端请求状态: {}", response.status());
    
    // 代理配置示例（注释掉的代码，实际使用时取消注释）
    // let proxy_client = reqwest::Client::builder()
    //     .proxy(reqwest::Proxy::http("http://proxy.example.com:8080")?)
    //     .build()?;
    // println!("代理客户端已创建");
    
    println!();
    Ok(())
}

/// 8. 认证示例
async fn authentication_examples() -> Result<(), Error> {
    println!("8️⃣ 认证示例");
    println!("-------------");
    
    let client = reqwest::Client::new();
    
    // Basic Authentication
    let response = client
        .get("https://httpbin.org/basic-auth/user/passwd")
        .basic_auth("user", Some("passwd"))
        .send()
        .await?;
    
    println!("Basic认证状态: {}", response.status());
    
    // Bearer Token认证
    let token = "your-jwt-token-here";
    let response = client
        .get("https://httpbin.org/bearer")
        .bearer_auth(token)
        .send()
        .await?;
    
    println!("Bearer认证状态: {}", response.status());
    
    // 自定义Authorization头
    let response = client
        .get("https://httpbin.org/headers")
        .header("Authorization", "Custom token-value")
        .send()
        .await?;
    
    println!("自定义认证头状态: {}", response.status());
    
    println!();
    Ok(())
}

/// 9. 文件上传示例
async fn file_upload_examples() -> Result<(), Error> {
    println!("9️⃣ 文件上传示例");
    println!("------------------");
    
    let client = reqwest::Client::new();
    
    // 创建一个虚拟文件内容用于演示
    let file_content = "这是一个测试文件的内容\n这里是第二行内容";
    
    // 使用multipart表单上传文件
    let form = reqwest::multipart::Form::new()
        .text("description", "这是文件描述")
        .text("category", "测试文件")
        .part(
            "file",
            reqwest::multipart::Part::text(file_content)
                .file_name("test.txt")
                .mime_str("text/plain")?
        );
    
    let response = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;
    
    println!("文件上传状态: {}", response.status());
    
    // 流式上传（适用于大文件）
    let stream_data = "流式数据内容".as_bytes();
    let response = client
        .put("https://httpbin.org/put")
        .header("Content-Type", "application/octet-stream")
        .body(stream_data.to_vec())
        .send()
        .await?;
    
    println!("流式上传状态: {}", response.status());
    
    // 下载文件示例
    let response = client
        .get("https://httpbin.org/json")
        .send()
        .await?;
    
    if response.status().is_success() {
        let content = response.text().await?;
        println!("下载的内容长度: {} 字符", content.len());
        // 在实际应用中，您可以将内容写入文件
        // std::fs::write("downloaded_file.json", content)?;
    }
    
    println!();
    Ok(())
}
