/*!日志-外包装
 * [dependencies]
 * tracing = "0.1.32"
 * tracing-subscriber = "0.3.9"
 * 
 * 对于没有内置 tracing 支持或者无法使用 #instrument 的函数，例如外部库的函数，
 * 我们可以使用 Span 结构体的 in_scope 方法，它可以将同步代码包裹在一个 span 中：
*/
use tracing::info_span;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn foo(ans: i32) -> Result<i32, String> {
    println!("in foo: {}", ans);
    return Err("this is error".to_string());
}

fn main() -> Result<(), String>  {
    tracing_subscriber::registry().with(fmt::layer()).init();
    let _json = info_span!("json.parse").in_scope(|| foo(20))?;
    Ok(())
}
