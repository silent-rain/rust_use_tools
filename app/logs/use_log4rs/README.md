# 日志库

## Log4rs 的 pattern 支持以下内容：

- d，data 日期，默认为 ISO 9601 格式，可以通过 {d(%Y-%m-%d %H:%M:%S)} 这种方式改变日期格式
- l，log 级别
- h，高亮显示，debug 灰，info 绿，warn 黄，error 红
- L，line log 消息所在行数
- M，Module log 消息所在模块
- m，message log 消息
- n，具体平台的换行符
- X，mdc 映射诊断环境
- P，pid - The current process id.
- t，target - The target of the log message. 可能与 Module 相同
- T，thread - The name of the current thread. 线程名称
- I，thread_id - The ID of the current thread. 线程 ID
