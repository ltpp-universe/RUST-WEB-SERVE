use crate::global::global::{
    CONFIG_PATH, DEFAULT_BUFFER_SIZE, DEFAULT_LISTEN_IP, DEFAULT_LISTEN_PORT, DEFAULT_LOG_DIR_PATH,
    DEFAULT_ROOT_PATH, JSON_DECODE_FAIL,
};
use crate::print::print::{self, GREEN};
use std::{
    clone, fmt,
    fs::{self, File},
    io::{self, Write},
    prelude,
};

#[derive(serde::Deserialize, serde::Serialize, fmt::Debug, clone::Clone)]
pub struct Server {
    pub listen_ip: String,
    pub listen_port: usize,
    pub buffer_size: usize,
    pub root_path: String,
    pub log_dir_path: String,
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "listen_ip:{}\nlisten_port:{}\nbuffer_size:{}",
            self.listen_ip, self.listen_port, self.buffer_size
        )
    }
}

#[derive(serde::Deserialize, serde::Serialize, fmt::Debug, clone::Clone)]
pub struct Config {
    pub server: Vec<Server>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut servers_str = String::new();
        for server in &self.server {
            servers_str.push_str(&format!("{}\n", server));
        }
        write!(f, "Servers:\n{}", servers_str)
    }
}

impl Config {
    pub fn creat_config() -> io::Result<Config> {
        // 创建文件并写入内容
        let server: Vec<Server> = vec![Server {
            listen_ip: (*DEFAULT_LISTEN_IP).to_owned(),
            listen_port: *DEFAULT_LISTEN_PORT,
            buffer_size: *DEFAULT_BUFFER_SIZE,
            root_path: (*DEFAULT_ROOT_PATH).to_owned(),
            log_dir_path: (*DEFAULT_LOG_DIR_PATH).to_owned(),
        }];
        let config: Config = Config { server };
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
        for one_config in &config.server {
            print::println(&one_config, GREEN, &one_config);
        }
        Ok(config)
    }
}
