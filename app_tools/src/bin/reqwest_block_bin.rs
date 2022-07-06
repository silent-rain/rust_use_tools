/*!http请求
 *
 * [dependencies]
 * reqwest = { version = "0.11.10", features = ["blocking", "json"] }
 * tokio = { version = "1.17.0", features = ["full"] }
 * serde_json = "1.0.79"
 */

use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}