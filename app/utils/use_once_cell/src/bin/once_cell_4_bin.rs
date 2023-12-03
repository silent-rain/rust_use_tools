//! 单例模式/全局变量 嵌入至结构体中
#![allow(unused)]
use once_cell::unsync::OnceCell;
use std::{fs, path::PathBuf};

struct Ctx {
    config_path: PathBuf,
    config: OnceCell<String>,
}

impl Ctx {
    pub fn get_config(&self) -> Result<&str, std::io::Error> {
        let cfg = self
            .config
            .get_or_try_init(|| fs::read_to_string(&self.config_path))?;
        Ok(cfg.as_str())
    }
}

fn main() {}
