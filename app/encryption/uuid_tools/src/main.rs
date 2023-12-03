//! UUID 使用
use uuid::{uuid, Uuid};

fn main() {
    // 创建新的UUID
    let id_ = Uuid::new_v4();
    println!("id: {}", id_);

    // 如果你有一个 UUID 值，你可以使用它的字符串文字形式内联：
    let id_: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
    println!("id: {}", id_);
}
