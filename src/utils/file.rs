use crate::config::config::Server;
use crate::file_safe::file_safe;
use crate::global::global::{
    HOTLINK_PROTECTION_MATCH_MSG, RESOURCE_LOAD_FAIL, RESOURCE_LOAD_SUCCESS,
};
use crate::http::response;
use crate::print::print::{self, GREEN, RED};
use std::{fs, io};

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

pub fn get_file_data(file_path: &str, server: &Server) -> Option<Vec<u8>> {
    let mut contents_result: io::Result<Vec<u8>> = Ok(vec![]);
    if let Ok(metadata) = fs::metadata(&file_path) {
        if metadata.is_file() {
            contents_result = fs::read(&file_path);
            match contents_result {
                Ok(contents) => {
                    print::println(
                        &format!("{} => {}", &RESOURCE_LOAD_SUCCESS, &file_path),
                        &GREEN,
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
                        &RED,
                        server,
                    );
                }
            }
        }
    }
    None
}
