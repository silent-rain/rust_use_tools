/*!序化、反序化
 *
 * [dependencies]
 * serde = { version = "1.0.136", features = ["derive"] }
 * serde_json = "1.0.79"
 *
 * 序列化:
 *  serde_json Crate 提供了 serde_json::to_string、serde_json::to_vec 和 serde_json::to_writer;
 *  三个函数将 Rust 数据类型转为 JSON String、Vec<u8> 和 io::Write（比如文件或者 TCP 流）。
 *
 * 反序列化:
 *  serde_json Crate 提供了 serde_json::from_str、serde_json::from_slice 和 serde_json::from_reader;
 *  三个函数将字符串、字节切片（&[u8]）和 IO 输入（文件或者 TCP 流）解析为对应的 Rust 数据类型。
 * 
 * serde_json::Value:
 *  如果一个 JSON 格式数据没有对应的 Rust 数据类型，那么可以将其解析为 serde_json::Value 类型。这是 serde_json Crate 提供的一个递归的枚举类型，它可以表示任何有效的 JSON 数据。
 *  将 JSON 格式数据解析为 serde_json::Value 类型，同样使用的是：serde_json::from_str、serde_json::from_slice 和 serde_json::from_reader 三个函数。
 *  这三个函数返回值是泛型的，其返回值类型由指定给变量的类型决定。
 */
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

// 序列化 结构体->str
fn struct_to_string() -> Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

// 反序列化
fn string_to_struct() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);
    Ok(())
}

/*
这是 serde_json Crate 提供的一个递归的枚举类型，它可以表示任何有效的 JSON 数据，其定义如下：
    enum Value {
        Null,
        Bool(bool),
        Number(Number),
        String(String),
        Array(Vec<Value>),
        Object(Map<String, Value>),
    }
*/

// 未知类型
fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;
    // let v = serde_json::from_str::<Value>(data)?; // 或者可以这样写

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

fn main() {
    let _ = struct_to_string();
    let _ = string_to_struct();
    let _ = untyped_example();
}
