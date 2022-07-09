mod config;
mod dao;

use log;
use log4rs;

#[tokio::main]
async fn main() {
    // 初始化日志配置
    if let Err(err) = log4rs::init_file("./app_rocket/log4rs.yaml", Default::default()) {
        log::warn!("log init config error: {}", err);
    }

    // 初始化全局配置实例
    if let Err(err) = config::init_config("./app_rocket/app.yml") {
        panic!("全局配置实例初始化失败! err: {}", err);
    }

    // 获取全局配置
    let config = config::global_config();
    println!("{:?}", config.mysql);
    println!("{:?}", config.sqlite);

    // 初始化全局数据库实例
    if let Err(err) = dao::connect_sqlite_db(&config.sqlite.to_owned()).await {
        panic!("全局 sqlite 数据库实例初始化失败! err: {}", err);
    }

    // if let Err(err) = dao::User::post().await {
    //     println!("post: {:?}", err);
    // }
    // if let Err(err) = dao::User::update().await {
    //     println!("post: {:?}", err);
    // }
    // if let Err(err) = dao::User::delete().await {
    //     println!("post: {:?}", err);
    // }
    // if let Err(err) = dao::User::get().await {
    //     println!("get: {:?}", err);
    // }
    if let Err(err) = dao::User::get_ser_fetch_one2().await {
        println!("get: {:?}", err);
    }
    if let Err(err) = dao::User::get_ser_fetch_all().await {
        println!("get: {:?}", err);
    }
}
