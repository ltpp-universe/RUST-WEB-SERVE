use crate::config::config::Server;
use crate::file_safe::file_safe;
use crate::global::global::{
    DEFAULT_HTTP_PORT, DELETE, GET, HOST, PARSE_RESPONSE_HEADER_FAILED, POST, PROXY_FAILED,
    PROXY_URL_INFO, PUT, REQUEST_QUERY_INFO,
};
use crate::http::request::HttpRequest;
use crate::http::response;
use crate::print::print::{self, RED, YELLOW};
use http::Uri;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, CONTENT_TYPE},
    Error as ReqwestError,
};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
/**
 * 发送请求
 */
fn send_request(
    server: &Server,
    headers: HashMap<String, String>,
    url: &str,
    method: &str,
    query_str: &String,
    proxy_url: &String,
) -> Result<(HashMap<String, String>, Vec<u8>), Box<dyn std::error::Error>> {
    let client: Client = Client::new();
    let mut request_builder: reqwest::blocking::RequestBuilder;

    match method {
        GET => {
            request_builder = client.get(proxy_url);
            print::println(
                &format!("{} => {}", PROXY_URL_INFO, proxy_url),
                YELLOW,
                server,
            );
        }
        POST => {
            request_builder = client
                .post(url)
                .body(query_str.clone())
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded");
            print::println(
                &format!(
                    "{} => {}\n{} => {}",
                    PROXY_URL_INFO, &url, REQUEST_QUERY_INFO, &query_str
                ),
                YELLOW,
                server,
            );
        }
        PUT | DELETE => {
            request_builder = client.request(reqwest::Method::from_bytes(method.as_bytes())?, url);
        }
        _ => {
            request_builder = client.get(proxy_url);
            print::println(
                &format!("{} => {}", PROXY_URL_INFO, proxy_url),
                YELLOW,
                server,
            );
        }
    }

    for (ref key, ref value) in headers {
        request_builder = request_builder.header(key, value);
    }

    let mut res: Response = request_builder.send()?;

    let headers: HashMap<String, String> = convert_headers_to_hashmap(&server, res.headers());

    let mut body: Vec<u8> = Vec::new();
    res.read_to_end(&mut body)?;

    Ok((headers, body))
}

/**
 * 响应头转HashMap
 */
fn convert_headers_to_hashmap(server: &Server, headers: &HeaderMap) -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();
    for (key, value) in headers.iter() {
        match value.to_str() {
            Ok(tem_value) => {
                hashmap.insert(key.to_string(), tem_value.to_string());
            }
            Err(err) => {
                print::println(
                    &format!("{} => {:?}", PARSE_RESPONSE_HEADER_FAILED, err),
                    RED,
                    server,
                );
            }
        }
    }

    hashmap
}

/**
 * 生成查询字符串
 */
fn generate_query_string(data: &[(String, String)]) -> Result<String, ReqwestError> {
    if data.is_empty() {
        return Ok(String::new());
    }
    let query_pairs: Vec<String> = data
        .iter()
        .map(|(key, value)| {
            let encoded_key = percent_encode(&key.as_bytes(), NON_ALPHANUMERIC);
            let encoded_value = percent_encode(&value.as_bytes(), NON_ALPHANUMERIC);
            format!("{}={}", encoded_key, encoded_value)
        })
        .collect();

    Ok(query_pairs.join("&"))
}

/**
 * 发送请求
 */
pub fn send_sync_request(
    server: &Server,
    request: &HttpRequest,
    proxy_index: usize,
) -> Result<(HashMap<String, String>, Vec<u8>), Box<dyn Error>> {
    let mut request_header: HashMap<String, String> = request.headers.clone();
    let method: String = request.method.clone();
    let url: String = format!(
        "{}{}",
        HttpRequest::get_url_without_path_query_hash(&server.proxy[proxy_index]),
        request.path
    );

    // 查询信息
    let mut query: Vec<(String, String)> =
        HttpRequest::get_query_from_request_url(&server.proxy[proxy_index]);
    // 请求查询信息
    let request_query: Vec<(String, String)> = request.query.clone();
    // 整合信息
    query.extend(request_query);

    // 完整URL的防盗链校验
    let query_str: String = generate_query_string(&query)?;
    let proxy_url: String = format!("{}?{}", url, query_str);
    if !file_safe::check_source_full_path_safe(&server, &proxy_url) {
        let (contents, _code) = response::load_other_html(404, server);
        return Ok((HashMap::new(), contents));
    }

    // 解析 URI
    let uri: Uri = match Uri::try_from(url.clone()) {
        Ok(uri) => uri,
        Err(e) => {
            print::println(&format!("{} => {:?}", &PROXY_FAILED, &e), RED, server);
            return Err(e.into());
        }
    };

    // 获取主机名
    let host: &str = match uri.host() {
        Some(host) => host,
        None => "",
    };

    // 获取端口
    let port: u16 = uri.port_u16().unwrap_or(*DEFAULT_HTTP_PORT as u16);

    // 请求头
    request_header.insert(HOST.to_owned(), format!("{}:{}", host.to_owned(), port));
    let mut response_header: HashMap<String, String> = HashMap::new();
    let mut response_content: Vec<u8> = vec![];

    // 请求
    match send_request(
        &server,
        request_header,
        &url,
        &method,
        &query_str,
        &proxy_url,
    ) {
        Ok((tem_response_header, tem_response_content)) => {
            response_content = tem_response_content.clone();
            response_header = tem_response_header.clone();
        }
        Err(err) => print::println(&format!("{} => {:?}", &PROXY_FAILED, err), RED, server),
    }

    Ok((response_header, response_content))
}
