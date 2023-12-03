//! 打开浏览器

fn main() {
    // open a website
    let b = webbrowser::open("https://www.rust-lang.org");
    assert!(b.is_ok());

    // open a file
    let b = webbrowser::open("./Cargo.toml");
    assert!(b.is_ok());
}
