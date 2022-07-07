/*!单例模式/全局变量 Lazy产生全局变量
 *
 * [dependencies]
 * once_cell = "1.10.0"
 */
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

fn main() {
    println!("{:?}", GLOBAL_DATA.lock().unwrap());
}
