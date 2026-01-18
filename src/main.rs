use std::{io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Config {
    pattern: String,

    filename: String,

    #[arg(short, long)]
    ignore_case: bool,
}

fn main() {
    let config = Config::parse();

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{

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

fn search(pattern: &str, bfr: impl BufRead) -> io::Result<Vec<String>> {
    let mut matches: Vec<String> = Vec::new();

    for line_result in bfr.lines() {
        let line = line_result?;
        if line.contains(&pattern) {
            matches.push(line);
        }
    }

    Ok(matches)
}

fn search_case_insensitive(pattern: &str, bfr: impl BufRead) -> io::Result<Vec<String>> {
    let pattern= pattern.to_lowercase();
    let mut matches: Vec<String> = Vec::new();

    for line_res in bfr.lines() {
        let line = line_res?;
        if line.to_lowercase().contains(&pattern) {
            matches.push(line);
        }
    }
    Ok(matches)
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