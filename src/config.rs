use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    pub pattern: String,

    pub filename: String,

    #[arg(short, long)]
    pub ignore_case: bool,
}