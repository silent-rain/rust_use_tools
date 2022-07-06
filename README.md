# rust_use_tools
rust 用例工具箱

## 工作空间
- 运行指定的包

```
cargo run -p app_tools
```
- 运行指定的包, 包中存在多个二进制crate
```
cargo run -p app_tools --bin app_tools
```
- 编译指定的包, 包中存在多个二进制crate
```
cargo build
cargo build -p app_tools
cargo build -p app_tools --bin app_tools
```

## 编译
- 编译(Linux环境程序)
```
cargo build --release --target x86_64-unknown-linux-gnu
```

- 编译(Windows环境程序)
```
sudo apt install mingw-w64
cargo build --release --target x86_64-pc-windows-gnu
```

### 编译参考文档
- https://www.cnblogs.com/007sx/p/15191400.html
- [Rust 交叉编译与条件编译总结](https://www.jianshu.com/p/0e4251bc10eb)
- [如何在Linux操作系统上交叉编译Rust程序？](https://magiclen.org/rust-cross-compile/)
- [Rust交叉编译Mac编译Linux/Windows平台](https://www.cnblogs.com/007sx/p/15191400.html)
- [减少rust编译后程序体积](https://www.jianshu.com/p/279407cad24c)

## 工具库

- [ ] yaml 读取yaml文件 serde_yaml
- [ ] email 发送邮件 lettre lettre_email
- [ ] 打开链接或文件 webbrowser/opener
- [ ] 日志  log4rs/tracing
- [ ] 序化、反序化 serde_json
- [ ] CSV读写器 csv
- [ ] excel 读写 calamine(reader)/umya-spreadsheet/excelize-rs/xlsxwriter-rs/xlsx-rs/calamine(reader)/teletypewriter
- [ ] 时间库 chrono
- [ ] 网络请求 Reqwest
- [ ] uuid
- [ ] md5
- [ ] 文件夹操作 std::fs
- [ ] 文件路径操作 std::path::Path
- [x] web服务 Rocket
- [x] GUI Tauri
- [ ] SQL sqlx
- [x] 基于属性的测试库 proptest
- [x] 把Go或其他c-lib库混入Rust libloading



### Log4rs 的 pattern 支持以下内容：
- d，data 日期，默认为 ISO 9601 格式，可以通过 {d(%Y-%m-%d %H:%M:%S)} 这种方式改变日期格式
- l，log 级别
- h，高亮显示，debug 灰，info 绿，warn 黄，error 红
- L，line log消息所在行数
- M，Module log 消息所在模块
- m，message log 消息
- n，具体平台的换行符
- X，mdc 映射诊断环境
- P，pid - The current process id.
- t，target - The target of the log message. 可能与 Module 相同
- T，thread - The name of the current thread. 线程名称
- I，thread_id - The ID of the current thread. 线程 ID


## bin目录
- 每个文件都有单独的main入口
- 想要引入src资源的包, 需要在lib中进行公开模块
- 指定执行bin包: cargo run --bin log4rs_bin


### bin目录参考文档
- https://www.cnblogs.com/s-seven/p/14864269.html
- https://rustcc.cn/article?id=dcc947c4-21a9-4ba0-ba59-43f6b580aae6


## rustup 指令
- 本机编译链工具: rustup show
- 编译链列表: rustup target list
- 添加指定编译链: rustup target add x86_64-pc-windows-gnu
- 安装工具链: rustup toolchain install x86_64-pc-windows-gnu

## 软件库参考地址
- [crates.io](https://crates.io/)
- [GitHub软件库精选](https://github.com/rust-unofficial/awesome-rust)


## Windows 编译软件图标
- [给rust写的windows程序加个图标](https://zhuanlan.zhihu.com/p/366341784)
- [embed-resource](https://crates.io/crates/embed-resource)


## 库参考文档
- [Rodio - 一个Rust音频播放库](https://blog.csdn.net/u012067469/article/details/109153091)
- [Rust Cargo 使用总结](http://www.javashuo.com/article/p-cyeyuedk-bh.html)

- 全局变量
  - [要如何实现一个全局变量的初始化（单例）](https://rustcc.cn/article?id=31cfe6b0-4e36-44c9-a2b7-ea53486dde9e)
  - [once_cell - 最多初始化一次的cell](https://copyfuture.com/blogs-details/20201218163532459q9gi3pvr2d2y3tk)

