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
