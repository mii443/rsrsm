use chrono::Local;
use colored::Colorize;

pub fn log(kind: &str, color: &str, message: &str) {
    let time = Local::now();
    let time = time.format("%m-%d %H:%M:%S");

    println!("[{} {}]: {}", time, kind.color(color), message);
}
