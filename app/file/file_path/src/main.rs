//! Path 路径操作

use std::ffi::OsStr;
use std::path::Path;

fn main() {
    let path = Path::new("/tmp/foo/bar.txt");
    let parent = path.parent();
    assert_eq!(parent, Some(Path::new("/tmp/foo")));
    println!("path: {:?}", path);
    println!("parent: {:?}", parent);

    let file_stem = path.file_stem();
    assert_eq!(file_stem, Some(OsStr::new("bar")));
    println!("file_stem: {:?}", file_stem);

    let extension = path.extension();
    assert_eq!(extension, Some(OsStr::new("txt")));
    println!("extension: {:?}", extension);
}
