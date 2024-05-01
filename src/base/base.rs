include!("../config/mod.rs");
include!("../http/mod.rs");
include!("../global/mod.rs");
use config::Config;
use request::HttpRequest;
use std::borrow::Cow;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

pub struct Base {}

impl Base {
    pub fn run() {
        let config: Config = Config::load_config();
        Base::listen(&config);
    }

    pub fn listen(config: &Config) {
        let host: String = format!("{}:{}", config.listen_ip, config.listen_port);
        let listener: TcpListener = TcpListener::bind(host).unwrap();
        for stream in listener.incoming() {
            let stream: TcpStream = stream.unwrap();
            thread::spawn(|| {
                Base::handle_connection(stream);
            });
        }
    }

    pub fn handle_connection(mut stream: TcpStream) {
        let mut buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request: Cow<str> = String::from_utf8_lossy(&buffer[..]);
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
