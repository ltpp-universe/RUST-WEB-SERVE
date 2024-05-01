include!("../print/mod.rs");
use print::{println, BLUE, RED};
use std::collections::HashMap;
use std::str::{Split, SplitWhitespace};

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn parse_http_request(request_str: &str) -> Option<HttpRequest> {
        let mut lines: Split<char> = request_str.split('\n');
        if let Some(request_line) = lines.next() {
            let mut parts: SplitWhitespace = request_line.split_whitespace();
            let method: String = parts.next()?.to_string();
            let path: String = parts.next()?.to_string();
            let mut headers: HashMap<String, String> = HashMap::new();
            for line in lines.clone() {
                if let Some(pos) = line.find(":") {
                    let key: String = line[..pos].trim().to_string();
                    let value: String = line[pos + 1..].trim().to_string();
                    headers.insert(key, value);
                }
            }
            let body: String = lines.collect::<Vec<_>>().join("\n");
            println(&body, BLUE);
            Some(HttpRequest {
                method,
                path,
                headers,
                body,
            })
        } else {
            None
        }
    }
}
