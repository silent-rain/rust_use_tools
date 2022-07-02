/*!打开链接或浏览器
 *
 */
use webbrowser;

fn main() {
    // open a website
    let b = webbrowser::open("https://www.rust-lang.org");
    assert!(b.is_ok());

    // open a file
    let b = webbrowser::open("./Cargo.toml");
    assert!(b.is_ok());
}
