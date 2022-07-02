/*
如果想要将某个函数的整个函数体都设置为 span 的范围，
最简单的方法就是为函数标记上 #[instrument]，此时 tracing 会自动为函数创建一个 span，
span 名跟函数名相同，在输出的信息中还会自动带上函数参数。
*/
use tracing::{info, instrument};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn bar() {
    info!("in bar");
}

#[instrument]
fn bar2() {
    info!("in bar2");
}

#[instrument]
fn foo(ans: i32) {
    info!("in foo");
    bar();
    bar2();
}

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    foo(42);
}
