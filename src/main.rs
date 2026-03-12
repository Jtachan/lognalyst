mod input;
mod log_data;

use input::Config;
use std::error::Error;
use std::{env, fs, process};

fn read_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    println!("{}", contents);
    Ok(())
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
