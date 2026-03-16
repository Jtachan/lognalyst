//! Information directly related with LOG files and their contents.

use std::fmt::{Display, Formatter, Result};

/// Enumeration for all the valid log levels that can be contained within a log file.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl LogLevel {
    /// Initialization of the enumeration from a string value, which is internally normalized.
    ///
    /// # Example
    /// ```
    /// let info_level = LogLevel::from_str("info").unwrap();
    /// dbg!(info_level);  // LogLevel::Info
    ///
    /// let wrong_level = LogLevel::from_str("invalid");
    /// dgb!(wrong_level)  // None
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "DEBUG" => Some(LogLevel::Debug),
            "INFO" => Some(LogLevel::Info),
            "WARN" => Some(LogLevel::Warning),
            "WARNING" => Some(LogLevel::Warning),
            "ERROR" => Some(LogLevel::Error),
            "CRITICAL" => Some(LogLevel::Critical),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogTimestamp<'a> {
    // todo: check the crate 'chrono'
    pub date: &'a str,
    pub time: &'a str,
}

#[derive(Debug, Clone)]
pub struct LogEntry<'a> {
    pub timestamp: LogTimestamp<'a>,
    pub level: LogLevel,
    pub message: String,
}

impl LogEntry<'_> {
    pub fn display(&self) {
        println!("-----------\nLog Entry\n-----------");
        println!("[Level]   : {}", &self.level);
        println!(
            "[Time]    : {} {}",
            &self.timestamp.date, &self.timestamp.time
        );
        println!("[Message] : {}\n", &self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_level_from_str() {
        assert_eq!(LogLevel::from_str("DEBug").unwrap(), LogLevel::Debug);
        assert_eq!(LogLevel::from_str("Info").unwrap(), LogLevel::Info);
        assert_eq!(LogLevel::from_str("warn").unwrap(), LogLevel::Warning);
        assert_eq!(LogLevel::from_str("ErrOr").unwrap(), LogLevel::Error);
        assert_eq!(LogLevel::from_str("CRITICAL").unwrap(), LogLevel::Critical);
        assert_eq!(LogLevel::from_str("other"), None);
    }
}
