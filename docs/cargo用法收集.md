# cargo 用法收集

## 工作空间

- 运行指定的包

```
cargo run -p app_tools
```

- 运行指定的包, 包中存在多个二进制 crate

```
cargo run -p app_tools --bin app_tools
```

- 编译指定的包, 包中存在多个二进制 crate

```
cargo build
cargo build -p app_tools
cargo build -p app_tools --bin app_tools
```

## cargo 用法

- 编译(Linux 环境程序)

```
cargo build --release --target x86_64-unknown-linux-gnu
```

- 编译(Windows 环境程序)

```
sudo apt install mingw-w64
cargo build --release --target x86_64-pc-windows-gnu
```

- 文件监听与编译

```
cargo install cargo-watch
cargo watch -s "cargo clippy && cargo build && RUST_BACKTRACE=1 cargo test && RUST_BACKTRACE=1 cargo run"
```

- 代码覆盖工具

```
cargo install cargo-tarpaulin
cargo tarpaulin --ignore-tests
```

- 代码质量检查

```
cargo clippy
```

### 编译参考文档

- https://www.cnblogs.com/007sx/p/15191400.html
- [Rust 交叉编译与条件编译总结](https://www.jianshu.com/p/0e4251bc10eb)
- [如何在 Linux 操作系统上交叉编译 Rust 程序？](https://magiclen.org/rust-cross-compile/)
- [Rust 交叉编译 Mac 编译 Linux/Windows 平台](https://www.cnblogs.com/007sx/p/15191400.html)
- [减少 rust 编译后程序体积](https://www.jianshu.com/p/279407cad24c)

## bin 目录

- 每个文件都有单独的 main 入口
- 想要引入 src 资源的包, 需要在 lib 中进行公开模块
- 指定执行 bin 包: cargo run --bin log4rs_bin

### bin 目录参考文档

- https://www.cnblogs.com/s-seven/p/14864269.html
- https://rustcc.cn/article?id=dcc947c4-21a9-4ba0-ba59-43f6b580aae6

## rustup 指令

- 本机编译链工具: rustup show
- 编译链列表: rustup target list
- 添加指定编译链: rustup target add x86_64-pc-windows-gnu
- 安装工具链: rustup toolchain install x86_64-pc-windows-gnu

## 软件库参考地址

- [crates.io](https://crates.io/)
- [GitHub 软件库精选](https://github.com/rust-unofficial/awesome-rust)
- [rust cargo 一些方便的三方 cargo 子命令扩展](https://www.cnblogs.com/rongfengliang/p/11088481.html)

## Windows 编译软件图标

- [给 rust 写的 windows 程序加个图标](https://zhuanlan.zhihu.com/p/366341784)
- [embed-resource](https://crates.io/crates/embed-resource)
