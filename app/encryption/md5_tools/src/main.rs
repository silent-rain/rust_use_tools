//! MD5 使用案例

fn main() {
    let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
}
