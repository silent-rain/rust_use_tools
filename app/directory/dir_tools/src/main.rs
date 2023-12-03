//! 创建文件夹
use std::fs;

fn demo1() {
    if fs::read_dir("./mydir").is_ok() {
        println!("dir is exists");
        return;
    }

    // 创建空目录
    if let Err(err) = fs::create_dir("./mydir") {
        println!("Error creating directory: {}", err);
        return;
    }

    // 创建级联目录
    if let Err(err) = fs::create_dir_all("./mydir/test") {
        println!("Error creating directory: {}", err);
        return;
    }

    println!("目录创建成功!");

    // 删除文件夹
    fs::remove_dir_all("./mydir").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("目录删除成功!");
}

fn main() {
    demo1();
}
