/*!日志-简单使用
 * [dependencies]
 * tracing = "0.1.32"
 * tracing-subscriber = "0.3.9"
*/
use tracing::{span, Level, info};


fn main() {
    let span = span!(Level::TRACE, "my_span");

    // `enter` 返回一个 RAII ，当其被 drop 时，将自动结束该 span
    let _enter = span.enter();
    // 这里开始进入 `my_span` 的上下文
    // 下面执行一些任务，并记录一些信息到 `my_span` 中
    // ...
    info!("this is a log level info");
} // 这里 enter 将被 drop，`my_span` 也随之结束
