use crate::config::config::{Config, Server};
use crate::global::global::{
    ACCEPTED_TEXT, BAD_GATEWAY_TEXT, BAD_REQUEST_TEXT, CONTINUE_TEXT, CREATED_TEXT, FORBIDDEN_TEXT,
    FOUND_TEXT, INTERNAL_SERVER_ERROR_TEXT, METHOD_NOT_ALLOWED_TEXT, MOVED_PERMANENTLY_TEXT,
    NOT_FOUND_TEXT, NOT_IMPLEMENTED_TEXT, NOT_MODIFIED_TEXT, NO_CONTENT_TEXT, OK_TEXT,
    REQUEST_TIMEOUT_TEXT, SERVICE_UNAVAILABLE_TEXT, SWITCHING_PROTOCOLS_TEXT, UNAUTHORIZED_TEXT,
    UNKNOWN_STATUS_CODE,
};
use crate::ssl::ssl;
use crate::template::template;
use crate::utils::file;
use std::{path, str};

/**
 * 加载其他HTML
 */
pub fn load_other_html(code: usize, server: &Server) -> (Vec<u8>, usize) {
    let try_files_path: String = Config::get_full_try_files_path(&server);
    if let Some(html_res) = file::get_file_data(&try_files_path, server) {
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
    if let Some(html_res) = file::get_file_data(&file_path, server) {
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
 * 响应结果
 */
pub fn response(code: usize, content: &Vec<u8>, server: &Server) -> Vec<u8> {
    let mut res_response: Vec<u8> = vec![];
    let (ssl_certificate, ssl_certificate_key) = ssl::get_ssl(server);
    let () = res_response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n",
        code,
        get_code_msg(code),
        content.len(),
    )
    .into_bytes();
    res_response.extend(content);
    res_response
}
