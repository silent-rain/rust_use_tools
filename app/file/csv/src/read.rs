//! CSV 读写
use csv::StringRecord;
use serde::Deserialize;
use std::{error::Error, fs::File, io};

/// 从标准输入读取 CSV 数据并将每条记录打印到标准输出。
pub fn read_stdin() -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let mut data_list = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
        data_list.push(record);
    }
    Ok(data_list)
}

#[derive(Debug, Default, Deserialize)]
pub struct Record {
    pub city: String,
    pub region: String,
    pub country: String,
    pub population: Option<u64>,
}

/// 将 CSV 数据从标准输入读取到您自己的自定义结构中。
/// 默认情况下，结构的成员名称与 CSV 数据的标头记录中的值匹配。
pub fn read_stdin_by_struct() -> Result<Vec<Record>, Box<dyn Error>> {
    let mut data_list = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
        data_list.push(record);
    }
    Ok(data_list)
}

/// 从文件读取
pub fn read_file(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let headers = rdr.headers()?;
    println!("{:?}", headers);

    let mut data_list = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let item = record
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        data_list.push(item);
    }
    Ok(data_list)
}

/// 从文件读取
/// 定义标题头、字符分隔符等配置
pub fn read_file_by_config(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(file_path)?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false) // 是否返回标题头
        .delimiter(b';') // 分隔符
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(file);

    let mut data_list = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let item = record
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        data_list.push(item);
    }
    Ok(data_list)
}

/// 结构体、字段重命名
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record2 {
    #[serde(rename = "Latitude")]
    pub latitude: f64,
    #[serde(rename = "Longitude")]
    pub longitude: f64,
    #[serde(rename = "Population")]
    pub population: Option<u64>,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "State")]
    pub state: String,
}
/// 从文件读取数据到结构体
pub fn read_file_by_struct(file_path: &str) -> Result<Vec<Record2>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let headers = rdr.headers()?;
    println!("{:?}", headers);

    let mut data_list = Vec::new();
    for result in rdr.deserialize() {
        let record: Record2 = result?;
        // println!("{:?}", record);
        // Try this if you don't like each record smushed on one line:
        println!("{:#?}", record);
        data_list.push(record)
    }
    Ok(data_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_stdin() {
        let ok = read_stdin().is_ok();
        assert!(ok)
    }

    #[test]
    fn test_read_file() {
        let ok = read_file("demo1.csv").is_ok();
        assert!(ok)
    }

    #[test]
    fn test_read_file_by_config() {
        let ok = read_file_by_config("demo1.csv").is_ok();
        assert!(ok)
    }

    #[test]
    fn test_read_file_by_struct() {
        let ok = read_file_by_struct("demo2.csv").is_ok();
        assert!(ok)
    }
}
