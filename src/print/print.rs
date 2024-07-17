use crate::config::config::Server;
use crate::log::log;
use crate::utils::time;
use std::{fmt, sync};

pub const GREEN: &'static str = "\x1B[32m"; // 绿色
pub const RED: &'static str = "\x1B[31m"; // 红色
pub const YELLOW: &'static str = "\x1B[33m"; // 黄色
pub const BLUE: &'static str = "\x1B[34m"; // 蓝色
const END: &'static str = "\x1B[0m"; // 结束
const PRINTLN_MUTEX: sync::Mutex<()> = sync::Mutex::new(());

fn base_print<T: fmt::Display + fmt::Debug>(str: &T, color: &str, server: &Server, has_br: bool) {
    let mut _print_msg: String = String::new();
    let mut _log_msg: String = String::new();
    let now: String = time::format_now_time();
    let print_mutex: sync::Mutex<()> = PRINTLN_MUTEX;
    let lock: sync::MutexGuard<()> = match print_mutex.lock() {
        Ok(tem_lock) => tem_lock,
        _ => {
            return;
        }
    };
    if has_br {
        _print_msg = format!("{}[{}]\n{}{}{}{}\n", GREEN, now, END, color, *str, END);
        _log_msg = format!("[{}]\n{}\n", now, *str);
    } else {
        _print_msg = format!("{}[{}]\n{}{}{}{}", GREEN, now, END, color, *str, END);
        _log_msg = format!("[{}]\n{}", now, *str);
    }
    print!("{}", _print_msg);
    drop(lock);
    log::write(server, &_log_msg);
}

pub fn println<T: fmt::Display + fmt::Debug>(str: &T, color: &str, server: &Server) {
    base_print(str, color, server, true);
}
