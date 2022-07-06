mod config;
use config::global_config;

use log::{debug, error, info, trace, warn};
use log4rs;

fn main() {
    // 获取全局配置
    let config = global_config();

    // 日志初始化
    log4rs::init_file("./app_rocket/log4rs.yaml", Default::default()).unwrap();

    println!("{:?}", config.mysql);

    trace!("this is a log level trace");
    debug!("this is a log level debug");
    info!("this is a log level info");
    warn!("this is a log level warn");
    error!("this is a log level error");
    error!("this is a log level error");
    let s = demo(10);
    println!("{:?}", s);
    let s = demo(999);
    println!("{:?}", s);
}

use once_cell::sync::OnceCell;
static POOL: OnceCell<i32> = OnceCell::new();
fn demo(i :i32) -> i32{
    let pool = POOL.get_or_init(|| i);
    *pool
}
