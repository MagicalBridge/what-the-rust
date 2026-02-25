# 区块链 Rust 实现

一个用 Rust 编程语言实现的简单区块链项目，用于学习和理解区块链的基本概念。

## 项目简介

这是一个基础的区块链实现，包含了区块链的核心组件：
- **区块（Block）**: 包含前一个区块的哈希值、当前区块的哈希值和数据
- **区块链（BlockChain）**: 管理区块的集合
- **哈希计算**: 使用 SHA256 算法确保数据完整性

## 功能特性

- ✅ 区块创建和哈希计算
- ✅ 创世区块（Genesis Block）生成
- ✅ SHA256 哈希算法
- ✅ 区块链数据展示
- ✅ 数据完整性验证

## 技术栈

- **语言**: Rust 2021 Edition
- **哈希算法**: SHA256 (通过 `sha2` crate)
- **依赖**: 
  - `sha2 = "0.11.0-pre.3"` - SHA2 哈希函数实现

## 快速开始

### 环境要求

- Rust 1.70+ 
- Cargo 包管理器

### 安装和运行

1. 克隆项目到本地：
```bash
cd blockchain_rust
```

2. 编译项目：
```bash
cargo build
```

3. 运行程序：
```bash
cargo run
```

## 代码结构

```
blockchain_rust/
├── src/
│   └── main.rs          # 主程序文件
├── Cargo.toml           # 项目配置文件
└── README.md           # 项目说明文档
```

### 核心组件

#### Block 结构体
```rust
struct Block {
    prev_hash: Vec<u8>,  // 前一个区块的哈希值
    hash: Vec<u8>,       // 当前区块的哈希值
    data: Vec<u8>,       // 区块数据
}
```

#### BlockChain 结构体
```rust
struct BlockChain {
    blocks: Vec<Block>,  // 区块链中的所有区块
}
```

## 程序输出示例

运行程序后，您将看到类似以下的输出：

```
======== block height : 0 =======
PreHash : [0]
Hash : e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
Data : The Times 03/Jan/2009 Chancellor on brink of second bailout for banks
```

## 主要功能说明

### 1. 区块创建
- 每个区块包含数据、前一个区块的哈希值
- 自动计算当前区块的 SHA256 哈希值

### 2. 创世区块
- 使用比特币创世区块的经典数据：
  `"The Times 03/Jan/2009 Chancellor on brink of second bailout for banks"`
- 前一个哈希值设置为 `[0x00]`

### 3. 哈希计算
- 使用 SHA256 算法
- 基于前一个区块哈希值和当前数据计算
- 确保区块链的完整性和不可篡改性

## 学习要点

这个项目适合学习以下概念：

1. **区块链基础**: 理解区块和区块链的基本结构
2. **哈希函数**: 学习 SHA256 在区块链中的应用
3. **数据完整性**: 了解哈希如何确保数据不被篡改
4. **Rust 编程**: 学习 Rust 的结构体、方法和内存安全特性

## 扩展建议

可以考虑添加以下功能来增强项目：

- [ ] 添加新区块到区块链
- [ ] 区块链验证功能
- [ ] 工作量证明（Proof of Work）
- [ ] 交易系统
- [ ] 网络通信功能
- [ ] 持久化存储

## 开发环境

- **IDE**: 推荐使用 VS Code 配合 Rust 插件
- **调试**: 使用 `cargo run` 进行调试
- **测试**: 可以添加 `#[cfg(test)]` 模块进行单元测试

## 许可证

本项目仅用于学习目的。

## 贡献

欢迎提交 Issues 和 Pull Requests 来改进这个项目！

---

*这是一个教育性的区块链实现，主要用于学习 Rust 和区块链概念。*