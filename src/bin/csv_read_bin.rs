/*!CSV读写器
 *
 * [dependencies]
 * csv = "1.1.6"
 */
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;

/*
excel:
City,State,Population,Latitude,Longitude
Davidsons Landing,AK,,65.2419444,-165.2716667
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
*/


// 从标准输入读取 CSV 数据并将每条记录打印到标准输出。
fn read_stdin() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

// 将 CSV 数据从标准输入读取到您自己的自定义结构中。默认情况下，结构的成员名称与 CSV 数据的标头记录中的值匹配。
fn read_stdin_struct() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

// 从文件读取
fn read_file() -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file_path = "CSV-demo.csv";
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


// 从文件读取
fn read_file2() -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file_path = "CSV-demo.csv";
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record3 {
    #[serde(rename = "latitude")]
    latitude: f64,
    longitude: f64,
    population: Option<u64>,
    city: String,
    state: String,
}

// 从文件读取， 返回结构体
fn read_file3() -> Result<Vec<Record3>, Box<dyn Error>> {
    let file_path = "CSV-demo.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let headers = rdr.headers()?;
    println!("{:?}", headers);

    let mut data_list = Vec::new();
    for result in rdr.deserialize() {
        let record: Record3 = result?;
        // println!("{:?}", record);
        // Try this if you don't like each record smushed on one line:
        // println!("{:#?}", record);
        data_list.push(record)
    }
    Ok(data_list)
}


fn main() {
    let rsp = read_file();
    println!("{:?}", rsp);
    println!("==========================");
    let rsp = read_file2();
    println!("{:?}", rsp);
    println!("==========================");
    let rsp = read_file3();
    println!("{:?}", rsp);

    process::exit(0);
}
