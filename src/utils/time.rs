use chrono::prelude::*;

pub fn format_now_time() -> String {
    let now: DateTime<Local> = Local::now();
    let formatted_datetime: String = now.format("%Y-%m-%d %H:%M:%S").to_string();
    formatted_datetime
}

pub fn format_now_day() -> String {
    let now: DateTime<Local> = Local::now();
    let formatted_datetime: String = now.format("%Y-%m-%d").to_string();
    formatted_datetime
}
