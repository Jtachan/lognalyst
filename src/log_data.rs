//! Information directly related with LOG files and their contents.

/// Enumeration for all the valid log levels that can be contained within a log file.
#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    INFO,
    ERROR,
    WARN,
    DEBUG,
}

impl LogLevel {
    /// Initialization of the enumeration from a string value, which is internally normalized.
    ///
    /// # Example
    /// ```
    /// let info_level = LogLevel::from_str("info").unwrap();
    /// dbg!(info_level);  // LogLevel::INFO
    ///
    /// let wrong_level = LogLevel::from_str("invalid");
    /// dgb!(wrong_level)  // None
    /// ```
    fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "INFO" => Some(LogLevel::INFO),
            "ERROR" => Some(LogLevel::ERROR),
            "WARN" => Some(LogLevel::WARN),
            "DEBUG" => Some(LogLevel::DEBUG),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Timestamp<'a> {
    pub date: &'a str,
    pub time: &'a str,
}

#[derive(Debug, Clone)]
pub struct LogEntry<'a> {
    pub timestamp: Timestamp<'a>,
    pub level: LogLevel,
    pub message: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_level_from_str() {
        assert_eq!(LogLevel::from_str("INFO").unwrap(), LogLevel::INFO);
        assert_eq!(LogLevel::from_str("ErrOr").unwrap(), LogLevel::ERROR);
        assert_eq!(LogLevel::from_str("warn").unwrap(), LogLevel::WARN);
        assert_eq!(LogLevel::from_str("DEBug").unwrap(), LogLevel::DEBUG);
        assert_eq!(LogLevel::from_str("other"), None);
    }
}