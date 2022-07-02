# seasnft7
seas7 nft 自动化工具


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

### 交叉编译参考文档
- https://www.cnblogs.com/007sx/p/15191400.html
- [Rust 交叉编译与条件编译总结](https://www.jianshu.com/p/0e4251bc10eb)
- [如何在Linux操作系统上交叉编译Rust程序？](https://magiclen.org/rust-cross-compile/)
- [Rust交叉编译Mac编译Linux/Windows平台](https://www.cnblogs.com/007sx/p/15191400.html)

## 工具库


### 待添加
- [ ] yaml 读取yaml文件 serde_yaml
- [ ] email 发送邮件 lettre lettre_email
- [ ] 打开链接或文件 webbrowser/opener
- [ ] 日志  log4rs/tracing
- [ ] 序化、反序化
- [x] excel 读写
- [x] 网络请求 Reqwest
- [x] web服务 Rocket
- [x] GUI Tauri
- [x] SQL sqlx
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

