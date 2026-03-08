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
