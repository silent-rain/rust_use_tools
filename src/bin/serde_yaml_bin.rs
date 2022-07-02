/*!读取yaml文件 */
use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use std::error;
use std::fs::read_to_string;

/// 全局配置 结构
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub mysql: Mysql,
}

/// 数据库配置 结构
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Mysql {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub db: String,
    pub pool_min_idle: u64,
    pub pool_max_open: u64,
    pub timeout_seconds: u64,
}

/// 解析配置文件
/// # Examples
/// ```
/// let config = parse_config("./app.yml");
/// assert!(config.is_ok());
/// ```
fn parse_config(path: &str) -> Result<Config, Box<dyn error::Error>> {
    let content = read_to_string(&path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}

/// 解析配置文件
/// # Examples
/// ```
/// let config = parse_config2("./app.yml");
/// assert!(config.is_ok());
/// ```
fn parse_config2(path: &str) -> Result<Config, Box<dyn error::Error>> {
    let content = read_to_string(&path)?;
    let schema = serde_yaml::from_str::<RootSchema>(&content);
    return match schema {
        Ok(json) => {
            let data = serde_json::to_string_pretty(&json)?;
            let config: Config = serde_json::from_str(&*data)?;
            return Ok(config);
        }
        Err(err) => Err(Box::new(err)),
    };
}

fn main() {
    let config = parse_config("./app.yml");
    assert!(config.is_ok());

    let config = parse_config2("./app.yml");
    assert!(config.is_ok());
}
