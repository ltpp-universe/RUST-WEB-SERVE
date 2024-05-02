// 配置
pub const DEFAULT_LISTEN_IP: &'static str = &"0.0.0.0";
pub const DEFAULT_LISTEN_PORT: &'static usize = &80;
pub const DEFAULT_BUFFER_SIZE: &'static usize = &1024;
pub const DEFAULT_ROOT_PATH: &'static str = &"./";
pub const DEFAULT_LOG_DIR_PATH: &'static str = &"./logs";
// 成功
pub const RESOURCE_LOAD_SUCCESS: &'static str = "Resource load success";
pub const CONFIG_PATH: &'static str = "config.json";
// 失败
pub const JSON_DECODE_FAIL: &'static str = "Failed to deserialize JSON";
pub const PARSE_HTTP_FAIL: &'static str = "Failed to parse HTTP";
pub const GET_TIME_FAIL: &'static str = "Time went backwards";
pub const GET_MONTH_FAIL: &'static str = "Invalid month";
pub const GET_CONFIG_FAIL: &'static str = "Failed to get config";
pub const RESOURCE_LOAD_FAIL: &'static str = "Resource load fail";
pub const OPEN_LOG_FILE_FAILED: &'static str = "Failed to open log file";
pub const WRITE_LOG_FILE_FAILED: &'static str = "Failed to write to log file";
// 404
pub const NOT_FOUND_HTML: &'static str = "<h1>404 Not Found</h1>";
