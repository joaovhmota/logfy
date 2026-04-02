use std::fmt::Display;

use colored::{Color, ColoredString, Colorize};

pub enum LogLevel {
    Debug,
    Information,
    Success,
    Warning,
    Error,
    Critical,
}

impl From<&LogLevel> for Color {
    fn from(value: &LogLevel) -> Self {
        match value {
            LogLevel::Debug => Color::BrightMagenta,
            LogLevel::Information => Color::BrightCyan,
            LogLevel::Success => Color::Green,
            LogLevel::Warning => Color::Yellow,
            LogLevel::Error => Color::Red,
            LogLevel::Critical => Color::BrightWhite,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogLevel::Debug => String::from("DEBUG"),
                LogLevel::Information => String::from("INFORMATION"),
                LogLevel::Success => String::from("SUCCESS"),
                LogLevel::Warning => String::from("WARNING"),
                LogLevel::Error => String::from("ERROR"),
                LogLevel::Critical => String::from("CRITICAL"),
            }
        )
    }
}

impl LogLevel {
    pub fn build(&self) -> ColoredString {
        let label_text = self.to_string();
        let label_foreground_color: Color = self.into();
        let optional_label_background_color = match self {
            LogLevel::Critical => Some(Color::Red),
            _ => None,
        };

        match optional_label_background_color {
            None => label_text.color(label_foreground_color),
            Some(label_background_color) => label_text
                .color(label_foreground_color)
                .on_color(label_background_color),
        }
    }
}
