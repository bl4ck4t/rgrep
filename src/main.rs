use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::env::Args;
use std::error::Error;

struct Config {
    pattern: String,
    filename: String,
    ignore_case: bool
}

impl Config {
    fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next(); // This is to skip the command name

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        let pattern = match args.next() {
            Some(arg) => arg,
            None => return  Err("Missing search pattern"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing File name"),
        };

        Ok(Config { pattern, filename, ignore_case})
    }
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
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