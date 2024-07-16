use crate::config::config::Server;
use crate::global::global::{DANGER_PATH, HOTLINK_PROTECTION_MATCH_MSG, REFUSE_LOAD_FILE};
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
        if let Ok(https_regex) = Regex::new(&one_hotlink_protection) {
            if !https_regex.is_match(&source_path) {
                continue;
            }
            print::println(
                &format!(
                    "{} => {}\nError => {}",
                    &REFUSE_LOAD_FILE, &source_path, HOTLINK_PROTECTION_MATCH_MSG
                ),
                RED,
                server,
            );
            return false;
        } else {
            return false;
        };
    }
    return true;
}
