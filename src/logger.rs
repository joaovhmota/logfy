use std::fmt::Arguments;

use chrono::Local;
use colored::Colorize;

use crate::log_level::LogLevel;

pub fn handle_logging(level: LogLevel, message: Arguments) {
    let colored_time = Local::now()
        .format("[%d/%m/%Y %H:%M:%S]")
        .to_string()
        .bright_black();
    let colored_label = level.get_colored_label();
    let colored_message = message.to_string().bright_white();

    println!("{colored_time} {colored_label} {colored_message}");
}
