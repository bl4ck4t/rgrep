// Core search logic.
// This module contains no CLI or filesystem concerns beyond buffered input, which makes it easy to test and reuse.

use std::io::{self, BufRead};

// Performs a case-sensitive search over a buffered input.
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

// Performs a case-insensitive search.
// Pattern is normalized once to avoid repeated allocations per line.
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
