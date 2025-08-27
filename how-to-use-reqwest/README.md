# Reqwest 用法示例大全

这个项目展示了 Rust 中 `reqwest` HTTP 客户端库的各种用法和用例。

## 功能特性

本项目包含以下 reqwest 使用示例：

1. **基础 GET 和 POST 请求** - 展示如何发送简单的 HTTP 请求
2. **JSON 数据处理** - 演示如何发送和接收 JSON 数据
3. **表单数据处理** - 包括 URL 编码表单和 multipart 表单
4. **请求头处理** - 自定义请求头和响应头处理
5. **错误处理** - 各种错误场景的处理方式
6. **客户端配置** - 超时、重定向、User-Agent 等配置
7. **认证示例** - Basic Auth、Bearer Token 等认证方式
8. **文件上传和下载** - 文件传输功能示例

## 运行示例

确保您已安装 Rust，然后执行：

```bash
cargo run
```

## 依赖项

- `reqwest` - HTTP 客户端库
- `tokio` - 异步运行时
- `serde` - 序列化/反序列化
- `serde_json` - JSON 处理
- `url` - URL 处理

## 注意事项

- 所有示例都使用 `httpbin.org` 和 `jsonplaceholder.typicode.com` 作为测试端点
- 某些示例可能需要网络连接才能正常运行
- 代理配置示例已注释，需要时可以取消注释并配置实际的代理服务器

## 学习建议

建议按照代码中的顺序阅读和运行示例，每个函数都包含详细的中文注释，便于理解和学习。
