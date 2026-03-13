//! Module related to the input that the user can provide.

/// Structure containing all the parameters to be parsed via CLI.
#[derive(Debug)]
pub struct Config {
    pub file_path: String,
}

impl Config {
    /// Function to build the configuration out of the CLI arguments.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //Remove the first argument, as it corresponds to the path of the executable.
        args.next();
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing parameter: file_path."),
        };
        Ok(Config { file_path })
    }
}
