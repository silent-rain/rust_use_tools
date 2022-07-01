/*!配置文件
*/
use once_cell::sync::Lazy;
use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use std::error;
use std::fs::read_to_string;
use std::sync::Arc;

// 配置文件
const APP_CONFIG_FILE: &str = "./app.yml";
// 全局配置对象
static GLOBAL_CONFIG: Lazy<Arc<Config>> = Lazy::new(|| {
    let config = parse_config(APP_CONFIG_FILE).unwrap_or_else(|err| {
        panic!("配置初始化失败! err: {}", err);
    });
    Arc::new(config)
});

/// 解析配置文件
/// # Examples
/// ```
/// config = parse_config("./app.yml")
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
/// config = parse_config2("./app.yml")
/// assert!(config.is_ok());
/// ```
#[allow(dead_code)]
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

/// 获取全局配置
/// # Examples
/// ```
/// config = global_config()
/// assert!(config.is_ok());
/// ```
pub fn global_config() -> Arc<Config> {
    GLOBAL_CONFIG.clone()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let path = "./app.yml";
        let config = parse_config(path);
        assert!(config.is_ok())
    }

    #[test]
    fn test_load_conf2() {
        let path = "./app.yml";
        let config = parse_config2(path);
        assert!(config.is_ok())
    }

    #[test]
    fn test_include_str() {
        let yaml_str = include_str!("../../app.yml");
        assert_ne!(yaml_str, "");
    }

    #[test]
    fn test_global_config() {
        let config = global_config();
        assert_ne!(
            config.mysql,
            Mysql {
                host: "".to_string(),
                port: 0,
                user: "".to_string(),
                password: "".to_string(),
                db: "".to_string(),
                pool_min_idle: 0,
                pool_max_open: 0,
                timeout_seconds: 0
            }
        );
    }
}
