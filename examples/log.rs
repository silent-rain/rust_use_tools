/*!
 * 日志库使用案例
*/
use log::log_enabled;
use log::Level::Debug;
use log::{debug, error, info, trace, warn};
use log::{log, Level};

/*
Log 特征：

```
pub trait Log: Sync + Send {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool;
    fn log(&self, record: &Record<'_>);
    fn flush(&self);
}
```
- enabled 用于判断某条带有元数据的日志是否能被记录，它对于 log_enabled! 宏特别有用
- log 会记录 record 所代表的日志
- flush 会将缓存中的日志数据刷到输出中，例如标准输出或者文件中


日志宏: trace!、debug!、info!、warn!、error!
 - 引入: use log::{info, trace, warn};
 - trace!，它比 debug! 的日志级别还要低、记录的信息还要详细，这么说吧，如果你想事无巨细的追踪某个流程的所有信息，就可以用它了。

 其他宏: log! 和 log_enabled!, 后者用于确定一条消息在当前模块中，对于给定的日志级别是否能够被记录.


日志输出在哪里？
    运行了，看不到任何输出。

    为什么？原因很简单，log 仅仅是日志门面库，它并不具备完整的日志库功能！，因此你无法在控制台中看到任何日志输出，这种情况下，说实话，远不如一个 println! 有用！

    但是别急，让我们看看该如何让 log 有用起来。
```
 */

// 标准的宏
fn demo1() {
    trace!("this is a log level trace");
    debug!("this is a log level debug");
    info!("this is a log level info");
    warn!("this is a log level warn");
    error!("this is a log level error");
}

// 通用的日志
fn demo2() {
    let data = (42, "Forty-two");
    let private_data = "private";

    log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
    log!(target: "app_events", Level::Warn, "App warning: {}, {}, {}",
    data.0, data.1, private_data);
}

// log_enabled! 宏
fn demo3() {
    // 判断能否记录 Debug 消息
    if log_enabled!(Debug) {
        let data = (42, "Forty-two");
        // 下面的日志记录较为昂贵，因此我们先在前面判断了是否能够记录，能，才继续这里的逻辑
        debug!("expensive debug data: {} {}", data.0, data.1);
    }
    if log_enabled!(target: "Global", Debug) {
        let data = (42, "Forty-two");
        debug!(target: "Global", "expensive debug data: {} {}", data.0, data.1);
    }
}

fn main() {
    demo1();
    demo2();
    demo3();
}
