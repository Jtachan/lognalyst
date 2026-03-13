mod input;
mod log_data;

use input::Config;
use log_data::*;
use std::error::Error;
use std::{env, fs, process};

fn read_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let mut entries: Vec<LogEntry> = Vec::new();

    for line in contents.lines() {
        if let Some(log_entry) = parse_line(line) {
            entries.push(log_entry)
        }
    }
    for entry in entries {
        println!("{entry:?}");
    }
    Ok(())
}

fn parse_line(line: &'_ str) -> Option<LogEntry<'_>> {
    let items: Vec<&str> = line.split_whitespace().collect();

    if items.len() < 4 {
        return None;
    }

    let timestamp = LogTimestamp {
        date: items[0],
        time: items[1],
    };
    let level = LogLevel::from_str(items[2])?;
    let message = items[3..].join(" ");
    Some(LogEntry {
        timestamp,
        level,
        message,
    })
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments -> {err}");
        process::exit(1);
    });
    if let Err(e) = read_file(&config.file_path) {
        eprintln!("[Error] -- {e}");
        process::exit(1);
    }
}
