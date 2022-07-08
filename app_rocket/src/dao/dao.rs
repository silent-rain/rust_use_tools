/*!数据库模块 */
use once_cell::sync::OnceCell;
use sqlx::mysql::{MySql, MySqlPoolOptions};
use sqlx::pool::Pool;
use sqlx::sqlite::{Sqlite, SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::ConnectOptions;
use std::str::FromStr;
use std::sync::Arc;
use std::time;

use crate::config;

// 全局数据库MySql对象
#[allow(dead_code)]
static GLOBAL_MYSQL_DAO: OnceCell<Arc<Pool<MySql>>> = OnceCell::new();
// 全局数据库Sqlite对象
#[allow(dead_code)]
static GLOBAL_SQLITE_DAO: OnceCell<Arc<Pool<Sqlite>>> = OnceCell::new();

// 获取全局MySQL数据库对象
#[allow(dead_code)]
pub fn global_mysql() -> Arc<Pool<MySql>> {
    let db_dao = GLOBAL_MYSQL_DAO.get();
    match db_dao {
        Some(db_dao) => Arc::clone(db_dao),
        None => {
            log::error!("MySql not initialized!");
            panic!("MySql not initialized!")
        }
    }
}

// 获取全局Sqlite数据库对象
#[allow(dead_code)]
pub fn global_splite() -> Arc<Pool<Sqlite>> {
    let db_dao = GLOBAL_SQLITE_DAO.get();
    match db_dao {
        Some(db_dao) => Arc::clone(db_dao),
        None => {
            log::error!("Sqlite not initialized!");
            panic!("Sqlite not initialized!")
        }
    }
}

/// 连接 MySql 数据库
/// # Examples
/// ```
/// db = connect_mysql_db(&config::Mysql{})
/// assert!(db.is_ok());
/// ```
#[allow(dead_code)]
pub async fn connect_mysql_db(conf: &config::Mysql) -> Result<(), Box<dyn std::error::Error>> {
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        conf.user, conf.password, conf.host, conf.port, conf.db_name
    );
    let pool = MySqlPoolOptions::new()
        .max_connections(conf.pool_max_open) // 连接池的上限
        .min_connections(conf.pool_min_idle) // 连接池的下限
        .acquire_timeout(time::Duration::from_secs(conf.timeout_seconds)) // 连接超时时间
        // .max_lifetime(std::time::Duration::from_secs(1800)) // 所有连接的最大生命周期
        // .idle_timeout(time::Duration::from_secs(600)) // 空闲连接的生命周期
        .connect(&database_url)
        .await?;
    GLOBAL_MYSQL_DAO.get_or_init(|| Arc::new(pool));
    Ok(())
}

/// 连接 sqlite 数据库
/// # Examples
/// ```
/// db = connect_sqlite_db(&config::Sqlite{})
/// assert!(db.is_ok());
/// ```
#[allow(dead_code)]
pub async fn connect_sqlite_db(conf: &config::Sqlite) -> Result<(), Box<dyn std::error::Error>> {
    // 创建 Sqlite3 数据库
    create_sqlite_db(&conf.db_url).await?;
    // 创建连接池
    let pool = SqlitePoolOptions::new()
        // .max_connections(conf.pool_max_open) // 连接池的上限
        // .min_connections(conf.pool_min_idle) // 连接池的下限
        // .acquire_timeout(time::Duration::from_secs(conf.timeout_seconds)) // 连接超时时间
        // .max_lifetime(std::time::Duration::from_secs(1800)) // 所有连接的最大生命周期
        // .idle_timeout(time::Duration::from_secs(600)) // 空闲连接的生命周期
        .connect(&conf.db_url)
        .await?;
    GLOBAL_SQLITE_DAO.get_or_init(|| Arc::new(pool));
    Ok(())
}

/// 创建 sqlite3 数据库
/// # Examples
/// ```
/// let database_url = "sqlite::memory:"
/// let database_url = "sqlite://app_rocket/data.sqlite3"
/// db = create_sqlite_db(database_url)
/// assert!(db.is_ok());
/// ```
/// 参考文档:
/// https://docs.rs/sqlx/0.6.0/sqlx/sqlite/struct.SqliteConnectOptions.html
/// https://crates.io/crates/sqlx
/// https://blog.csdn.net/kk3909/article/details/107236877/
/// https://blog.csdn.net/wyansai/article/details/105326744
/// https://zhuanlan.zhihu.com/p/377943210
#[allow(dead_code)]
async fn create_sqlite_db(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _pool = SqliteConnectOptions::from_str(database_url)?
        .journal_mode(SqliteJournalMode::Wal) // 设置数据库连接的日志模式
        // .read_only(true) // 只读模式
        .create_if_missing(true) // 如果文件不存在，则设置访问模式以创建数据库文件
        // .serialized(false) // 序列化
        .connect()
        .await?;
    Ok(())
}

/*

    // 插入记录
    // let sql = "INSERT INTO todos ( id,description ,done)VALUES ( $1,$2,$3 )";
    // let count = sqlx::query(sql)
    //     .bind(3)
    //     .bind("test3")
    //     .bind(false)
    //     .execute(&mut conn)
    //     .await?;
    // println!("{:?}", count);

    // 更新记录
    // let sql = "UPDATE todos SET done = TRUE WHERE id = $1";
    // let count = sqlx::query(sql).bind(3).execute(&mut conn).await?;
    // println!("{:?}", count);

    // 查询记录
    // let sql = "select id,description,done from todos WHERE id = $1";
    // let mut rows = sqlx::query(sql).bind(1).fetch(&mut conn);
    // while let Some(row) = rows.try_next().await? {
    //     let id: i32 = row.get(0);
    //     // map the row into a user-defined domain type
    //     let description: String = row.try_get("description")?;
    //     let done: bool = row.get(2);
    //     println!("{},{},{}", id, description, done);
    // }

    // 删除记录
    let sql = "delete from todos  WHERE id = $1";
    let count = sqlx::query(sql).bind(1).execute(&mut conn).await?;
    println!("{:?}", count);

*/
