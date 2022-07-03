/*!sqlx 使用
 *
 * [dependencies]
 * futures = "0.3.21"
 * tokio = { version = "1.17.0", features = ["full"] }
 * sqlx = {version = "0.6.0", default-features = false, features = ["runtime-tokio-native-tls", "chrono", "mysql", "sqlite"]}
 */
use futures::TryStreamExt;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool};
use sqlx::{ConnectOptions, Executor, MySqlPool, Row};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let DATABASE_URL: &str = "mysql://one:pass@127.0.0.1:3306/testdb";
    // let pool = MySqlPool::builder()
    //     .max_size(100) // 连接池的上限
    //     .min_size(10) // 连接池的下限
    //     .connect_timeout(std::time::Duration::from_secs(10)) // 连接超时时间
    //     .max_lifetime(std::time::Duration::from_secs(1800)) // 所有连接的最大生命周期
    //     .idle_timeout(std::time::Duration::from_secs(600)) // 空闲连接的生命周期
    //     .build(DATABASE_URL)
    //     .await?;

    // https://docs.rs/sqlx/0.6.0/sqlx/sqlite/struct.SqliteConnectOptions.html
    // let DATABASE_URL: &str = "sqlite::memory:";
    let DATABASE_URL: &str = "sqlite://data.sqlite3";

    let mut conn = SqliteConnectOptions::from_str(DATABASE_URL)?
        .journal_mode(SqliteJournalMode::Wal) // 设置数据库连接的日志模式
        // .read_only(true) // 只读模式
        .create_if_missing(true) // 如果文件不存在，则设置访问模式以创建数据库文件
        .serialized(false) // 序列化
        .connect()
        .await?;
    println!("{:?}", conn);

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

    Ok(())
}

/*
https://crates.io/crates/sqlx
https://blog.csdn.net/kk3909/article/details/107236877/
https://blog.csdn.net/wyansai/article/details/105326744
https://zhuanlan.zhihu.com/p/377943210
*/
