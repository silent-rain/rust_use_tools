/*!日志-事件
 * [dependencies]
 * tracing = "0.1.32"
 * tracing-subscriber = "0.3.9"
*/
use tracing::{event, span, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

/*
Event 代表了某个时间点发生的事件，这方面它跟日志类似，但是不同的是，Event 还可以产生在 span 的上下文中。
虽然 event 在哪里都可以使用，但是最好只在 span 的上下文中使用：用于代表一个时间点发生的事件，例如记录 HTTP 请求返回的状态码，从队列中获取一个对象，等等。
*/

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    // 在 span 的上下文之外记录一次 event 事件
    event!(Level::INFO, "something happened");

    let span = span!(Level::INFO, "my_span");
    let _guard = span.enter();

    // 在 "my_span" 的上下文中记录一次 event
    event!(Level::DEBUG, "something happened inside my_span");
}
