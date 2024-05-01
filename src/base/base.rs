include!("../config/mod.rs");
use config::Config;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct Base {}

impl Base {
    pub fn run() {
        let config: Config = Config::load_config();
        Base::listen(&config.listen_ip, config.listen_port);
    }

    pub fn listen(ip: &str, port: usize) {
        let host: String = format!("{}:{}", ip, port);
        let listener: TcpListener = TcpListener::bind(host).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            Base::handle_connection(stream);
        }
    }

    pub fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let get = b"GET / HTTP/1.1\r\n";
        if buffer.starts_with(get) {
            let contents = fs::read_to_string("hello.html").unwrap();
            let response = format!(
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
