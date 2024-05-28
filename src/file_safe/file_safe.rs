use crate::config::config::Server;
use crate::global::global::{DANGER_PATH, HOTLINK_PROTECTION_MATCH_MSG, RESOURCE_LOAD_FAIL};
use crate::print::print::{self, RED};
use regex::Regex;

/**
 * 检查资源路径是否安全
 */
pub fn check_source_full_path_safe(server: &Server, source_path: &str) -> bool {
    // 危险路径
    if source_path.contains(DANGER_PATH) {
        return false;
    }
    // 防盗链
    let hotlink_protection: Vec<String> = server.hotlink_protection.clone();
    for one_hotlink_protection in hotlink_protection {
        match Regex::new(&one_hotlink_protection) {
            Ok(https_regex) => {
                if https_regex.is_match(&source_path) {
                    print::println(
                        &format!(
                            "{} => {}\nError => {}",
                            &RESOURCE_LOAD_FAIL, &source_path, HOTLINK_PROTECTION_MATCH_MSG
                        ),
                        RED,
                        server,
                    );
                    return true;
                }
                continue;
            }
            _ => {
                return true;
            }
        };
    }
    return true;
}
