// Binary entry point.
// Responsible only for CLI parsing and high-level error handling.
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use thiserror::Error;

use rgrep::config::Config;
use rgrep::search::*;


#[derive(Error, Debug)]
enum AppError {
    #[error("Failed to read file")]
    Io(#[from] std::io::Error),
}

fn main() {
    // Parse command-line arguments into a configuration struct.
    let config = Config::parse();

    // Delegate execution to `run` and handle any top-level errors.
    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        std::process::exit(1);
    }
}

// Orchestrates file I/O and search execution.
// Keeps side effects (I/O) separate from pure search logic.
fn run(config: Config) -> Result<(), AppError> {

    let file = File::open(&config.filename)?;
    let reader = BufReader::new(file);
    
    let res= 
    if config.ignore_case {
        search_case_insensitive(&config.pattern, reader)?
    } else {
        search(&config.pattern, reader)?
    };

    for line in res {
        println!("{line}");
    }

    Ok(())
}
