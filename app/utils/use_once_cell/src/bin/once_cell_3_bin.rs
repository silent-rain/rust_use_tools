//! 单例模式/全局变量 Lazy产生局部变量
use once_cell::unsync::Lazy;

fn main() {
    let ctx = [1, 2, 3];
    let thunk = Lazy::new(|| ctx.iter().sum::<i32>());

    assert_eq!(*thunk, 6);
}
