use crate::config::config::Server;
use crate::global::global::GET_TIME_FAIL;
use crate::log::log;
use crate::utils::time;
use std::{
    fmt, sync,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub const GREEN: &'static str = "\x1B[32m"; // 绿色
pub const RED: &'static str = "\x1B[31m"; // 红色
pub const YELLOW: &'static str = "\x1B[33m"; // 黄色
pub const BLUE: &'static str = "\x1B[34m"; // 蓝色
pub const MAGENTA: &'static str = "\x1B[35m"; // 洋红色
pub const CYAN: &'static str = "\x1B[36m"; // 青色
pub const WHITE: &'static str = "\x1B[37m"; // 白色
const END: &'static str = "\x1B[0m"; // 结束
const PRINTLN_MUTEX: sync::Mutex<()> = sync::Mutex::new(());

fn base_print<T: fmt::Display + fmt::Debug>(str: &T, color: &str, server: &Server, has_br: bool) {
    let mut print_msg: String = String::from("");
    let mut log_msg: String = String::from("");
    let now: Duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(GET_TIME_FAIL);
    let now: String = time::format_now_time();
    let print_mutex: sync::Mutex<()> = PRINTLN_MUTEX;
    let lock: sync::MutexGuard<()> = print_mutex.lock().unwrap();
    if has_br {
        print_msg = format!("{}[{}]{}{}{:#?}{}\n", GREEN, now, END, color, *str, END);
        log_msg = format!("[{}]\n{:#?}\n", now, *str);
    } else {
        print_msg = format!("{}[{}]{}{}{:#?}{}", GREEN, now, END, color, *str, END);
        log_msg = format!("[{}]\n{:#?}", now, *str);
    }
    print!("{}", print_msg);
    drop(lock);
    log::write(&log_msg, server);
}

pub fn print<T: fmt::Display + fmt::Debug>(str: &T, color: &str, server: &Server) {
    base_print(str, color, server, false);
}

pub fn println<T: fmt::Display + fmt::Debug>(str: &T, color: &str, server: &Server) {
    base_print(str, color, server, true);
}
