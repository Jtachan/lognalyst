//! Structs and functions related to obtain and display statistics out of the log entries.
use super::log_data::{LogEntry, LogLevel};
use std::collections::HashMap;

pub struct LogStatistics {
    total_entries: usize,
    level_counts: HashMap<LogLevel, usize>,
}

impl LogStatistics {
    /// Creates empty statistics.
    pub fn new() -> Self {
        let level_counts = HashMap::from([
            (LogLevel::Debug, 0),
            (LogLevel::Info, 0),
            (LogLevel::Warning, 0),
            (LogLevel::Error, 0),
            (LogLevel::Critical, 0),
        ]);
        LogStatistics {
            total_entries: 0,
            level_counts,
        }
    }
    /// Updates the count based on an entry's level.
    pub fn add_entry(&mut self, entry: &LogEntry) {
        self.total_entries += 1;
        *self.level_counts.get_mut(&entry.level).unwrap() += 1;
    }

    pub fn display(&self) {
        println!("===== File Log Statistics =====");
        println!("Nof. Entries: {}", self.total_entries);
        println!("Breakdown by level:");

        let mut sorted_counts: Vec<_> = self.level_counts.iter().collect();
        sorted_counts.sort_by_key(|(level, _)| match level {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warning => 2,
            LogLevel::Error => 3,
            LogLevel::Critical => 4,
        });
        for (level, count) in sorted_counts {
            println!("   {:8} --> {}", level.to_string(), count);
        }
    }
}
