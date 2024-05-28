use crate::config::config::Server;
use crate::global::global::{RESOURCE_LOAD_FAIL, RESOURCE_LOAD_SUCCESS, TEXT_HTML};
use crate::print::print::{self, GREEN, RED};
use mime_guess::from_path;
use std::path::Path;
use std::{fs, io};

/**
 * 获取文件类型
 */
pub fn get_content_type(file_path: &str) -> String {
    let path = Path::new(file_path);
    match from_path(path).first().map(|mime| mime.to_string()) {
        Some(file_type) => file_type,
        _ => TEXT_HTML.to_owned(),
    }
}

/**
 * 路径是否存在
 */
pub fn dir_exists(path: &str) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

/**
 * 创建目录和子级目录
 */
pub fn judge_creat_dir(dir_path: &str) -> bool {
    let mut res: bool = false;
    if !dir_exists(dir_path) {
        // 递归创建目录
        if let Ok(_) = fs::create_dir_all(dir_path) {
            res = true;
        }
    }
    res
}

/**
 * 获取文件数据
 */
pub fn get_file_data(server: &Server, file_path: &str) -> Option<Vec<u8>> {
    let mut _contents_result: io::Result<Vec<u8>> = Ok(vec![]);
    if let Ok(metadata) = fs::metadata(&file_path) {
        if metadata.is_file() {
            _contents_result = fs::read(&file_path);
            match _contents_result {
                Ok(contents) => {
                    print::println(
                        &format!("{} => {}", &RESOURCE_LOAD_SUCCESS, &file_path),
                        GREEN,
                        server,
                    );
                    return Some(contents);
                }
                Err(err) => {
                    print::println(
                        &format!(
                            "{} => {}\nError => {}",
                            &RESOURCE_LOAD_FAIL, &file_path, err
                        ),
                        RED,
                        server,
                    );
                }
            }
        }
    }
    None
}
