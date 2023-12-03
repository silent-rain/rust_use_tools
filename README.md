# rust_use_tools

rust 用例工具箱

## 运行示例

```shell
cargo run -p file_path
```

## 工具库

### 读写文件

- [x] [读写 CSV](./app/file/csv)
- [x] [读写 Excel ](./app/file/excel)
- [x] [操作文件路径](./app/file/file_path)
- [x] [读取 yaml](./app/file/use_yaml)

### 文件夹

- [x] [操作文件夹](./app/directory/dir_tools)

## 网络库

- [x] [网络请求 Reqwest](./app/http/use_reqwest)

### 加密

- [x] [MD5](./app/encryption/md5_tools)
- [x] [UUID](./app/encryption/uuid_tools)

### ORM

- [x] [sqlx-sqlite 综合示例](./app/orm/app_sqlx_sqlite)
- [x] [SQL sqlx 一种异步、纯 Rust†SQL 板条箱，具有编译时检查查询功能，没有 DSL](./app/orm/sqlx)
- [ ] rbatis 高性能、安全、动态 SQL（编译时）ORM 框架

## 日志

- [x] [日志 log4rs](./app/logs/use_log4rs)
- [x] [日志 tracing](./app/logs/use_tracing)

## 测试框架

- [ ] 基于属性的测试库 proptest
- [ ] 代码覆盖工具 cargo-tarpaulin
- [ ] QuickCheck 一种使用随机生成的输入进行基于属性的测试的方法。
- [ ] speculate 受 RSpec 启发的最小锈蚀测试框架。

### 常用工具

- [x] [dotenv 环境变量](./app/utils/env)
- [x] [监听文件 cargo-watch](./app/utils/watch_project)
- [x] [时间库 chrono](./app/utils/time)
- [x] [打开链接或文件](./app/utils/open_file_link)
- [x] [打开浏览器](./app/utils/webbrowser)
- [x] [序化、反序化 serde_json](./app/utils/use_serde_json)
- [x] [内嵌资源](./app/utils/use_embed_resource)
- [x] [单例模式/全局变量](./app/utils/use_once_cell)
- [x] [email 发送邮件](./app/utils/use_email)

## 其他

- [ ] web 服务 Rocket
- [ ] GUI Tauri
- [ ] 把 Go 或其他 c-lib 库混入 Rust libloading
- [ ] parking_lot 比 Rust 标准库更小、更快和更灵活的互斥、RwLock、Condvar 和 Once 的实现，以及支持递归锁定的可重入 tmutex 类型。它还公开了一个低级 API，用于创建自己的高效同步原语。

## 库参考文档

- [Rodio - 一个 Rust 音频播放库](https://blog.csdn.net/u012067469/article/details/109153091)
- [Rust Cargo 使用总结](http://www.javashuo.com/article/p-cyeyuedk-bh.html)
- [cargo-watch](https://formulae.brew.sh/formula/cargo-watch)

- 全局变量
  - [要如何实现一个全局变量的初始化（单例）](https://rustcc.cn/article?id=31cfe6b0-4e36-44c9-a2b7-ea53486dde9e)
  - [once_cell - 最多初始化一次的 cell](https://copyfuture.com/blogs-details/20201218163532459q9gi3pvr2d2y3tk)
