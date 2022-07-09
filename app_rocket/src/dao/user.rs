/*!用户模块
 *
*/

use crate::dao::global_splite;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Connection, FromRow, Row, SqliteConnection};

/*
CREATE TABLE IF NOT EXISTS `runoob_tbl`(
    `id` INT UNSIGNED AUTO_INCREMENT comment '用户ID',
    `name` VARCHAR(100) NOT NULL comment '姓名',
    `gender` Boolean NOT NULL comment '性别: 0:女,1:男',
    `age` INT comment '年龄',
    `birth` VARCHAR(32) comment '出生日期',
    `phone` VARCHAR(11) comment '手机号码',
    `email` VARCHAR(32) comment '邮件',
    `password` VARCHAR(32) comment '密码',
    `status` TINYINT(1) comment '是否禁用,0:禁用,1:启用',
    `created` VARCHAR(32) comment '创建时间',
    `updated` VARCHAR(32) comment '更新时间',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;
*/

/// User 结构
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: u32,          // 用户ID
    pub name: String,     // 姓名
    pub gender: bool,     // 性别: 0:女,1:男
    pub age: u32,         // 年龄
    pub birth: String,    // 出生日期
    pub phone: String,    // 手机号码
    pub email: String,    // 邮件
    pub password: String, // 密码
    pub status: String,   // 是否禁用,0:禁用,1:启用
    pub created: String,  // 创建时间
    pub updated: String,  // 更新时间
}

impl User {
    // 匿名读取记录 fetch
    pub async fn _get_fetch() -> Result<(), Box<dyn std::error::Error>> {
        let conn = global_splite();
        let sql = "select * FROM user WHERE name = $1";
        let mut rows = sqlx::query(sql).bind("张珊").fetch(conn.as_ref());
        while let Some(row) = rows.try_next().await? {
            // let id: i32 = row.get(0);
            let id: u32 = row.try_get("id")?;
            let age: u32 = row.try_get("age")?;
            let phone: String = row.try_get("phone")?;
            let name: String = row.try_get("name")?;
            println!("======== {},{},{},{}", id, age, name, phone);
        }
        Ok(())
    }
    // 匿名读取记录 fetch_one
    pub async fn _get_fetch_one() -> Result<(), Box<dyn std::error::Error>> {
        let conn = global_splite();
        let sql = "select * FROM user WHERE name = $1";
        let rows = sqlx::query(sql)
            .bind("张思")
            .fetch_one(conn.as_ref())
            .await?;
        println!("{:?}", rows.columns());
        Ok(())
    }
    // 结构化读取记录 fetch_one SELECT $1
    pub async fn _get_ser_fetch_one1() -> Result<(), Box<dyn std::error::Error>> {
        let conn = global_splite();
        // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(conn.as_ref())
            .await?;

        assert_eq!(row.0, 150);
        Ok(())
    }
    // 结构化读取记录 fetch_one
    pub async fn get_ser_fetch_one2() -> Result<User, Box<dyn std::error::Error>> {
        let conn = global_splite();
        let sql = "select id,name,gender,age,birth,phone,email,password,status,created,updated FROM user WHERE name = ?";

        let row: User = sqlx::query_as(sql)
            .bind("张思")
            .fetch_one(conn.as_ref())
            .await?;
            println!("{:?}", row);
        Ok(row)
    }
    // 结构化读取记录 fetch_all
    pub async fn get_ser_fetch_all() -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let conn = global_splite();
        let sql = "
        select
         id,name,gender,age,birth,phone,email,password,status,created,updated
        FROM user
        WHERE name = ?
         ";

        let rows: Vec<User> = sqlx::query_as(sql)
            .bind("张思")
            .fetch_all(conn.as_ref())
            .await?;
        for row in &rows {
            println!("{:?}", row);
        }
        Ok(rows)
    }
    // 插入记录
    pub async fn post() -> Result<(), Box<dyn std::error::Error>> {
        let conn = global_splite();
        let sql = "INSERT INTO user (name,gender,age,phone) VALUES ( $1,$2,$3,$4 )";
        let count = sqlx::query(sql)
            .bind("张思")
            .bind(0)
            .bind(19)
            .bind("183****1246")
            .execute(conn.as_ref())
            .await?;
        // SqliteQueryResult { changes: 1, last_insert_rowid: 4 }
        println!("{:?}", count);
        Ok(())
    }
    // 更新记录
    pub async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let conn = global_splite();
        let sql = "UPDATE user SET birth = 'te-updated' WHERE id = $1";
        let count = sqlx::query(sql).bind(1).execute(conn.as_ref()).await?;
        // SqliteQueryResult { changes: 1, last_insert_rowid: 0 }
        println!("{:?}", count);
        Ok(())
    }
    // 删除记录
    pub async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let conn = global_splite();
        let sql = "delete from user  WHERE name = $1";
        let count = sqlx::query(sql).bind("张珊").execute(conn.as_ref()).await?;
        // SqliteQueryResult { changes: 3, last_insert_rowid: 0 }
        println!("{:?}", count);
        Ok(())
    }
}
