/*!http请求
 *
 * [dependencies]
 * reqwest = { version = "0.11.10", features = ["json"] }
 * tokio = { version = "1.17.0", features = ["full"] }
 * serde_json = "1.0.79"
 */

use reqwest::header;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::value::Value;
use std::collections::HashMap;
use std::time::Duration;

// get请求不含token, 输出json 格式
async fn get_calss_list_out_json() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("http://example.url")
        .await?
        .json::<HashMap<String, Value>>()
        .await?;
    println!("{:#?}", res);
    Ok(())
}

// get请求不含token, 输出 text 格式
async fn get_calss_list_out_text() -> Result<(), reqwest::Error> {
    let res = reqwest::get("http://example.url").await?.text().await?;
    Ok(println!("{:#?}", res))
}

// 创建一个 client
async fn get_column_list() -> Result<HashMap<String, Value>, reqwest::Error> {
    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", "Bearer  token_in_here".parse().unwrap());
    headers.insert(
        "Accept",
        header::HeaderValue::from_static("application/json"),
    );

    // post 参数
    // 组装要提交的数据
    let mut data = HashMap::new();
    data.insert("params", "1");

    Ok(client
        .post("https://http://example.url")
        .headers(headers)
        .timeout(Duration::from_secs(3)) // 设置超时时间
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
}

// 创建一个 client builder
async fn create_client_builder() -> Result<(), Box<dyn std::error::Error>> {
    let mut h = header::HeaderMap::new();
    h.insert(
        "Accept",
        header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::Client::builder().default_headers(h).build()?;

    let doge = client
        .get("https://api.coinstats.app/public/v1/coins/dogecoin")
        .send()
        .await?
        .text()
        .await?;
    println!("{:}", doge);
    Ok(())
}

// 创建一个 client post data
async fn get_other() -> Result<(), reqwest::Error> {
    // post 参数
    let mut data = HashMap::new();
    data.insert("params1", "1");
    data.insert("params2", "2");
    data.insert(
        "params3",
        "{\"channel_name\":\"新闻广播\",\"logo\":\"\",\"desc\":\"123\"}",
    );
    data.insert("compereName", "主持人");
    data.insert("status", "1");

    let res = reqwest::Client::new()
        .post("http://example.url")
        .json(&data)
        // .body("Some interesting content")
        // .form(&data)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Access-Control-Allow-Origin", "*")
        .send()
        .await?;
    let text = res.text().await?;
    Ok(println!("{:#?}", text))
}

#[derive(Deserialize, Debug)]
struct Response {
    coins: Vec<Coin>,
}
#[derive(Deserialize, Debug)]
struct Coin {
    id: String,
    name: String,
    icon: String,
    symbol: String,
    price: f32,
    priceBtc: f32,
}

// 反序列化 JSON 响应正文
async fn serde_json() -> Result<(), Box<dyn std::error::Error>> {
    let http_response =
        reqwest::get("https://api.coinstats.app/public/v1/coins?skip=0&limit=10").await?;
    let response = http_response.json::<Response>().await?;
    println!("{:#?}", response.coins);
    Ok(())
}

// 使用 reqwest 的自定义 HTTP 代理
async fn proxy_http() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::https("https://my-proxy.local")?)
        // .no_proxy() // 禁用代理
        .build()?;

    let doge = client
        .get("https://api.coinstats.app/public/v1/coins/dogecoin")
        .send()
        .await?
        .text()
        .await?;
    println!("{:}", doge);
    Ok(())
}

#[tokio::main]
async fn main() {
    //get请求
    if let Ok(res) = get_calss_list_out_json().await {
        println!("{:#?}", res)
    };
    //get请求
    if let Ok(res) = get_calss_list_out_text().await {
        println!("{:#?}", res)
    };

    //    post传参加token请求
    //    if let Ok(res)=get_column_list().await{
    //        println!("{:#?}",res)
    //    }

    //post new 对象
    //    if let Ok(res)=get_other().await{
    //     println!("{:#?}",res)
    // }
}
