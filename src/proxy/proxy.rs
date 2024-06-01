use crate::config::config::Server;
use crate::file_safe::file_safe;
use crate::global::global::{
    ReadWrite, APPLICATION_X_WWW_FORM_URLENCODED, CLOSE, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE,
    DEFAULT_HTTP_PORT, HEADER_BR, HEADER_BR_DOUBLE, HOST, HTTPS_SCHEME, ORIGIN,
    PARSE_RESPONSE_HEADER_FAILED, POST, PROXY_FAILED, PROXY_REQUEST_INFO, PROXY_URL_INFO, REFERER,
};
use crate::http::request::HttpRequest;
use crate::http::response;
use crate::print::print::{self, RED, YELLOW};
use crate::utils::tools;
use http::{HeaderMap, Uri};
use native_tls::TlsConnector;
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

/**
 * 获取响应结果
 */
fn read_response(reader: &mut dyn BufRead) -> Result<String, Box<dyn std::error::Error>> {
    let mut line: String = String::new();
    let mut response: String = String::new();
    loop {
        line.clear();
        if let Ok(_bytes_read) = reader.read_line(&mut line) {
            if response::judge_data_read_end(&line) {
                break;
            }
            response.push_str(&line);
        } else {
            break;
        }
    }
    Ok(response)
}

fn send_request(
    server: &Server,
    headers: &mut HashMap<String, String>,
    url: &str,
    host: &str,
    port: u16,
    method: &str,
    body: &str,
    proxy_url: &str,
) -> Result<(HashMap<String, String>, Vec<u8>), Box<dyn Error>> {
    let scheme: String = HttpRequest::get_scheme(proxy_url);

    let mut stream: Box<dyn ReadWrite> = if scheme == HTTPS_SCHEME {
        // 加密连接
        let tls_connector: TlsConnector = TlsConnector::builder().build()?;
        let tcp_stream: TcpStream = TcpStream::connect((host.clone(), port))?;
        tcp_stream.set_read_timeout(Some(Duration::from_secs(
            server.proxy_timeout_seconds as u64,
        )))?;
        Box::new(tls_connector.connect(&host, tcp_stream)?)
    } else {
        // 普通连接
        let tcp_stream: TcpStream = TcpStream::connect((host.clone(), port))?;
        tcp_stream.set_read_timeout(Some(Duration::from_secs(
            server.proxy_timeout_seconds as u64,
        )))?;
        Box::new(tcp_stream)
    };

    let request: String = HttpRequest::get_http_request_protocol_head(&proxy_url, method);
    let body_len: usize = body.len();

    print::println(
        &format!("{} => {}", PROXY_URL_INFO, proxy_url),
        YELLOW,
        server,
    );

    headers.insert(
        ORIGIN.to_lowercase(),
        HttpRequest::get_url_without_query_hash(url),
    );
    headers.insert(REFERER.to_lowercase(), url.to_string());
    headers.insert(HOST.to_lowercase(), HttpRequest::get_ip_domain(proxy_url));

    if body_len == 0 {
        headers.remove(&CONTENT_LENGTH.to_lowercase());
        headers.remove(&CONTENT_TYPE.to_lowercase());
    } else {
        headers.insert(CONTENT_LENGTH.to_lowercase(), body_len.to_string());
        headers.insert(
            CONTENT_TYPE.to_lowercase(),
            APPLICATION_X_WWW_FORM_URLENCODED.to_string(),
        );
    }

    headers.insert(CONNECTION.to_lowercase(), CLOSE.to_string());

    let mut headers_str: String = String::new();

    for (key, value) in headers.into_iter() {
        headers_str.push_str(&format!("{}: {}{}", key, value, HEADER_BR));
    }

    // 请求头结束需要一个空行
    headers_str.push_str(HEADER_BR);

    print::println(
        &format!(
            "{}:\n{}{}{}",
            PROXY_REQUEST_INFO, request, headers_str, body
        ),
        YELLOW,
        server,
    );

    // 请求行
    stream.write_all(request.as_bytes())?;
    // 请求头
    stream.write_all(headers_str.as_bytes())?;
    //请求数据
    stream.write_all(body.as_bytes())?;

    let mut reader: BufReader<&mut dyn ReadWrite> = BufReader::new(stream.as_mut());

    let response: String = read_response(&mut reader)?;

    let mut headers_map: HashMap<String, String> = HashMap::new();
    let mut response_body: Vec<u8> = Vec::new();
    if let Some((header_part, body_part)) = response.split_once(HEADER_BR_DOUBLE) {
        for line in header_part.lines() {
            if let Some((key, value)) = line.split_once(": ") {
                headers_map.insert(key.to_string(), value.to_string());
            }
        }
        response_body.extend_from_slice(body_part.as_bytes());
    }

    Ok((headers_map, response_body))
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
    let mut body: HashMap<String, String> =
        HttpRequest::get_query_from_request_url(&server.proxy[proxy_index]);
    // 请求查询信息
    let request_query: HashMap<String, String> = request.body.clone();
    // 整合信息
    body.extend(request_query);

    // 完整URL的防盗链校验
    let query_str: String = match request.method.as_str() {
        POST => url.clone(),
        _ => HttpRequest::generate_query_string(&body),
    };
    let proxy_url: String = format!("{}?{}", url, query_str);
    if !file_safe::check_source_full_path_safe(&server, &proxy_url) {
        let (contents, _code) = response::load_other_html(404, server);
        return Ok((HashMap::new(), contents));
    }

    // 构造请求数据部分
    let body_str: String = HttpRequest::generate_http_data(
        &method,
        &tools::get_hash_map_one_value(&request_header, &CONTENT_TYPE.to_lowercase()),
        &body,
    );

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
        &mut request_header,
        &url,
        &host,
        port,
        &method,
        &body_str,
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
