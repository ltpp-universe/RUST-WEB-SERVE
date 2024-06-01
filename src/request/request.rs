use crate::config::config::{Config, Server};
use crate::file_safe::file_safe;
use crate::global::global::{
    BINDING, FAILED_TO_LOCK_THE_LISTENER, GET_CONFIG_FAIL, HOST, HTTP_HTTPS_REGEX, LISTENING,
    NOT_PROXY,
};
use crate::http::{request::HttpRequest, response};
use crate::print::print::{self, RED, YELLOW};
use rand::Rng;
use regex::Regex;
use std::collections::HashMap;
use std::{
    borrow,
    io::prelude::{Read, Write},
    net, path, str,
    sync::{Arc, Mutex},
    thread,
};

pub struct Request {}

impl Request {
    /**
     * 运行
     */
    pub fn run() {
        let config: Result<Config, std::io::Error> = Config::load_config();
        match config {
            Ok(config) => Request::listen(&config),
            _ => panic!("{}", GET_CONFIG_FAIL),
        }
    }

    /**
     * 监听
     */
    fn listen(config: &Config) {
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];
        let mut port_server_list: HashMap<usize, HashMap<String, Server>> = HashMap::new();
        let mut port_map_listener: HashMap<usize, Arc<Mutex<net::TcpListener>>> = HashMap::new();
        // 配置整合
        for one_config in &config.server {
            for (one_server_key, one_server_value) in &one_config.bind_server_name {
                match port_server_list.get_mut(&one_config.listen_port) {
                    Some(http_server) => match http_server.get(one_server_key) {
                        Some(mut _tem_http_server) => {
                            _tem_http_server = &mut one_server_value.clone();
                        }
                        None => {
                            http_server.insert(one_server_key.clone(), one_server_value.clone());
                        }
                    },
                    None => {
                        let mut tem_hash: HashMap<String, Server> = HashMap::new();
                        // 不含端口
                        tem_hash.insert(one_server_key.clone(), one_server_value.clone());
                        // 含端口
                        tem_hash.insert(
                            format!("{}:{}", one_server_key.clone(), one_config.listen_port),
                            one_server_value.clone(),
                        );
                        port_server_list.insert(one_config.listen_port, tem_hash);
                    }
                }
            }
        }
        for one_config in &config.server {
            let port: usize = one_config.listen_port;
            for (_one_server_key, one_server_value) in &one_config.bind_server_name {
                match port_map_listener.get(&port) {
                    Some(_has_use_port) => {
                        continue;
                    }
                    _ => {}
                }
                let host: String = format!("{}:{}", one_config.listen_ip, port);
                let listener: net::TcpListener = net::TcpListener::bind(host).unwrap();
                // 超时设置
                let _ = listener.set_ttl(one_server_value.proxy_timeout_seconds as u32);
                let listener_arc: Arc<Mutex<net::TcpListener>> = Arc::new(Mutex::new(listener));
                port_map_listener.insert(port, Arc::clone(&listener_arc));

                print::println(
                    &format!("{} => {}:{}", LISTENING, one_config.listen_ip, port),
                    YELLOW,
                    &one_server_value,
                );

                for one_bind_server in config.server.clone() {
                    for (one_bind_server_key, _one_bind_server_value) in
                        one_bind_server.bind_server_name
                    {
                        print::println(
                            &format!("{} => {}:{}", BINDING, one_bind_server_key, port),
                            YELLOW,
                            &one_server_value,
                        );
                    }
                }
                let listener_clone: Arc<Mutex<net::TcpListener>> = Arc::clone(&listener_arc);
                let one_config_clone: Server = one_server_value.clone();
                let port_map_listener_clone: HashMap<usize, Arc<Mutex<net::TcpListener>>> =
                    port_map_listener.clone();
                // K -> 域名 | V -> Server
                let server_map_list: HashMap<String, Server> = match port_server_list.get(&port) {
                    Some(tem_server_list) => tem_server_list.clone(),
                    _ => HashMap::new(),
                };
                let handle: thread::JoinHandle<()> = thread::spawn(move || {
                    use std::sync::MutexGuard;
                    let listener: MutexGuard<net::TcpListener> = match listener_clone.lock() {
                        Ok(guard) => guard,
                        Err(err) => {
                            print::println(
                                &format!("{} => {:?}", FAILED_TO_LOCK_THE_LISTENER, err),
                                RED,
                                &one_config_clone.clone(),
                            );
                            return;
                        }
                    };
                    for stream in listener.incoming() {
                        let stream: net::TcpStream = match stream {
                            Ok(stream) => stream,
                            Err(_) => {
                                continue;
                            }
                        };
                        let _port_map_listener_clone: HashMap<usize, Arc<Mutex<net::TcpListener>>> =
                            port_map_listener_clone.clone();
                        let server_map_list_clone: HashMap<String, Server> =
                            server_map_list.clone();
                        thread::spawn(move || {
                            Request::handle_connection(stream, server_map_list_clone);
                        });
                    }
                });
                handles.push(handle);
            }
        }
        // 等待所有线程结束
        for handle in handles {
            handle.join().unwrap();
        }
    }

    /**
     * 获取资源完整路径
     */
    fn get_full_file_path(server: &Server, request_path: &str) -> String {
        let mut tem_request_path: String = String::from(request_path);
        let mut root_path: String = server.root_path.clone();
        if let Some(unix_path_str) = path::PathBuf::from(&root_path).to_str() {
            root_path = unix_path_str.replace("\\", "/");
        }
        if let Some(unix_path_str) = path::PathBuf::from(&tem_request_path).to_str() {
            tem_request_path = unix_path_str.replace("\\", "/");
        }
        if root_path.ends_with('/') {
            root_path.pop();
        }
        if tem_request_path.starts_with("/") {
            tem_request_path.remove(0);
        }
        if tem_request_path.is_empty() {
            return Config::get_full_try_files_path(server);
        }
        format!("{}/{}", root_path, tem_request_path)
    }

    /**
     * 判断是否需要代理
     */
    pub fn judge_need_proxy(server: &Server) -> String {
        let proxy_len: usize = server.proxy.len();
        if proxy_len == 0 {
            return NOT_PROXY.to_string();
        }
        let mut safe_proxy_list: Vec<String> = vec![];
        let https_regex: Regex = Regex::new(HTTP_HTTPS_REGEX).unwrap();
        for tem in &server.proxy {
            if https_regex.is_match(tem) {
                safe_proxy_list.push(tem.clone());
            }
        }
        let safe_proxy_len: usize = safe_proxy_list.len();
        if safe_proxy_len == 0 {
            return NOT_PROXY.to_string();
        }
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let random_index: usize = rng.gen_range(0..safe_proxy_len);
        safe_proxy_list[random_index].clone()
    }

    /**
     * 处理请求
     */
    fn handle_connection(mut stream: net::TcpStream, server_map: HashMap<String, Server>) {
        // 是否找到请求来源域名/IP对应配置，只允许绑定的域名/IP访问，只允许防盗链以外的安全的访问
        let mut is_safe_request: bool = false;
        let mut server: &Server = &Config::get_default_server();
        let mut buffer: Vec<u8> = vec![0; server.buffer_size];
        stream.read(&mut buffer).unwrap();
        let request: borrow::Cow<str> = String::from_utf8_lossy(&buffer[..]);
        let res: Option<HttpRequest> = HttpRequest::parse_http_request(server, &request);
        let http_request: HttpRequest = HttpRequest::process_request(res.clone());
        let request_path: String = http_request.path.clone();

        match http_request.headers.get(&HOST.to_lowercase()) {
            Some(host) => match server_map.get(host) {
                Some(tem_server) => {
                    server = tem_server;
                    is_safe_request = true;
                }
                _ => {}
            },
            _ => {}
        }
        let file_path: String = Request::get_full_file_path(server, &request_path);

        // 防盗链校验
        if !file_safe::check_source_full_path_safe(&server, &file_path) {
            let (_contents, _code) = response::load_other_html(404, server);
            is_safe_request = false;
        }

        // 获取结果
        let res_response: Vec<u8> =
            response::get_res_response(&server, &http_request, is_safe_request, &file_path);

        // 响应结果
        stream.write(&res_response).unwrap();
        stream.flush().unwrap();
    }
}
