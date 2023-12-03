//! CSV 读写
pub mod read;
pub mod wtite;

use std::process;

fn main() {
    // 从标准输入读取 CSV 数据
    // read::read_stdin().unwrap();
    // 从标准输入读取 CSV 数据, 解析到结构体中
    // read::read_stdin_by_struct().unwrap();
    // 写到标准输出
    // wtite::writer_stdout().unwrap();

    // 写到文件
    wtite::writer_file("demo1.csv").unwrap();
    // 从文件读取
    read::read_file("demo1.csv").unwrap();

    // 写到文件
    wtite::writer_file("demo1.csv").unwrap();
    // 从文件读取
    let result = read::read_file("demo1.csv").unwrap();
    println!("result: {:#?}\n", result);

    // 写到文件
    wtite::writer_file("demo1.csv").unwrap();
    // 从文件读取
    let result = read::read_file("demo1.csv").unwrap();
    println!("result read_file: {:#?}\n", result);
    // 从文件读取数据, 自自定义配置参数
    let result = read::read_file_by_config("demo1.csv").unwrap();
    println!("read_file_by_config result: {:#?}\n", result);

    // 从结构体数据写到文件
    wtite::writer_file_by_struct("demo2.csv").unwrap();
    // 从文件读取数据到结构体
    let result = read::read_file_by_struct("demo2.csv").unwrap();
    println!("read_file_by_struct result: {:#?}\n", result);

    process::exit(0);
}
