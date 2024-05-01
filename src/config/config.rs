include!("../global/mod.rs");
include!("../print/mod.rs");
use global::{
    CONFIG_PATH, DEFAULT_BUFFER_SIZE, DEFAULT_LISTEN_IP, DEFAULT_LISTEN_PORT, JSON_DECODE_FAIL,
};
use print::{println, GREEN};
use std::{
    clone, fmt,
    fs::{self, File},
    io::{self, Write},
    prelude,
};

#[derive(serde::Deserialize, serde::Serialize, fmt::Debug, clone::Clone)]
pub struct Config {
    pub listen_ip: String,
    pub listen_port: usize,
    pub buffer_size: usize,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "listen_ip: {}\nlisten_port: {}\nbuffer_size: {}\n",
            self.listen_ip, self.listen_port, self.buffer_size
        )
    }
}

impl Config {
    pub fn creat_config() -> io::Result<Config> {
        // 创建文件并写入内容
        let config: Config = Config {
            listen_ip: (*DEFAULT_LISTEN_IP).to_string(),
            listen_port: *DEFAULT_LISTEN_PORT,
            buffer_size: *DEFAULT_BUFFER_SIZE,
        };
        let mut file: File = File::create(CONFIG_PATH)?;
        let json_str: String = serde_json::to_string(&config)?;
        file.write_all(json_str.as_bytes())?;
        Ok(config)
    }

    pub fn load_config() -> io::Result<Config> {
        if !File::open(CONFIG_PATH).is_ok() {
            Config::creat_config();
        }
        let json_str: String = fs::read_to_string(CONFIG_PATH).unwrap();
        let config: Config = serde_json::from_str(&json_str).expect(JSON_DECODE_FAIL);
        println(&config, GREEN);
        Ok(config)
    }
}
