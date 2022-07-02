mod config;
use config::global_config;


fn main() {
    let config = global_config();
    println!("{:?}", config.mysql);
}
