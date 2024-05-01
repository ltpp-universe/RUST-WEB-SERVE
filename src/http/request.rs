use crate::print::print::{println, BLUE};
use std::collections::HashMap;
use std::fmt;
use std::str::{Split, SplitWhitespace};

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: HashMap<String, Vec<String>>,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut headers_str: String = String::new();
        for (key, value) in &self.headers {
            headers_str.push_str(&format!("{}: {}\n", key, value));
        }

        let mut body_str: String = String::new();
        for (key, values) in &self.body {
            for value in values {
                body_str.push_str(&format!("{}: {}\n", key, value));
            }
        }

        write!(
            f,
            "method: {}\npath: {}\nheaders:\n{}\nbody:\n{}",
            self.method, self.path, headers_str, body_str
        )
    }
}

impl HttpRequest {
    pub fn parse_http_request(request_str: &str) -> Option<HttpRequest> {
        let mut lines: Split<char> = request_str.split('\n');
        if let Some(request_line) = lines.next() {
            let mut parts: SplitWhitespace = request_line.split_whitespace();
            let method: String = parts.next()?.to_string();
            let path: String = parts.next()?.to_string();
            let mut headers: HashMap<String, String> = HashMap::new();
            let mut body: HashMap<String, Vec<String>> = HashMap::new();
            for line in lines.clone() {
                if let Some(pos) = line.find(":") {
                    let key: String = line[..pos].trim().to_string();
                    let value: String = line[pos + 1..].trim().to_string();
                    headers.insert(key.clone(), value.clone());
                    body.entry(key).or_insert_with(Vec::new).push(value);
                }
            }
            let res_request: HttpRequest = HttpRequest {
                method,
                path,
                headers,
                body,
            };
            println(&res_request, BLUE);
            Some(res_request)
        } else {
            None
        }
    }
}
