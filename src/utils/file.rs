use crate::config::config::Server;
use crate::global::global::RESOURCE_LOAD_SUCCESS;
use crate::print::print::{println, GREEN};
use std::fs;

pub fn dir_exists(path: &str) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

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

pub fn get_file_data(file_path: &str, server: &Server) -> Option<String> {
    let mut contents: String = String::from("");
    if let Ok(metadata) = fs::metadata(&file_path) {
        if metadata.is_file() {
            contents = fs::read_to_string(&file_path).unwrap();
            println(
                &format!("{}:{}", &RESOURCE_LOAD_SUCCESS, &file_path),
                &GREEN,
                server,
            );
            return Some(contents);
        }
    }
    None
}
