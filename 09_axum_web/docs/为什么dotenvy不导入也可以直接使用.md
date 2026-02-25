## Rust 2018 版本的预导入特性

### 1. 依赖声明
在 `Cargo.toml` 中有：
```toml
dotenvy = "0.15"
```

### 2. Rust 2018 的模块系统
从 Rust 2018 版本开始，**外部 crate 中的内容可以通过 `crate_name::item` 的方式直接访问**，不需要显式的 `extern crate` 声明。

### 3. 三种使用方式对比

**方式1：完整路径（当前使用）**
```rust
// 不需要导入语句，直接使用完整路径
dotenvy::dotenv().ok();
```

**方式2：导入后使用**
```rust
use dotenvy::dotenv;  // 需要导入语句

dotenv().ok();
```

**方式3：导入所有内容**
```rust
use dotenvy::*;  // 导入所有公开内容

dotenv().ok();
```

## 为什么当前代码可以工作

1. **Rust 2018+ 版本**：你的项目使用 `edition = "2021"`，支持这种特性
2. **依赖已声明**：`dotenvy = "0.15"` 在 Cargo.toml 中已声明
3. **完整路径访问**：`dotenvy::dotenv()` 使用了完整的 crate 路径