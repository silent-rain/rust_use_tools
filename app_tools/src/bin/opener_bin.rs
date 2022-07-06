/*!打开链接或文件
 * [dependencies]
 * opener = "0.5.0"
 */
use opener;

fn main() {
    // open a website
    let b = opener::open("https://www.rust-lang.org");
    assert!(b.is_ok());

    // open a file
    let b = opener::open("./Cargo.toml");
    assert!(b.is_ok());
}
