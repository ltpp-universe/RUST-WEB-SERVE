use crate::global::global::{
    BINDING, CONFIG_PATH, DEFAULT_BUFFER_SIZE, DEFAULT_EMPTY_PATH_TRY_FILES_PATH,
    DEFAULT_HOTLINK_PROTECTION, DEFAULT_LISTEN_IP, DEFAULT_LISTEN_PORT, DEFAULT_LOG_DIR_PATH,
    DEFAULT_PROXY, DEFAULT_RESPONSE_HEADER, DEFAULT_ROOT_PATH, DEFAULT_SERVER_NAME,
    DEFAULT_SSL_CERTIFICATE_KEY_PATH, DEFAULT_SSL_CERTIFICATE_PATH, JSON_DECODE_FAIL,
    LOCALHOST_LISTEN_IP, LOCAL_LISTEN_IP, PROXY_TIMEOUT_SECONDS,
};
use crate::print::print::{self, GREEN};
use std::collections::HashMap;
use std::{
    clone, fmt,
    fs::{self, File},
    io::{self, Write},
    path, prelude,
};

#[derive(serde::Deserialize, serde::Serialize, fmt::Debug, clone::Clone)]
pub struct Server {
    pub root_path: String,
    pub log_dir_path: String,
    pub ssl_certificate_path: String,
    pub ssl_certificate_key_path: String,
    pub empty_path_try_files_path: String,
    pub response_header_list: Vec<String>,
    pub proxy: Vec<String>,
    pub proxy_timeout_seconds: usize,
    pub hotlink_protection: Vec<String>,
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "root_path:{}\nlog_dir_path:{}\nssl_certificate_path:{}\nssl_certificate_key_path:{}\nempty_path_try_files_path:{}",
             self.root_path, self.log_dir_path,self.ssl_certificate_path,self.ssl_certificate_key_path,self.empty_path_try_files_path
        )
    }
}

#[derive(serde::Deserialize, serde::Serialize, fmt::Debug, clone::Clone)]
pub struct ServerNameBind {
    pub listen_ip: String,
    pub listen_port: usize,
    pub buffer_size: usize,
    pub bind_server_name: HashMap<String, Server>,
}

impl fmt::Display for ServerNameBind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bind_server_name: String = String::new();
        for (key, value) in &self.bind_server_name {
            bind_server_name.push_str(&format!(
                "{} => {}:{}\n{:#?}\n",
                BINDING, key, self.listen_port, value
            ));
        }
        f.write_str(&bind_server_name)
    }
}

#[derive(serde::Deserialize, serde::Serialize, fmt::Debug, clone::Clone)]
pub struct Config {
    pub server: Vec<ServerNameBind>,
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
    /**
     * 获取默认Server
     */
    pub fn get_default_server() -> Server {
        Server {
            root_path: (*DEFAULT_ROOT_PATH).to_owned(),
            log_dir_path: (*DEFAULT_LOG_DIR_PATH).to_owned(),
            ssl_certificate_path: (*DEFAULT_SSL_CERTIFICATE_PATH).to_owned(),
            ssl_certificate_key_path: (*DEFAULT_SSL_CERTIFICATE_KEY_PATH).to_owned(),
            empty_path_try_files_path: (*DEFAULT_EMPTY_PATH_TRY_FILES_PATH).to_owned(),
            response_header_list: vec![(*DEFAULT_RESPONSE_HEADER).to_owned()],
            proxy: vec![(*DEFAULT_PROXY).to_owned()],
            proxy_timeout_seconds: (*PROXY_TIMEOUT_SECONDS).to_owned(),
            hotlink_protection: vec![(*DEFAULT_HOTLINK_PROTECTION).to_owned()],
        }
    }

    /**
     * 获取默认ServerNameBind
     */
    fn get_default_server_name_bind() -> ServerNameBind {
        let mut server_name_map: HashMap<String, Server> = HashMap::new();
        // localhost
        server_name_map.insert(
            (*LOCALHOST_LISTEN_IP).to_owned(),
            Config::get_default_server(),
        );
        // 127.0.0.1
        server_name_map.insert((*LOCAL_LISTEN_IP).to_owned(), Config::get_default_server());
        // 0.0.0.0
        server_name_map.insert(
            (*DEFAULT_SERVER_NAME).to_owned(),
            Config::get_default_server(),
        );
        ServerNameBind {
            listen_ip: (*DEFAULT_LISTEN_IP).to_owned(),
            listen_port: *DEFAULT_LISTEN_PORT,
            buffer_size: *DEFAULT_BUFFER_SIZE,
            bind_server_name: server_name_map,
        }
    }

    /**
     * 创建配置
     */
    pub fn creat_config() -> io::Result<Config> {
        // 创建文件并写入内容
        let server: Server = Config::get_default_server();
        let server_name_bind = Config::get_default_server_name_bind();
        let config: Config = Config {
            server: vec![server_name_bind],
        };
        let mut file: File = File::create(CONFIG_PATH)?;
        let json_str: String = serde_json::to_string(&config)?;
        file.write_all(json_str.as_bytes())?;
        Ok(config)
    }

    /**
     * 加载配置
     */
    pub fn load_config() -> io::Result<Config> {
        if !File::open(CONFIG_PATH).is_ok() {
            Config::creat_config();
        }
        let json_str: String = fs::read_to_string(CONFIG_PATH).unwrap();
        let config: Config = serde_json::from_str(&json_str).expect(JSON_DECODE_FAIL);
        for one_config in &config.server {
            for (one_server_key, one_server_value) in &one_config.bind_server_name {
                print::println(&one_config, GREEN, &one_server_value);
            }
        }
        Ok(config)
    }

    /**
     * 获取try_files_path
     */
    fn get_try_files_path(server: &Server) -> String {
        let mut try_files_path: String = String::new();
        let mut is_start: bool = false;
        for tem_char in server.empty_path_try_files_path.chars() {
            if !is_start && (tem_char == '.' || tem_char == '/') {
                continue;
            }
            is_start = true;
            try_files_path.push(tem_char.to_owned());
        }
        try_files_path
    }

    /**
     * 获取完整路径
     */
    pub fn get_full_try_files_path(server: &Server) -> String {
        let mut try_files_path: String = Config::get_try_files_path(server);
        let mut root_path: String = server.root_path.clone();
        if let Some(unix_path_str) = path::PathBuf::from(&root_path).to_str() {
            root_path = unix_path_str.replace("\\", "/");
        }
        if let Some(unix_path_str) = path::PathBuf::from(&try_files_path).to_str() {
            try_files_path = unix_path_str.replace("\\", "/");
        }
        if root_path.ends_with('/') {
            root_path.pop();
        }
        if try_files_path.starts_with("/") {
            try_files_path.remove(0);
        }
        if try_files_path.is_empty() {
            try_files_path = Config::get_try_files_path(&server);
        }
        format!("{}/{}", root_path, try_files_path)
    }
}
