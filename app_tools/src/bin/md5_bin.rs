/*!md5使用案例
 *
 * [dependencies]
 * md5 = "0.7.0"
 */

fn main() {
    let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
}
