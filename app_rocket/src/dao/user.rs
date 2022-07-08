/*!用户模块
 *
*/

use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::dao::global_splite;

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
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u64,          // 用户ID
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
    pub async fn get() -> Result<(), Box<dyn std::error::Error>> {
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
}
