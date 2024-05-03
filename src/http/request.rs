use super::body;
use crate::config;
use crate::config::config::Server;
use crate::log::log;
use crate::print::print::{self, BLUE};
use std::collections::HashMap;
use std::{
    fmt,
    ptr::hash,
    str::{Split, SplitWhitespace},
};

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
    /**
     * 解析HTTP请求
     */
    pub fn parse_http_request(request_str: &str, server: &Server) -> Option<HttpRequest> {
        let mut lines: Split<char> = request_str.split('\n');
        if let Some(request_line) = lines.next() {
            let mut parts: SplitWhitespace = request_line.split_whitespace();
            let method: String = parts.next()?.to_owned();
            let full_path: String = parts.next()?.to_string();
            let mut headers: HashMap<String, String> = HashMap::new();
            let mut body: HashMap<String, Vec<String>> = HashMap::new();
            for line in lines.clone() {
                if let Some(pos) = line.find(":") {
                    let key: String = line[..pos].trim().to_owned();
                    let value: String = line[pos + 1..].trim().to_owned();
                    headers.insert(key.clone(), value.clone());
                    body.entry(key).or_insert_with(Vec::new).push(value);
                }
            }
            let path: String = HttpRequest::get_path_from_request_path(&full_path);
            let res_request: HttpRequest = HttpRequest {
                method,
                path,
                headers,
                body,
            };
            log::write_no_print(&res_request, server);
            Some(res_request)
        } else {
            None
        }
    }

    /**
     * 获取GET参数
     */
    pub fn get_query(url: &str) -> Vec<(String, String)> {
        let mut query_params: Vec<(String, String)> = vec![];
        if let Some(query_start) = url.find('?') {
            // 找到第一个 '#' 或者直到字符串结尾
            let query_end: usize = url.find('#').unwrap_or(url.len());
            let query_str: &str = &url[query_start + 1..query_end];
            for pair in query_str.split('&') {
                let mut parts: Split<char> = pair.split('=');
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    query_params.push((key.to_owned(), value.to_owned()));
                }
            }
        }
        query_params
    }

    /**
     * 获取域名
     */
    pub fn get_domain(url: &str) -> String {
        let find_str: &str = "://";
        if let Some(pos_scheme_end) = url.find(find_str) {
            let pos_domain_start: usize = pos_scheme_end + find_str.len();
            let domain_start: &str = &url[pos_domain_start..];
            let pos_path_start: Option<usize> = domain_start
                .find('/')
                .map(|pos: usize| pos_domain_start + pos);
            let pos_query_start: Option<usize> = domain_start
                .find('?')
                .map(|pos: usize| pos_domain_start + pos);
            let pos_hash_start: Option<usize> = domain_start
                .find('#')
                .map(|pos: usize| pos_domain_start + pos);
            let url_len: usize = url.len();
            let pos_domain_arr: [Option<usize>; 4] = [
                pos_path_start,
                pos_query_start,
                pos_hash_start,
                Some(url_len),
            ];
            let pos_domain_end: &usize = pos_domain_arr.iter().flatten().min().unwrap_or(&url_len);
            let domain: &str = &url[pos_domain_start..*pos_domain_end];
            domain.to_owned()
        } else {
            String::new()
        }
    }

    /**
     * 获取路径
     */
    pub fn get_path(url: &str) -> String {
        let find_str: &str = "://";
        if let Some(pos_scheme_end) = url.find(find_str) {
            let pos_domain_start: usize = pos_scheme_end + find_str.len();
            let domain_start: &str = &url[pos_domain_start..];
            let pos_path_start: Option<usize> = domain_start
                .find('/')
                .map(|pos: usize| pos_domain_start + pos);
            let pos_query_start: Option<usize> = domain_start
                .find('?')
                .map(|pos: usize| pos_domain_start + pos);
            let pos_hash_start: Option<usize> = domain_start
                .find('#')
                .map(|pos: usize| pos_domain_start + pos);
            let url_len: usize = url.len();
            let pos_path_arr: [Option<usize>; 3] = [pos_query_start, pos_hash_start, Some(url_len)];
            let pos_path_end: &usize = pos_path_arr.iter().flatten().min().unwrap_or(&url_len);
            let pos_domain_arr: [Option<usize>; 4] = [
                pos_path_start,
                pos_query_start,
                pos_hash_start,
                Some(url_len),
            ];
            let pos_domain_end: &usize = pos_domain_arr.iter().flatten().min().unwrap_or(&url_len);
            let path: &str = &url[*pos_domain_end..*pos_path_end];
            path.to_owned()
        } else {
            String::new()
        }
    }

    /**
     * 从请求路径获取路径
     */
    pub fn get_path_from_request_path(url_path: &str) -> String {
        let pos_path_start: Option<usize> = url_path.find('/');
        let pos_query_start: Option<usize> = url_path.find('?');
        let pos_hash_start: Option<usize> = url_path.find('#');
        let url_len: usize = url_path.len();
        let pos_path_arr: [Option<usize>; 3] = [pos_query_start, pos_hash_start, Some(url_len)];
        let pos_path_end: &usize = pos_path_arr.iter().flatten().min().unwrap_or(&url_len);
        let mut path: &str = "";
        if let Some(path_start) = pos_path_start {
            path = &url_path[path_start..*pos_path_end];
        }
        path.to_owned()
    }
}
