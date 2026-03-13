//! Structs and functions related to obtain and display statistics out of the log entries.
use super::log_data::{LogEntry, LogLevel};

pub struct LogStatistics {
    total_entries: usize,
    info_count: usize,
    error_count: usize,
    warn_count: usize,
    debug_count: usize,
}

impl LogStatistics {
    /// Creates empty statistics.
    pub fn new() -> Self {
        LogStatistics {
            total_entries: 0,
            error_count: 0,
            warn_count: 0,
            info_count: 0,
            debug_count: 0
        }
    }
    /// Updates the count based on an entry's level.
    pub fn add_entry(&mut self, entry: &LogEntry) {
        self.total_entries += 1;
        match entry.level {
            LogLevel::INFO => self.info_count += 1,
            LogLevel::ERROR => self.error_count += 1,
            LogLevel::WARN => self.warn_count += 1,
            LogLevel::DEBUG => self.debug_count += 1,
        };
    }

    pub fn display(&self) {
        println!("");
    }
}