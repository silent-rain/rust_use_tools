//! 配置文件
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::error;
use std::fs::read_to_string;
use std::sync::Arc;

// 全局配置对象
static GLOBAL_CONFIG: OnceCell<Arc<Config>> = OnceCell::new();

/// 初始化, 解析配置文件
/// # Examples
///
/// ```
/// let config = init_config("./app.yml");
/// assert!(config.is_ok());
/// ```
pub fn init_config(path: &str) -> Result<(), Box<dyn error::Error>> {
    let content = read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    GLOBAL_CONFIG.get_or_init(|| Arc::new(config));
    Ok(())
}

/// 获取全局配置
/// # Examples
/// ```
/// config = global_config()
/// assert!(config.is_ok());
/// ```
pub fn global_config() -> Arc<Config> {
    let config = GLOBAL_CONFIG.get();
    match config {
        Some(config) => Arc::clone(config),
        None => {
            log::error!("configuration not initialized!");
            panic!("configuration not initialized!")
        }
    }
}

/// 全局配置 结构
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub mysql: Mysql,
    pub sqlite: Sqlite,
}

/// Mysql 数据库配置 结构
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Mysql {
    #[serde(rename = "host")]
    pub host: String, // 服务地址
    #[serde(rename = "port")]
    pub port: u32, // 端口
    #[serde(rename = "user")]
    pub user: String, // 账号
    #[serde(rename = "password")]
    pub password: String, // 密码
    #[serde(rename = "db_name")]
    pub db_name: String, // DB 数据库名称
    #[serde(rename = "pool_min_idle")]
    pub pool_min_idle: u32, // 最小连接数
    #[serde(rename = "pool_max_open")]
    pub pool_max_open: u32, // 最大连接数
    #[serde(rename = "timeout_seconds")]
    pub timeout_seconds: u64, // 连接超时时间单位秒
}

/// Sqlite3 数据库配置 结构
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Sqlite {
    // 缺失值时, 自动调用 DefaultSqlite::db_url 函数
    #[serde(rename = "db_url", default = "DefaultSqlite::db_url")]
    pub db_url: String, // Sqlite3 数据库地址
    #[serde(rename = "pool_min_idle", default = "DefaultSqlite::pool_min_idle")]
    pub pool_min_idle: u32, // 最小连接数
    #[serde(rename = "pool_max_open", default = "DefaultSqlite::pool_max_open")]
    pub pool_max_open: u32, // 最大连接数
    #[serde(rename = "timeout_seconds", default = "DefaultSqlite::timeout_seconds")]
    pub timeout_seconds: u64, // 连接超时时间单位秒
}

struct DefaultSqlite {}

/// Sqlite3 缺失值, 默认调用函数
impl DefaultSqlite {
    fn db_url() -> String {
        // "ssqlite::memory:".to_owned()
        "sqlite://app_rocket/data.sqlite3".to_string()
    }
    fn pool_min_idle() -> u32 {
        8
    }
    fn pool_max_open() -> u32 {
        32
    }
    fn timeout_seconds() -> u64 {
        15
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let path = "./app.yml";
        let config = init_config(path);
        assert!(config.is_ok())
    }

    #[test]
    fn test_include_str() {
        let yaml_str = include_str!("../app.yml");
        assert_ne!(yaml_str, "");
    }
}
