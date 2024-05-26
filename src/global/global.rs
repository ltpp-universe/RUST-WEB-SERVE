use std::fmt::format;

// APP名称
pub const APP_NAME: &'static str = "RUST-WEB-SERVER";
// 默认信息
pub const DEFAULT_HTTP_PORT: &'static usize = &80;
pub const DEFAULT_SCHEME: &'static str = "http";
pub const DEFAULT_HOST: &'static str = "";
pub const DEFAULT_PORT: &'static str = "80";
pub const DEFAULT_PATH: &'static str = "/";
pub const DEFAULT_METHOD: &'static str = "GET";
pub const DANGER_PATH: &'static str = "../";
// 配置
pub const DEFAULT_LISTEN_IP: &'static str = &"127.0.0.1";
pub const DEFAULT_LISTEN_PORT: &'static usize = &80;
pub const DEFAULT_BUFFER_SIZE: &'static usize = &1024;
pub const DEFAULT_ROOT_PATH: &'static str = &"./";
pub const DEFAULT_LOG_DIR_PATH: &'static str = &"./logs";
pub const DEFAULT_SSL_CERTIFICATE_PATH: &'static str = &"./ssl/certificate.crt";
pub const DEFAULT_SSL_CERTIFICATE_KEY_PATH: &'static str = &"./ssl/certificate.key";
pub const DEFAULT_SERVER_NAME: &'static str = DEFAULT_LISTEN_IP;
pub const DEFAULT_EMPTY_PATH_TRY_FILES_PATH: &'static str = &"./index.html";
pub const DEFAULT_RESPONSE_HEADER: &'static str = &"Server: RUST-WEB-SERVER";
pub const DEFAULT_PROXY: &'static str = &"";
pub const PROXY_URL_INFO: &'static str = "Proxy url info";
pub const PROXY_TIMEOUT_SECONDS: &'static usize = &10;
// 成功
pub const RESOURCE_LOAD_SUCCESS: &'static str = "Resource load success";
pub const CONFIG_PATH: &'static str = "config.json";
pub const PROXY_SUCCESS: &'static str = "Proxy success";
// 失败
pub const INVALID_URL: &'static str = "Invalid URL";
pub const INVALID_HOST: &'static str = "Invalid host";
pub const JSON_DECODE_FAIL: &'static str = "Failed to deserialize JSON";
pub const PARSE_HTTP_FAIL: &'static str = "Failed to parse HTTP";
pub const GET_TIME_FAIL: &'static str = "Time went backwards";
pub const GET_MONTH_FAIL: &'static str = "Invalid month";
pub const GET_CONFIG_FAIL: &'static str = "Failed to get config";
pub const RESOURCE_LOAD_FAIL: &'static str = "Resource load fail";
pub const OPEN_LOG_FILE_FAILED: &'static str = "Failed to open log file";
pub const WRITE_LOG_FILE_FAILED: &'static str = "Failed to write to log file";
pub const PROXY_FAILED: &'static str = "Failed to proxy";
// 错误页
pub const DEFAULT_ERROR_HTML:&'static str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>{}</title><style>body{display:flex;justify-content:center}</style></head><body><h1>{}</h1></body></html>";
// 100
pub const CONTINUE_TEXT: &'static str = "Continue";
// 101
pub const SWITCHING_PROTOCOLS_TEXT: &'static str = "Switching Protocols";
// 200
pub const OK_TEXT: &'static str = "OK";
// 201
pub const CREATED_TEXT: &'static str = "Created";
// 202
pub const ACCEPTED_TEXT: &'static str = "Accepted";
// 204
pub const NO_CONTENT_TEXT: &'static str = "No Content";
// 301
pub const MOVED_PERMANENTLY_TEXT: &'static str = "Moved Permanently";
// 302
pub const FOUND_TEXT: &'static str = "Found";
// 304
pub const NOT_MODIFIED_TEXT: &'static str = "Not Modified";
// 400
pub const BAD_REQUEST_TEXT: &'static str = "Bad Request";
// 401
pub const UNAUTHORIZED_TEXT: &'static str = "Unauthorized";
// 403
pub const FORBIDDEN_TEXT: &'static str = "Forbidden";
// 404
pub const NOT_FOUND_TEXT: &'static str = "Not Found";
// 405
pub const METHOD_NOT_ALLOWED_TEXT: &'static str = "Method Not Allowed";
// 408
pub const REQUEST_TIMEOUT_TEXT: &'static str = "Request Timeout";
// 500
pub const INTERNAL_SERVER_ERROR_TEXT: &'static str = "Internal Server Error";
// 501
pub const NOT_IMPLEMENTED_TEXT: &'static str = "Not Implemented";
// 502
pub const BAD_GATEWAY_TEXT: &'static str = "Bad Gateway";
// 503
pub const SERVICE_UNAVAILABLE_TEXT: &'static str = "Service Unavailable";
// 未知状态码
pub const UNKNOWN_STATUS_CODE: &'static str = "Unknown Status Code";
