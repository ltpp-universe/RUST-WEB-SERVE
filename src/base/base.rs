use crate::config::config::{Config, Server};
use crate::global::global::{
    GET_CONFIG_FAIL, NOT_FOUND_TEXT, RESOURCE_LOAD_FAIL, RESOURCE_LOAD_SUCCESS,
};
use crate::http::body::REFERER;
use crate::http::header::{GET, HOST};
use crate::http::request::HttpRequest;
use crate::http::response;
use crate::print::print::{self, GREEN, RED, YELLOW};
use crate::proxy;
use crate::utils::file;
use regex::Regex;
use std::error::Error;

use std::collections::{HashMap, HashSet};
use std::{
    borrow, fs,
    io::prelude::{Read, Write},
    net, path, str,
    sync::{Arc, Mutex},
    thread,
};

pub struct Base {}

impl Base {
    /**
     * 运行
     */
    pub fn run() {
        let config: Result<Config, std::io::Error> = Config::load_config();
        match config {
            Ok(config) => Base::listen(&config),
            _ => panic!("{}", GET_CONFIG_FAIL),
        }
    }

    /**
     * 监听
     */
    pub fn listen(config: &Config) {
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];
        let mut port_server_list: HashMap<usize, HashMap<String, Server>> = HashMap::new();
        let mut port_map_listener: HashMap<usize, Arc<Mutex<net::TcpListener>>> = HashMap::new();
        // 配置整合
        for one_config in &config.server {
            for (one_server_key, one_server_value) in &one_config.bind_server_name {
                match port_server_list.get_mut(&one_config.listen_port) {
                    Some(http_server) => match http_server.get(one_server_key) {
                        Some(mut tem_http_server) => {
                            tem_http_server = &mut one_server_value.clone();
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
            let buffer_size: usize = one_config.buffer_size;
            for (one_server_key, one_server_value) in &one_config.bind_server_name {
                match port_map_listener.get(&port) {
                    Some(has_use_port) => {
                        continue;
                    }
                    _ => {}
                }
                let host: String = format!("{}:{}", one_config.listen_ip, port);
                let listener: net::TcpListener = net::TcpListener::bind(host).unwrap();
                let listener_arc: Arc<Mutex<net::TcpListener>> = Arc::new(Mutex::new(listener));
                port_map_listener.insert(port, Arc::clone(&listener_arc));
                print::println(
                    &format!("http://{}:{}", one_config.listen_ip, port),
                    &YELLOW,
                    &one_server_value,
                );
                let listener_clone: Arc<Mutex<net::TcpListener>> = Arc::clone(&listener_arc);
                let one_config_clone: Server = one_server_value.clone();
                let port_map_listener_clone: HashMap<usize, Arc<Mutex<net::TcpListener>>> =
                    port_map_listener.clone();
                // K->域名 | V->Server
                let server_map_list: HashMap<String, Server> = match port_server_list.get(&port) {
                    Some(tem_server_list) => tem_server_list.clone(),
                    _ => HashMap::new(),
                };
                let handle: thread::JoinHandle<()> = thread::spawn(move || {
                    use std::sync::MutexGuard;
                    let mut listener: MutexGuard<net::TcpListener> = listener_clone.lock().unwrap();
                    for stream in listener.incoming() {
                        let stream: net::TcpStream = stream.unwrap();
                        let copy_one_config: Server = one_config_clone.clone();
                        let port_map_listener_clone: HashMap<usize, Arc<Mutex<net::TcpListener>>> =
                            port_map_listener_clone.clone();
                        let server_map_list_clone: HashMap<String, Server> =
                            server_map_list.clone();
                        thread::spawn(move || {
                            Base::handle_connection(
                                stream,
                                port,
                                buffer_size,
                                server_map_list_clone,
                            );
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
    pub fn get_full_file_path(server: &Server, request_path: &String) -> String {
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
    pub fn judge_need_proxy(server: &Server) -> bool {
        if server.proxy.is_empty() {
            return false;
        }
        let https_regex = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
        https_regex.is_match(&server.proxy)
    }

    /**
     * 处理请求
     */
    pub fn handle_connection(
        mut stream: net::TcpStream,
        port: usize,
        buffer_size: usize,
        server_map: HashMap<String, Server>,
    ) {
        // 是否找到请求来源域名对应配置，只允许绑定的域名访问
        let mut has_find_server: bool = false;
        let mut server: &Server = &Config::get_default_server();
        let mut buffer: Vec<u8> = vec![0; buffer_size];
        stream.read(&mut buffer).unwrap();
        let request: borrow::Cow<str> = String::from_utf8_lossy(&buffer[..]);
        let res: Option<HttpRequest> = HttpRequest::parse_http_request(&request, server);
        let http_request: HttpRequest = HttpRequest::process_request(res.clone());
        let request_path: String = http_request.path.clone();
        match http_request.headers.get(HOST) {
            Some(host) => match server_map.get(host) {
                Some(tem_server) => {
                    server = tem_server;
                    has_find_server = true;
                }
                _ => {}
            },
            _ => {}
        }
        let file_path: String = Base::get_full_file_path(server, &request_path);
        let mut contents: Vec<u8> = vec![];
        let mut load_success: bool = false;
        let mut res_response: Vec<u8> = vec![];
        if has_find_server {
            if Base::judge_need_proxy(server) {
                contents =
                    match proxy::proxy::send_sync_request(&server, &http_request, buffer_size) {
                        Ok(response) => response,
                        Err(err) => vec![],
                    };
                load_success = true;
            } else {
                if let Some(html_res) = file::get_file_data(&file_path, server) {
                    load_success = true;
                    contents = html_res;
                    print::println(
                        &format!("{}:{}", &RESOURCE_LOAD_SUCCESS, &file_path),
                        &GREEN,
                        server,
                    );
                }
            }
            res_response = response::response(200, &contents, server);
        }
        if !load_success || !has_find_server {
            let (contents, code) = response::load_other_html(404, server);
            print::println(
                &format!("{}:{}", &RESOURCE_LOAD_FAIL, &file_path),
                &RED,
                server,
            );
            res_response = response::response(code, &contents, server);
        }
        stream.write(&res_response).unwrap();
        stream.flush().unwrap();
    }
}
