use crate::config::config::Server;
use crate::global::global::NOT_FOUND_TEXT;
use crate::template::template;
use crate::utils::file;
use std::path;

/**
 * 加载其他HTML
 */
pub fn load_other_html(code: usize, server: &Server) -> String {
    let mut html_file_name: String = format!("{}.html", code);
    let mut html: String = template::get_error_html(NOT_FOUND_TEXT);
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
    html
}
