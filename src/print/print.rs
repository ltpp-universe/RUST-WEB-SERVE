use crate::config::config::Server;
use crate::global::global::GET_TIME_FAIL;
use crate::log::log;
use crate::utils::time;
use std::{
    fmt,
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

pub fn print<T: fmt::Display + fmt::Debug>(str: &T, color: &str, server: &Server) {
    let now: Duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(GET_TIME_FAIL);
    let now: String = time::format_now_time();
    print!("{}[{}]{}", GREEN, now, END);
    print!("{}", color);
    print!("{:#?}", *str);
    print!("{}", END);
    let log_msg: &String = &format!("[{}]\n{:#?}", now, *str);
    log::write(log_msg, server);
}

pub fn println<T: fmt::Display + fmt::Debug>(str: &T, color: &str, server: &Server) {
    print(str, color, server);
    println!();
}
