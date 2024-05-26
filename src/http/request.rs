use super::body;
use crate::config::config::Server;
use crate::global::global::{DEFAULT_METHOD, INVALID_HOST, INVALID_URL};
use crate::http::header::HOST;
use crate::log::log;
use crate::print::print::{self, BLUE};
use std::collections::HashMap;
use std::{
    fmt,
    ptr::hash,
    str::{Split, SplitWhitespace},
};
use url::Url;

pub struct HttpBase {
    pub scheme: String,
    pub host: String,
    pub port: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub path: String,
    pub method: String,
    pub query: Vec<(String, String)>,
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
            let path: String = HttpRequest::get_path_from_request_url(&full_path);
            let query: Vec<(String, String)> = HttpRequest::get_query_from_request_url(&full_path);
            let res_request: HttpRequest = HttpRequest {
                method,
                path,
                headers,
                body,
                query,
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
    pub fn get_query_from_request_url(url: &str) -> Vec<(String, String)> {
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
     * 获取域名和端口
     */
    pub fn get_domain_port(url: &str) -> String {
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
     * 获取http_base
     */
    pub fn get_http_base(url: &str) -> HttpBase {
        // 解析 URL
        let parsed_url = Url::parse(url).expect(INVALID_URL);
        // 获取 scheme (方案) 部分
        let scheme: String = parsed_url.scheme().to_owned();
        // 获取 host (主机) 部分
        let host: String = parsed_url.host_str().expect(INVALID_HOST).to_owned();
        // 获取端口号，如果有的话
        let port: String = match parsed_url.port_or_known_default() {
            Some(port) => format!(":{}", port),
            None => "".to_string(),
        };
        // 获取 path (路径) 部分
        let path: String = parsed_url.path().to_owned();

        HttpBase {
            scheme,
            host,
            port,
            path,
        }
    }

    /**
     * 获取请求地址不含参数和哈希
     */
    pub fn get_url_without_query_hash(url: &str) -> String {
        let http_base: HttpBase = HttpRequest::get_http_base(url);
        // 拼接成不含查询参数和哈希的 URL
        format!(
            "{}://{}{}{}",
            http_base.scheme, http_base.host, http_base.port, http_base.path
        )
    }

    /**
     * 获取请求地址不含路径和参数和哈希
     */
    pub fn get_url_without_path_query_hash(url: &str) -> String {
        let http_base: HttpBase = HttpRequest::get_http_base(url);
        // 拼接成不含查询参数和哈希的 URL
        format!(
            "{}://{}{}",
            http_base.scheme, http_base.host, http_base.port
        )
    }

    /**
     * 从请求路径获取路径
     */
    pub fn get_path_from_request_url(url_path: &str) -> String {
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

    /**
     * 获取请求
     */
    pub fn process_request(res: Option<HttpRequest>) -> HttpRequest {
        if let Some(mut http_request) = res {
            let request_path = http_request.path.clone();
            // 检查是否有 Host 头
            match http_request.headers.get(HOST) {
                Some(http_host) => {
                    // 如果有 Host 头，复制它
                    let request_host = http_host.clone();
                    // 返回 http_request 的引用
                    http_request
                }
                None => {
                    // 如果没有 Host 头，返回一个新创建的 HttpRequest
                    HttpRequest {
                        method: DEFAULT_METHOD.to_owned(),
                        body: HashMap::new(), // 假设 body 是空的
                        path: request_path,
                        headers: HashMap::new(), // 假设 headers 是空的
                        query: vec![],
                    }
                }
            }
        } else {
            // 如果 res 是 None，返回一个默认的 HttpRequest
            HttpRequest {
                method: DEFAULT_METHOD.to_owned(),
                body: HashMap::new(),
                path: String::new(),
                headers: HashMap::new(),
                query: vec![],
            }
        }
    }
}
