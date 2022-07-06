/*!异步日志使用 - 不推荐
 * [dependencies]
 * tracing = "0.1.32"
 * tracing-subscriber = "0.3.9"
 * tokio = { version = "1.17.0", features = ["full"] }
 */
use tracing::{info_span};

async fn some_other_async_function() -> Result<i32, ()> {
    Ok(1)
}

async fn my_async_function()-> Result<i32, ()> {
    let span = info_span!("my_async_function");

    // WARNING: 该 span 直到 drop 后才结束，因此在 .await 期间，span 依然处于工作中状态
    let _enter = span.enter();

    // 在这里 span 依然在记录，但是 .await 会让出当前任务的执行权，然后运行时会去运行其它任务，此时这个 span 可能会记录其它任务的执行信息，最终记录了不正确的 trace 信息
    some_other_async_function().await

    // ...
}

#[tokio::main]
async fn main() -> Result<(), ()>{
    my_async_function().await?;
    Ok(())
}

