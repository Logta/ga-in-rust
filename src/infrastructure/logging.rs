use std::fmt;

/// Logging levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Simple logger for the application
#[derive(Debug)]
pub struct Logger {
    level: LogLevel,
    prefix: String,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            prefix: "[GA]".to_string(),
        }
    }

    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn debug(&self, message: &str) {
        if self.level <= LogLevel::Debug {
            println!("{} DEBUG: {}", self.prefix, message);
        }
    }

    pub fn info(&self, message: &str) {
        if self.level <= LogLevel::Info {
            println!("{} INFO: {}", self.prefix, message);
        }
    }

    pub fn warn(&self, message: &str) {
        if self.level <= LogLevel::Warn {
            eprintln!("{} WARN: {}", self.prefix, message);
        }
    }

    pub fn error(&self, message: &str) {
        if self.level <= LogLevel::Error {
            eprintln!("{} ERROR: {}", self.prefix, message);
        }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        match level {
            LogLevel::Debug => self.debug(message),
            LogLevel::Info => self.info(message),
            LogLevel::Warn => self.warn(message),
            LogLevel::Error => self.error(message),
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(LogLevel::Info)
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }

    #[test]
    fn test_logger_creation() {
        let logger = Logger::new(LogLevel::Debug);
        assert_eq!(logger.level, LogLevel::Debug);
        assert_eq!(logger.prefix, "[GA]");

        let logger = Logger::default();
        assert_eq!(logger.level, LogLevel::Info);
    }

    #[test]
    fn test_logger_with_prefix() {
        let logger = Logger::new(LogLevel::Info).with_prefix("[TEST]".to_string());
        assert_eq!(logger.prefix, "[TEST]");
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Debug.to_string(), "DEBUG");
        assert_eq!(LogLevel::Info.to_string(), "INFO");
        assert_eq!(LogLevel::Warn.to_string(), "WARN");
        assert_eq!(LogLevel::Error.to_string(), "ERROR");
    }
}
