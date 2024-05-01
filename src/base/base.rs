include!("../config/mod.rs");
include!("../http/mod.rs");
include!("../global/mod.rs");
use config::Config;
use global::GET_CONFIG_FAIL;
use request::HttpRequest;
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
        let host: String = format!("{}:{}", config.listen_ip, config.listen_port);
        let listener: net::TcpListener = net::TcpListener::bind(host).unwrap();
        for stream in listener.incoming() {
            let stream: net::TcpStream = stream.unwrap();
            thread::spawn(|| {
                Base::handle_connection(stream);
            });
        }
    }

    pub fn handle_connection(mut stream: net::TcpStream) {
        let mut buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request: borrow::Cow<str> = String::from_utf8_lossy(&buffer[..]);
        let res: Option<HttpRequest> = HttpRequest::parse_http_request(&request);
        let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
        if buffer.starts_with(get) {
            let contents: String = fs::read_to_string("hello.html").unwrap();
            let response: String = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
        }
    }
}
