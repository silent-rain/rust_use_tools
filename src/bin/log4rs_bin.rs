use log::{debug, error, info, trace, warn};
use log4rs;


fn demo1() {
    trace!("this is a log level trace");
    debug!("this is a log level debug");
    info!("this is a log level info");
    warn!("this is a log level warn");
    error!("this is a log level error");
    error!("this is a log level error");
}

fn main() {
    // 日志初始化
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();

    demo1();
}
