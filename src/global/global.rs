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
pub const HTTP_SCHEME: &'static str = "http";
pub const HTTPS_SCHEME: &'static str = "https";
pub const HTTP_PORT: &'static usize = &80;
pub const HTTPS_PORT: &'static usize = &443;
// 类型
pub trait ReadWrite: std::io::Read + std::io::Write {}
impl<T: std::io::Read + std::io::Write> ReadWrite for T {}
// 数据
pub const NOT_PROXY: &'static str = "";
pub const HEADER_BR: &'static str = "\r\n";
pub const HEADER_BR_DOUBLE: &'static str = "\r\n\r\n";
pub const HOTLINK_PROTECTION_MATCH_MSG: &'static str = "Matched to hotlink protection";
pub const USER_AGENT: &'static str = "user-agent";
pub const ACCEPT: &'static str = "accept";
pub const CONTENT_TYPE: &'static str = "content-type";
pub const CONTENT_LENGTH: &'static str = "content-length";
pub const CONNECTION: &'static str = "connection";
pub const ACCEPT_ENCODING: &'static str = "accept-encoding";
pub const ACCEPT_LANGUAGE: &'static str = "accept-language";
pub const CONTENT_ENCODING: &'static str = "content-encoding";
pub const HOST: &'static str = "host";
pub const GET: &'static str = "GET";
pub const POST: &'static str = "POST";
pub const PUT: &'static str = "PUT";
pub const DELETE: &'static str = "DELETE";
pub const REFERER: &'static str = "referer";
pub const ORIGIN: &'static str = "origin";
pub const COOKIE: &'static str = "cookie";
pub const GZIP: &'static str = "gzip";
pub const BINDING: &'static str = "Binding";
pub const LISTENING: &'static str = "Listening";
pub const TEXT_HTML: &'static str = "text/html";
pub const HTTP_HTTPS_REGEX: &'static str = &r"^https?://[^\s/$.?#].[^\s]*$";
pub const KEEP_ALIVE: &'static str = "keep-alive";
pub const CLOSE: &'static str = "close";
pub const APPLICATION_X_WWW_FORM_URLENCODED: &'static str = "application/x-www-form-urlencoded";
pub const APPLICATION_JSON: &'static str = "application/json";
// 配置
pub const DEFAULT_LISTEN_IP: &'static str = "0.0.0.0";
pub const LOCAL_LISTEN_IP: &'static str = "127.0.0.1";
pub const LOCALHOST_LISTEN_IP: &'static str = "localhost";
pub const DEFAULT_LISTEN_PORT: &'static usize = &80;
pub const DEFAULT_BUFFER_SIZE: &'static usize = &10240;
pub const DEFAULT_GZIP_LEVEL: &'static usize = &5;
pub const DEFAULT_ROOT_PATH: &'static str = "./";
pub const DEFAULT_LOG_DIR_PATH: &'static str = "./logs";
pub const DEFAULT_SERVER_NAME: &'static str = DEFAULT_LISTEN_IP;
pub const DEFAULT_EMPTY_PATH_TRY_FILES_PATH: &'static str = "./index.html";
pub const DEFAULT_RESPONSE_HEADER: &'static str = "Access-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: *\r\nAccess-Control-Allow-Headers: *\r\nAccess-Control-Allow-Credentials: true\r\nConnection: keep-alive\r\nContent-Type: text/html\r\nContent-Encoding: gzip\r\nServer: RUST-WEB-SERVER";
pub const DEFAULT_PROXY: &'static str = "";
pub const PROXY_TIMEOUT_SECONDS: &'static usize = &10;
pub const DEFAULT_HOTLINK_PROTECTION: &'static str = "";
// 日志
pub const PROXY_URL_INFO: &'static str = "Proxy url info";
pub const PROXY_REQUEST_INFO: &'static str = "Proxy request info";
pub const REQUEST_RESPONSE_INFO: &'static str = "Request response info";
pub const REQUEST_QUERY_INFO: &'static str = "Request query info";
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
pub const REFUSE_LOAD_FILE: &'static str = "Refuse load file";
pub const OPEN_LOG_FILE_FAILED: &'static str = "Failed to open log file";
pub const WRITE_LOG_FILE_FAILED: &'static str = "Failed to write to log file";
pub const PROXY_FAILED: &'static str = "Failed to proxy";
pub const PARSE_RESPONSE_HEADER_FAILED: &'static str = "Failed to parse response header";
pub const FAILED_TO_LOCK_THE_LISTENER: &'static str = "Failed to lock the listener";
pub const FAILED_TO_CREATE_DEFAULT_SSL_SERVERCONFIG: &'static str =
    "Failed to create default ssl serverconfig";
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
