pub fn format_time(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;
    format!(
        "{:02}:{:02}:{:02}:{:02}",
        days,
        hours % 24,
        mins % 60,
        secs % 60
    )
}
