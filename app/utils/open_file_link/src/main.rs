//! 打开链接或文件

fn main() {
    // open a website
    let b = opener::open("https://www.rust-lang.org");
    assert!(b.is_ok());

    // open a file
    let b = opener::open("./Cargo.toml");
    assert!(b.is_ok());
}
