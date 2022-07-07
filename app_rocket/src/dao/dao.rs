/*!数据库模块 */
use futures::TryStreamExt;
use once_cell::sync::OnceCell;
use sqlx::mysql::{MySql, MySqlPoolOptions};
use sqlx::pool::{Pool, PoolOptions};
use sqlx::sqlite::{Sqlite, SqliteConnectOptions, SqliteJournalMode, SqlitePool};
use sqlx::{ConnectOptions, Database, Executor, MySqlPool};
use std::str::FromStr;
use std::sync::Arc;
use std::time;

use crate::config;

// 全局数据库对象
static GLOBAL_DAO: OnceCell<Arc<Pool<Sqlite>>> = OnceCell::new();

/// 连接数据库
/// # Examples
/// ```
/// db = connect_db(&config::Mysql{}, &"mysql")
/// assert!(db.is_ok());
/// ```
async fn connect_db<T: Database>(
    conf: &config::Mysql,
    model: &str,
) -> Result<Pool<T>, Box<dyn std::error::Error>> {
    // let DATABASE_URL: &str = "mysql://xxx:xxx@127.0.0.1:3306/testdb";
    let database_url = match model {
        "mysql" => format!(
            "mysql://{}:{}@{}:{}/{}",
            conf.user, conf.password, conf.host, conf.port, conf.db_name
        ),
        "sqlite::memory" => "sqlite::memory:".to_string(),
        "sqlite" => "sqlite://app_rocket/data.sqlite3".to_string(),
        _ => "".to_string(),
    };

    let pool: Pool<T> = PoolOptions::new()
        .max_connections(100) // 连接池的上限
        .min_connections(10) // 连接池的下限
        .acquire_timeout(time::Duration::from_secs(10)) // 连接超时时间
        .max_lifetime(std::time::Duration::from_secs(1800)) // 所有连接的最大生命周期
        .idle_timeout(time::Duration::from_secs(600)) // 空闲连接的生命周期
        .connect(&database_url)
        .await?;
    Ok(pool)
}

// 初始化全局数据库
pub async fn init_db(mysql: &config::Mysql, model: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect_db::<Sqlite>(mysql, model).await?;
    GLOBAL_DAO.get_or_init(|| Arc::new(pool));
    Ok(())
}

async fn init_db2() -> Result<(), Box<dyn std::error::Error>> {
    // https://docs.rs/sqlx/0.6.0/sqlx/sqlite/struct.SqliteConnectOptions.html

    // let database_url: &str = "sqlite::memory:";
    let database_url: &str = "sqlite://app_rocket/data.sqlite3";

    let mut conn = SqliteConnectOptions::from_str(database_url)?
        .journal_mode(SqliteJournalMode::Wal) // 设置数据库连接的日志模式
        // .read_only(true) // 只读模式
        .create_if_missing(true) // 如果文件不存在，则设置访问模式以创建数据库文件
        .serialized(false) // 序列化
        .connect()
        .await?;
    println!("{:?}", conn);
    Ok(())
}

/*
https://crates.io/crates/sqlx
https://blog.csdn.net/kk3909/article/details/107236877/
https://blog.csdn.net/wyansai/article/details/105326744
https://zhuanlan.zhihu.com/p/377943210
*/

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
