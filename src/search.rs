use std::io;
use std::io::BufRead;

pub fn search(pattern: &str, bfr: impl BufRead) -> io::Result<Vec<String>> {
    let mut matches: Vec<String> = Vec::new();

    for line_result in bfr.lines() {
        let line = line_result?;
        if line.contains(&pattern) {
            matches.push(line);
        }
    }

    Ok(matches)
}

pub fn search_case_insensitive(pattern: &str, bfr: impl BufRead) -> io::Result<Vec<String>> {
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
