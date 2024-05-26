use crate::config::config::Server;
use crate::global::global::{DEFAULT_HTTP_PORT, PROXY_FAILED, PROXY_SUCCESS, PROXY_URL_INFO};
use crate::http::header::HOST;
use crate::http::request::{HttpBase, HttpRequest};
use crate::http::response;
use crate::print::print::{self, GREEN, RED};
use http::uri::Scheme;
use http::{header, method, request, Uri};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use reqwest::{blocking::Client, header::CONTENT_TYPE, Error as ReqwestError, RequestBuilder};
use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

/**
 * 发送请求
 */
fn send_request(
    server: &Server,
    headers: HashMap<String, String>,
    url: &str,
    method: &str,
    data: &Vec<(String, String)>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client: Client = Client::new();
    let mut request_builder: reqwest::blocking::RequestBuilder;

    match method {
        "GET" => {
            let query_str: String = generate_query_string(data)?;
            let proxy_url: String = format!("{}?{}", url, query_str);
            request_builder = client.get(&proxy_url);
            print::println(
                &format!("{}:\n{}", PROXY_URL_INFO, &proxy_url),
                &GREEN,
                server,
            );
        }
        "POST" => {
            let query_str: String = generate_query_string(data)?;
            request_builder = client
                .post(url)
                .body(query_str.clone())
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded");
            print::println(
                &format!("{}:\n{}\n{}", PROXY_URL_INFO, &url, &query_str),
                &GREEN,
                server,
            );
        }
        "PUT" | "DELETE" => {
            request_builder = client.request(reqwest::Method::from_bytes(method.as_bytes())?, url);
        }
        _ => {
            let query_str: String = generate_query_string(data)?;
            let proxy_url: String = format!("{}?{}", url, query_str);
            request_builder = client.get(&proxy_url);
            print::println(
                &format!("{}:\n{}", PROXY_URL_INFO, &proxy_url),
                &GREEN,
                server,
            );
        }
    }

    for (ref key, ref value) in headers {
        request_builder = request_builder.header(key, value);
    }

    let mut res: reqwest::blocking::Response = request_builder.send()?;

    let mut body: Vec<u8> = Vec::new();
    res.read_to_end(&mut body)?;

    Ok(body)
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
    buffer_size: usize,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut response: Vec<u8> = vec![];
    let mut headers: HashMap<String, String> = request.headers.clone();
    let method: String = request.method.clone();
    let url: String = format!(
        "{}{}",
        HttpRequest::get_url_without_path_query_hash(&server.proxy),
        request.path
    );
    // 查询信息
    let mut query: Vec<(String, String)> = HttpRequest::get_query_from_request_url(&server.proxy);
    // 请求查询信息
    let request_query: Vec<(String, String)> = request.query.clone();
    // 整合信息
    query.extend(request_query);
    // 解析 URI
    let uri: Uri = match Uri::try_from(url.clone()) {
        Ok(uri) => uri,
        Err(e) => {
            print::println(&format!("{}:{}", &PROXY_FAILED, &e), &RED, server);
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

    headers.insert(HOST.to_owned(), format!("{}:{}", host.to_owned(), port));

    match send_request(&server, headers, &url, &method, &query) {
        Ok(body) => {
            response = body;
            let response_str: std::borrow::Cow<str> = String::from_utf8_lossy(&response);
            print::println(
                &format!("{}:\n{}", PROXY_SUCCESS, response_str),
                &GREEN,
                server,
            );
        }
        Err(e) => print::println(&PROXY_FAILED, &RED, server),
    }

    Ok(response)
}
