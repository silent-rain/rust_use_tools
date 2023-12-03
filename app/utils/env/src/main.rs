//! 环境变量
//! 用于设置环境变量,实现开发或测试环境切换。
//!
//! 参考文档:
//! https://github.com/dotenv-rs/dotenv
//!
extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
