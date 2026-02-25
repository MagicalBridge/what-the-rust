# 猜数字游戏（带随机数）

基于 Rust 的终端猜数字小游戏，使用 `rand` 生成 1～100 的随机数，根据输入提示“太大 / 太小 / 猜中”。

## 项目概述

- 程序在 1～100 之间生成一个随机数作为答案
- 从标准输入读取一次猜测
- 比较猜测与答案，输出 `Too small!!` / `Too big!!` / `You win!!`

适合用来练习：`std::io` 输入、`rand::Rng::gen_range`、`match` 与 `Ordering` 比较。

## 环境要求

- Rust 1.70+
- Cargo

## 快速开始

```bash
cd guess_game_add_random
cargo run
```

按提示输入一个 1～100 的整数并回车即可。

## 编译

```bash
cargo build
# 运行生成的可执行文件
./target/debug/guess_game_add_random
```

## 依赖

- [rand](https://crates.io/crates/rand) 0.8.5 — 随机数生成

## 代码要点

- `rand::thread_rng().gen_range(1..101)`：生成 [1, 100] 的随机数
- `io::stdin().read_line(&mut guess)`：读取一行到字符串
- `guess.trim().parse::<u32>()`：去掉空白并解析为整数
- `guess.cmp(&secret_number)` 配合 `match`：比较并输出结果
