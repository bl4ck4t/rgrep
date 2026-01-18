use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use thiserror::Error;


mod config;
use config::Config;

mod search;
use search::*;


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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn finds_matching_lines() {
        let input = "\
hello world
this is rust
hello systems programming";
        
        let reader = Cursor::new(input);
        let res = search("hello", reader).unwrap();

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], "hello world");
        assert_eq!(res[1], "hello systems programming");
    }

    #[test]
    fn finds_case_insensitive_matches() {
        let input = "\
Hello World
this is Rust
HELLO systems";

        let reader = Cursor::new(input);
        let result = search_case_insensitive("hello", reader).unwrap();

        assert_eq!(result.len(), 2);
}

}