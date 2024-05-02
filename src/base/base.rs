use crate::config::config::{Config, Server};
use crate::global::global::{GET_CONFIG_FAIL, RESOURCE_LOAD_FAIL, RESOURCE_LOAD_SUCCESS};
use crate::http::request::HttpRequest;
use crate::http::response;
use crate::print::print::{self, GREEN, RED, YELLOW};
use crate::utils::file;

use std::{
    borrow, fs,
    io::prelude::{Read, Write},
    net, path, thread,
};

pub struct Base {}

impl Base {
    pub fn run() {
        let config: Result<Config, std::io::Error> = Config::load_config();
        match config {
            Ok(config) => Base::listen(&config),
            _ => panic!("{}", GET_CONFIG_FAIL),
        }
    }

    pub fn listen(config: &Config) {
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];
        for one_config in &config.server {
            let host: String = format!("{}:{}", one_config.listen_ip, one_config.listen_port);
            let listener: net::TcpListener = net::TcpListener::bind(host).unwrap();
            let clone_one_config: Server = one_config.clone();
            let handle: thread::JoinHandle<()> = thread::spawn(move || {
                for stream in listener.incoming() {
                    let stream: net::TcpStream = stream.unwrap();
                    let copy_one_config: Server = clone_one_config.clone();
                    thread::spawn(move || {
                        Base::handle_connection(stream, &copy_one_config);
                    });
                }
            });
            handles.push(handle);
        }
        // 等待所有线程结束
        for handle in handles {
            handle.join().unwrap();
        }
    }

    pub fn handle_connection(mut stream: net::TcpStream, server: &Server) {
        let mut buffer: Vec<u8> = vec![0; server.buffer_size];
        stream.read(&mut buffer).unwrap();
        let request: borrow::Cow<str> = String::from_utf8_lossy(&buffer[..]);
        let res: Option<HttpRequest> = HttpRequest::parse_http_request(&request, server);
        let mut request_path: String = String::from("");
        if let Some(http_request) = res {
            request_path = http_request.path;
        }
        let mut root_path: String = server.root_path.clone();
        if let Some(unix_path_str) = path::PathBuf::from(&root_path).to_str() {
            root_path = unix_path_str.replace("\\", "/");
        }
        if let Some(unix_path_str) = path::PathBuf::from(&request_path).to_str() {
            request_path = unix_path_str.replace("\\", "/");
        }
        if root_path.ends_with('/') {
            root_path.pop();
        }
        if request_path.starts_with("/") {
            request_path.remove(0);
        }
        let file_path: String = format!("{}/{}", root_path, request_path);
        let mut contents: String = String::from("");
        let mut load_success: bool = false;
        let mut response: String = String::from("");
        if let Some(html_res) = file::get_file_data(&file_path, server) {
            load_success = true;
            contents = html_res;
            print::println(
                &format!("{}:{}", &RESOURCE_LOAD_SUCCESS, &file_path),
                &GREEN,
                server,
            );
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
        }
        if !load_success {
            contents = response::load_other_html(404, server);
            print::println(
                &format!("{}:{}", &RESOURCE_LOAD_FAIL, &file_path),
                &RED,
                server,
            );
            response = format!(
                "HTTP/1.1 404 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
        }
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
