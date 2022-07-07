mod config;

use log::{debug, error, info, trace, warn};
use log4rs;

fn main() {
    // 初始化配置
    if let Err(err) = config::init_config("./app_rocket/app.yml") {
        panic!("配置初始化失败! err: {}", err);
    }

    // 获取全局配置
    let config = config::global_config();

    // 日志初始化
    log4rs::init_file("./app_rocket/log4rs.yaml", Default::default()).unwrap();

    println!("{:?}", config.mysql);

    trace!("this is a log level trace");
    debug!("this is a log level debug");
    info!("this is a log level info");
    warn!("this is a log level warn");
    error!("this is a log level error");
    error!("this is a log level error");
}
