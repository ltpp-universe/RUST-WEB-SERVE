include!("../global/mod.rs");
include!("../utils/mod.rs");
use global::GET_TIME_FAIL;
use std::fmt::{Debug, Display};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time::format_time;

pub const GREEN: &'static str = "\x1B[32m"; // 绿色
pub const RED: &'static str = "\x1B[31m"; // 红色
pub const YELLOW: &'static str = "\x1B[33m"; // 黄色
pub const BLUE: &'static str = "\x1B[34m"; // 蓝色
pub const MAGENTA: &'static str = "\x1B[35m"; // 洋红色
pub const CYAN: &'static str = "\x1B[36m"; // 青色
pub const WHITE: &'static str = "\x1B[37m"; // 白色
const END: &'static str = "\x1B[0m"; // 结束

pub fn print<T: Display + Debug>(str: &T, color: &str) {
    let color: &str = if color.is_empty() { GREEN } else { color };
    let now: Duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(GET_TIME_FAIL);
    print!("{}{:?}{}", GREEN, format_time(now), END);
    print!("{}", color);
    print!("{:?}", *str);
    print!("{}", END);
}

pub fn println<T: Display + Debug>(str: &T, color: &str) {
    print(str, color);
    println!();
}
