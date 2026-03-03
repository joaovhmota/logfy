use std::fmt::Arguments;

use chrono::Local;
use colored::{Color, ColoredString, Colorize};

pub enum LogLevel {
    Debug,
    Information,
    Success,
    Warning,
    Error,
    Critical,
}

impl LogLevel {
    pub fn get_label_text(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Information => "INFORMATION",
            LogLevel::Success => "SUCCESS",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
            LogLevel::Critical => "CRITICAL",
        }
    }

    pub fn get_foreground_color(&self) -> Color {
        match self {
            LogLevel::Debug => Color::BrightMagenta,
            LogLevel::Information => Color::BrightCyan,
            LogLevel::Success => Color::Green,
            LogLevel::Warning => Color::Yellow,
            LogLevel::Error => Color::Red,
            LogLevel::Critical => Color::BrightWhite,
        }
    }

    pub fn get_background_color(&self) -> Option<Color> {
        match self {
            LogLevel::Critical => Some(Color::Red),
            _ => None,
        }
    }

    pub fn get_colored_label(&self) -> ColoredString {
        let label_text = self.get_label_text();
        let label_foreground_color = self.get_foreground_color();
        let optional_label_background_color = self.get_background_color();

        match optional_label_background_color {
            None => label_text.color(label_foreground_color),
            Some(label_background_color) => label_text
                .color(label_foreground_color)
                .on_color(label_background_color),
        }
    }
}

pub fn handle_logging(level: LogLevel, message: Arguments) {
    let colored_time = Local::now()
        .format("[%d/%m/%Y %H:%M:%S]")
        .to_string()
        .bright_black();
    let colored_label = level.get_colored_label();
    let colored_message = message.to_string().bright_white();

    println!("{colored_time} {colored_label} {colored_message}");
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::handle_logging($crate::LogLevel::Debug, format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        $crate::handle_logging($crate::LogLevel::Success, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        $crate::handle_logging($crate::LogLevel::Warning, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::handle_logging($crate::LogLevel::Error, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! information {
    ($($arg:tt)*) => {
        $crate::handle_logging($crate::LogLevel::Information, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {
        $crate::handle_logging($crate::LogLevel::Critical, format_args!($($arg)*));
    };
}
