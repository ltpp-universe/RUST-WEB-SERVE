include!("../global/mod.rs");
use global::CONFIG_PATH;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub listen_ip: String,
    pub listen_port: usize,
    pub buffer_size: usize,
}

impl Config {
    pub fn load_config() -> Config {
        let json_str: String = fs::read_to_string(CONFIG_PATH).unwrap();
        let config: Config = serde_json::from_str(&json_str).expect(global::JSON_DECODE_FAIL);
        println!("Config load Finish:\n{:?}", config);
        config
    }
}
