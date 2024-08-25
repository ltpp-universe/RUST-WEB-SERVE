use crate::config::config::{Config, Server};
use crate::global::global::{
    ACCEPTED_TEXT, BAD_GATEWAY_TEXT, BAD_REQUEST_TEXT, CONTENT_ENCODING, CONTENT_LENGTH,
    CONTENT_TYPE, CONTINUE_TEXT, CREATED_TEXT, FORBIDDEN_TEXT, FOUND_TEXT, GZIP, HEADER_BR,
    HEADER_BR_DOUBLE, INTERNAL_SERVER_ERROR_TEXT, METHOD_NOT_ALLOWED_TEXT, MOVED_PERMANENTLY_TEXT,
    NOT_FOUND_TEXT, NOT_IMPLEMENTED_TEXT, NOT_MODIFIED_TEXT, NOT_PROXY, NO_CONTENT_TEXT, OK_TEXT,
    PROXY_FAILED, REQUEST_TIMEOUT_TEXT, RESOURCE_LOAD_FAIL, SERVICE_UNAVAILABLE_TEXT,
    SWITCHING_PROTOCOLS_TEXT, UNAUTHORIZED_TEXT, UNKNOWN_STATUS_CODE,
};
use crate::gzip::gzip;
use crate::http::request::HttpRequest;
use crate::print::print::{self, RED};
use crate::proxy;
use crate::request::request::Request;
use crate::template::template;
use crate::utils::{file, tools};
use std::collections::HashMap;
use std::{path, str};

/**
 * 加载其他HTML
 */
pub fn load_other_html(code: usize, server: &Server) -> (Vec<u8>, usize) {
    let try_files_path: String = Config::get_full_try_files_path(&server);
    if let Some(html_res) = file::get_file_data(server, &try_files_path) {
        return (html_res, 200);
    }
    let mut html: Vec<u8> = template::get_error_html(&format!("{} {}", code, NOT_FOUND_TEXT));
    let mut html_file_name: String = format!("{}.html", code);
    let mut root_path: String = server.root_path.clone();
    if let Some(unix_path_str) = path::PathBuf::from(&root_path).to_str() {
        root_path = unix_path_str.replace("\\", "/");
    }
    if let Some(unix_path_str) = path::PathBuf::from(&html_file_name).to_str() {
        html_file_name = unix_path_str.replace("\\", "/");
    }
    if root_path.ends_with('/') {
        root_path.pop();
    }
    if html_file_name.starts_with("/") {
        html_file_name.remove(0);
    }
    let file_path: String = format!("{}/{}", root_path, html_file_name);
    if let Some(html_res) = file::get_file_data(server, &file_path) {
        html = html_res;
    }
    (html, code)
}

/**
 * 获取状态码对应映射信息
 */
pub fn get_code_msg(code: usize) -> String {
    let code_msg = match code {
        100 => (*CONTINUE_TEXT).to_owned(),
        101 => (*SWITCHING_PROTOCOLS_TEXT).to_owned(),
        200 => (*OK_TEXT).to_owned(),
        201 => (*CREATED_TEXT).to_owned(),
        202 => (*ACCEPTED_TEXT).to_owned(),
        204 => (*NO_CONTENT_TEXT).to_owned(),
        301 => (*MOVED_PERMANENTLY_TEXT).to_owned(),
        302 => (*FOUND_TEXT).to_owned(),
        304 => (*NOT_MODIFIED_TEXT).to_owned(),
        400 => (*BAD_REQUEST_TEXT).to_owned(),
        401 => (*UNAUTHORIZED_TEXT).to_owned(),
        403 => (*FORBIDDEN_TEXT).to_owned(),
        404 => (*NOT_FOUND_TEXT).to_owned(),
        405 => (*METHOD_NOT_ALLOWED_TEXT).to_owned(),
        408 => (*REQUEST_TIMEOUT_TEXT).to_owned(),
        500 => (*INTERNAL_SERVER_ERROR_TEXT).to_owned(),
        501 => (*NOT_IMPLEMENTED_TEXT).to_owned(),
        502 => (*BAD_GATEWAY_TEXT).to_owned(),
        503 => (*SERVICE_UNAVAILABLE_TEXT).to_owned(),
        _ => (*UNKNOWN_STATUS_CODE).to_owned(),
    };
    code_msg
}

/**
 * 获取最终响应结果
 */
pub fn get_res_response(
    server: &Server,
    http_request: &HttpRequest,
    is_safe_request: bool,
    file_path: &str,
) -> Vec<u8> {
    let (mut response_header, mut response_content) = (HashMap::new(), Vec::new());
    let mut load_success: bool = false;
    let mut res_response: Vec<u8> = vec![];
    if is_safe_request {
        let proxy_url: String = Request::judge_need_proxy(server);
        if proxy_url == *NOT_PROXY {
            if let Some(html_res) = file::get_file_data(server, &file_path) {
                load_success = true;
                response_content = html_res;
            }
        } else {
            match proxy::proxy::send_sync_request(&server, &http_request, &proxy_url) {
                Ok((header, body)) => {
                    response_header = header;
                    response_content = body;
                }
                Err(err) => {
                    print::println(
                        &format!("{} => {:?} => {:?}", &PROXY_FAILED, &http_request, err),
                        RED,
                        server,
                    );
                }
            };
            load_success = true;
        }
        res_response = edit_response(
            server,
            file_path,
            http_request,
            200,
            &response_header,
            &response_content,
        );
    }

    if !load_success || !is_safe_request || res_response.len() == 0 {
        let (contents, code) = load_other_html(404, server);
        print::println(
            &format!("{} => {}", &RESOURCE_LOAD_FAIL, &file_path),
            RED,
            server,
        );
        res_response = edit_response(
            server,
            file_path,
            http_request,
            code,
            &response_header,
            &contents,
        );
    }
    res_response
}

/**
 * 获取响应头HTTP首部
 */
pub fn get_http_response_protocol_head(code: usize) -> String {
    format!("HTTP/1.1 {} {}", code, get_code_msg(code))
}

/**
 * 判断读取完成
 */
pub fn judge_data_read_end(now: &str) -> bool {
    now == HEADER_BR_DOUBLE || now.len() == 0
}

/**
 * 整合响应结果信息
 */
pub fn edit_response(
    server: &Server,
    file_path: &str,
    http_request: &HttpRequest,
    code: usize,
    response_header: &HashMap<String, String>,
    response_content: &Vec<u8>,
) -> Vec<u8> {
    let response_header_clone: HashMap<String, String> = response_header.clone();
    // 响应头避免大小写导致重复添加，均转小写
    let mut header: HashMap<String, String> =
        tools::parse_string_array_to_hashmap(&server.response_header_list)
            .iter()
            .map(|(k, v)| (k.to_lowercase(), v.clone()))
            .collect();
    for (key, value) in &response_header_clone {
        header.insert(key.to_lowercase(), value.clone());
    }

    // 先去除CONTENT_LENGTH响应头防止重复添加
    header.remove(&CONTENT_LENGTH.to_lowercase());
    // 是否需要开启GZIP
    let mut is_need_open_gzip: bool = gzip::judge_need_open_gzip(&http_request.headers, &header);

    // 如果代理服务器开启了GZIP，此服务器不在GZIP，代理后端响应头透传GZIP响应头，防止二次GZIP
    let encoding: String =
        tools::get_hash_map_one_value(&response_header_clone, &CONTENT_ENCODING.to_lowercase());
    let request_server_open_encoding: bool = encoding.contains(GZIP);
    if request_server_open_encoding {
        is_need_open_gzip = false;
    }

    // 资源不需要gzip且代理服务器无gzip去除响应头CONTENT_ENCODING
    if !is_need_open_gzip && !request_server_open_encoding {
        header.remove(&CONTENT_ENCODING.to_lowercase());
    }

    // 本地资源更新响应头CONTENT_TYPE
    let is_local_file_request: bool = response_header_clone.len() == 0;
    if is_local_file_request {
        header.insert(CONTENT_TYPE.to_owned(), file::get_content_type(file_path));
    }

    let mut res_content: Vec<u8> = response_content.to_vec();
    if is_need_open_gzip {
        res_content = gzip::encoder(&response_content, server.gzip_level as u32);
        header.insert(CONTENT_ENCODING.to_lowercase(), GZIP.to_string());
    }

    let response_header_str: String = tools::hash_map_to_string(&header, HEADER_BR);

    // 最终结果字符串
    let res_response_header_str: String = format!(
        "{}{}{}: {}{}{}{}",
        get_http_response_protocol_head(code),
        HEADER_BR,
        CONTENT_LENGTH.to_lowercase(),
        res_content.len(),
        HEADER_BR,
        response_header_str,
        HEADER_BR_DOUBLE,
    );

    let mut res_response: Vec<u8> = res_response_header_str.clone().into_bytes();

    res_response.extend(res_content);

    res_response
}
