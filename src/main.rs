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
    let config = Config::parse();

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        std::process::exit(1);
    }
}

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
