use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::env::Args;
use std::error::Error;

struct Config {
    pattern: String,
    filename: String,
}

impl Config {
    fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next(); // This is to skip the command name

        let pattern = match args.next() {
            Some(arg) => arg,
            None => return  Err("Missing search pattern"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing File name"),
        };

        Ok(Config { pattern, filename })
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
    
    let res = search(&config.pattern, reader)?;

    for line in res {
        println!("{line}");
    }

    Ok(())
}

fn search(pattern: &str, bfr: impl BufRead) -> io::Result<Vec<String>> {
    let mut matches: Vec<String> = Vec::new();

    for line_result in bfr.lines() {
        let line = line_result?;
        if line.contains(pattern) {
            matches.push(line);
        }
    }

    Ok(matches)
}