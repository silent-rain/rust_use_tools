/*!时间操作
 *
 * [dependencies]
 * chrono = { version = "0.4.19", features = ["serde"] }
 */
use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone, Utc};
use core::time;
use std::{ops::Sub, time::SystemTime};

fn main() {
    let sys_time = SystemTime::now();
    println!("now time 显示的是一个长整数 {:?}", sys_time);

    // 当前系统时间
    let now: NaiveDateTime = Local::now().naive_local();
    println!("now: {}", now);

    // 时间戳
    let mills: i64 = now.timestamp_millis(); // 1609761696945
    println!("current time millis: {}", mills);

    // 从String转换
    let end_time =
        NaiveDateTime::parse_from_str("2020-03-06 18:36:27", "%Y-%m-%d %H:%M:%S.3f").unwrap_or(now);
    println!("end_time: {:?}", end_time);

    // 时间增减操作
    // 减 9 分钟
    let start_time = end_time.sub(Duration::minutes(9));
    println!("start_time: {}", start_time);

    // 时间按格式输出String
    let start_day = start_time.format("%Y%m%d").to_string();
    println!("start_day: {}", start_day);

    let t3: DateTime<Local> = Local
        .datetime_from_str("2020-03-28 12:00:09", "%Y-%m-%d %H:%M:%S")
        .ok()
        .unwrap();
    println!("t3: DateTime<Local>({:?})", t3);

    // 时间加减
    let _t0_as_0 = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0) + Duration::seconds(1_000_000_000);
    let _t0_as_1 = Utc.ymd(2020, 2, 1).and_hms(0, 0, 0)
        - Duration::from_std(time::Duration::from_secs(1_000_000_258)).unwrap();
    println!(
        " 本地时间 20200201 + 1_000_000_000 10亿秒 把标准库10亿秒转换成time10亿秒 再加时间计算 {:?}",
        _t0_as_1.format("%y-%m-%d %H:%M:%S.%3f").to_string()
    );
}
