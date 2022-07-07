mod config;
mod dao;

use log::{debug, error, info, trace, warn};
use log4rs;

#[tokio::main]
async fn main() {
    // 初始化日志配置
    if let Err(err) = log4rs::init_file("./app_rocket/log4rs.yaml", Default::default()) {
        warn!("log init config error: {}", err);
    }

    // 初始化全局配置实例
    if let Err(err) = config::init_config("./app_rocket/app.yml") {
        panic!("全局配置实例初始化失败! err: {}", err);
    }

    // 获取全局配置
    let config = config::global_config();
    println!("{:?}", config.mysql);

    // 初始化全局数据库实例
    if let Err(err) = dao::init_db(&config.mysql.to_owned(), "sqlite").await {
        panic!("全局数据库实例初始化失败! err: {}", err);
    }

    trace!("this is a log level trace");
    debug!("this is a log level debug");
    info!("this is a log level info");
    warn!("this is a log level warn");
    error!("this is a log level error");
    error!("this is a log level error");
}
