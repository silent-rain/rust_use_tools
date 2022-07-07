/*!单例模式/全局变量  安全的初始化全局变量
 *
 * [dependencies]
 * once_cell = "1.10.0"
 */

use once_cell::sync::OnceCell;
use std::env;

#[derive(Debug)]
pub struct Logger {
    // ...
}
static INSTANCE: OnceCell<Logger> = OnceCell::new();
impl Logger {
    pub fn global() -> &'static Logger {
        INSTANCE.get().expect("logger is not initialized")
    }
    fn from_cli(args: env::Args) -> Result<Logger, std::io::Error> {
        // ...
        Ok(Logger {})
    }
}
fn main() {
    let logger = Logger::from_cli(env::args()).unwrap();
    INSTANCE.set(logger).unwrap();
    // 之后就统一使用`Logger::global()`
}
