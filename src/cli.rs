// Command-line parsing using clap
use clap::{Parser, Subcommand, command};

const COMMAND_NAME: &str = "dpl";

#[derive(Parser)]
#[command(name = COMMAND_NAME)]
pub struct Cli {
    pub file: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Repl,
    Explain { file: String },
}
