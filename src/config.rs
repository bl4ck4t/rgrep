// CLI configuration and argument parsing.
// Owns all user-provided input to avoid lifetime issues.
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    /// Pattern to search for
    pub pattern: String,

    /// File to search
    pub filename: String,
    
    /// Enable case-insensitive search
    #[arg(short, long)]
    pub ignore_case: bool,
}