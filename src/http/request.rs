use crate::config::config::Server;
use crate::global::global::{
    APPLICATION_JSON, DEFAULT_METHOD, HEADER_BR, HEADER_BR_DOUBLE, HOST, HTTPS_PORT, HTTPS_SCHEME,
    HTTP_PORT, INVALID_HOST, INVALID_URL, POST, PUT,
};
use crate::log::log;
use crate::utils::tools;
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use std::collections::HashMap;
use std::{
    fmt,
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
    pub headers: HashMap<String, String>,
    pub body: HashMap<String, String>,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut headers_str: String = String::new();
        for (key, value) in &self.headers {
            headers_str.push_str(&format!("{}: {}\n", key, value));
        }

        let mut body_str: String = String::new();
        for (key, value) in &self.body {
            body_str.push_str(&format!("{}: {}\n", key, value));
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
    pub fn parse_http_request(server: &Server, request_str: &str) -> Option<HttpRequest> {
        let parts: Vec<&str> = request_str.split(HEADER_BR_DOUBLE).collect();
        if parts.len() < HEADER_BR.len() {
            return None;
        }
        let header_part: &str = parts[0];
        let body_part: &str = parts[1];
        let mut method: String = String::new();
        let mut path: String = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut body: HashMap<String, String> = HashMap::new();
        let mut lines: Split<&str> = header_part.split(HEADER_BR);
        if let Some(request_line) = lines.next() {
            let mut parts: SplitWhitespace = request_line.split_whitespace();
            method = parts.next()?.to_owned();
            let full_path: String = parts.next()?.to_string();
            headers = HashMap::new();

            // 解析请求头
            for line in lines {
                if let Some(pos) = line.find(":") {
                    let key: String = line[..pos].trim().to_owned().to_lowercase();
                    let value: String = line[pos + 1..].trim().to_owned();
                    headers.insert(key.clone(), value.clone());
                }
            }

            // 解析路径和查询参数
            path = HttpRequest::get_path_from_request_url(&full_path);

            // 解析参数
            body = match method.as_str() {
                POST => {
                    let mut tem_query: HashMap<String, String> = HashMap::new();
                    for param in body_part.split('&') {
                        if let Some(pos) = param.find('=') {
                            let key: String = percent_decode_str(&param[..pos])
                                .decode_utf8_lossy()
                                .to_string()
                                .replace("\0", "");
                            let value: String = percent_decode_str(&param[pos + 1..])
                                .decode_utf8_lossy()
                                .to_string()
                                .replace("\0", "");
                            if !key.is_empty() {
                                tem_query.insert(key, value);
                            }
                        }
                    }
                    tem_query
                }
                _ => HttpRequest::get_query_from_request_url(&full_path),
            };
        }

        // 构造HttpRequest结构体
        let res_request: HttpRequest = HttpRequest {
            method,
            path,
            headers,
            body,
        };

        // 记录日志
        log::write_no_print(server, &res_request);
        Some(res_request)
    }

    /**
     * 获取GET参数
     */
    pub fn get_query_from_request_url(url: &str) -> HashMap<String, String> {
        let mut query_params: HashMap<String, String> = HashMap::new();
        if let Some(query_start) = url.find('?') {
            // 找到第一个 '#' 或者直到字符串结尾
            let query_end: usize = url.find('#').unwrap_or(url.len());
            let query_str: &str = &url[query_start + 1..query_end];
            for pair in query_str.split('&') {
                let mut parts: Split<char> = pair.split('=');
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    query_params.insert(key.to_string(), value.to_string());
                }
            }
        }
        query_params
    }

    /**
     * 生成查询字符串
     */
    pub fn generate_query_string(data: &HashMap<String, String>) -> String {
        if data.is_empty() {
            return String::new();
        }
        let query_pairs: Vec<String> = data
            .iter()
            .map(|(key, value)| {
                // 编码
                let encoded_key: percent_encoding::PercentEncode =
                    percent_encode(&key.as_bytes(), NON_ALPHANUMERIC);
                let encoded_value: percent_encoding::PercentEncode =
                    percent_encode(&value.as_bytes(), NON_ALPHANUMERIC);
                format!("{}={}", encoded_key, encoded_value)
            })
            .collect();
        query_pairs.join("&")
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
        format!(
            "{}://{}{}",
            http_base.scheme, http_base.host, http_base.port
        )
    }

    /**
     * 获取请求协议
     */
    pub fn get_scheme(url: &str) -> String {
        let http_base: HttpBase = HttpRequest::get_http_base(url);
        http_base.scheme
    }

    /**
     * 获取请求IP/域名
     */
    pub fn get_ip_domain(url: &str) -> String {
        let http_base: HttpBase = HttpRequest::get_http_base(url);
        http_base.host
    }

    /**
     * 获取请求端口
     */
    pub fn get_port(url: &str) -> u16 {
        let http_base: HttpBase = HttpRequest::get_http_base(url);
        let port: &str = tools::remove_str_first_char(&http_base.port);
        if port == 0.to_string() {
            let scheme: &str = &HttpRequest::get_scheme(url);
            match scheme {
                HTTPS_SCHEME => HTTPS_PORT,
                _ => HTTP_PORT,
            };
        }
        tools::str_to_number(&port)
    }

    /**
     * 获取请求IP/域名和端口
     */
    pub fn get_ip_domain_port(url: &str) -> String {
        format!(
            "{}:{}",
            HttpRequest::get_ip_domain(url),
            HttpRequest::get_port(url)
        )
    }

    /**
     * 获取请求头HTTP首部
     */
    pub fn get_http_request_protocol_head(url: &str, method: &str) -> String {
        format!("{} {} HTTP/1.1{}", method, url, HEADER_BR)
    }

    /**
     * 生成http请求数据部分
     */
    pub fn generate_http_data(
        method: &str,
        content_type: &str,
        body: &HashMap<String, String>,
    ) -> String {
        let mut http_data: String = String::new();
        if method == POST || method == PUT {
            if content_type == APPLICATION_JSON {
                if !body.is_empty() {
                    let json_body: String = serde_json::to_string(body).unwrap_or(String::new());
                    http_data.push_str(&json_body);
                }
                return http_data;
            }
            let mut url_encoded_body: String = String::new();
            for (key, value) in body.iter() {
                url_encoded_body.push_str(&format!("{}={}&", key, value));
            }
            // 移除末尾的多余的 & 符号
            url_encoded_body.pop();
            http_data.push_str(&url_encoded_body);
            return http_data;
        }
        let mut url_encoded_body: String = String::new();
        for (key, value) in body.iter() {
            url_encoded_body.push_str(&format!("{}={}&", key, value));
        }
        // 移除末尾的多余的 & 符号
        url_encoded_body.pop();
        http_data.push_str(&url_encoded_body);

        http_data
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
        if let Some(http_request) = res {
            let request_path: String = http_request.path.clone();
            // 检查是否有 Host 头
            match http_request.headers.get(&HOST.to_lowercase()) {
                Some(_http_host) => {
                    // 返回 http_request
                    http_request
                }
                None => HttpRequest {
                    method: DEFAULT_METHOD.to_owned(),
                    body: HashMap::new(),
                    path: request_path,
                    headers: HashMap::new(),
                },
            }
        } else {
            HttpRequest {
                method: DEFAULT_METHOD.to_owned(),
                body: HashMap::new(),
                path: String::new(),
                headers: HashMap::new(),
            }
        }
    }
}
