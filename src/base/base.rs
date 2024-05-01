use crate::config::config::Config;
use crate::global::global::GET_CONFIG_FAIL;
use crate::http::request::HttpRequest;

use std::{
    borrow, fs,
    io::prelude::{Read, Write},
    net, thread,
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
            let handle: thread::JoinHandle<()> = thread::spawn(move || {
                for stream in listener.incoming() {
                    let stream: net::TcpStream = stream.unwrap();
                    thread::spawn(|| {
                        Base::handle_connection(stream);
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

    pub fn handle_connection(mut stream: net::TcpStream) {
        let mut buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request: borrow::Cow<str> = String::from_utf8_lossy(&buffer[..]);
        let res: Option<HttpRequest> = HttpRequest::parse_http_request(&request);
        let contents: String = fs::read_to_string("hello.html").unwrap();
        let response: String = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
