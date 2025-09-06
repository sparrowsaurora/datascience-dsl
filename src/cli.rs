// Command-line parsing using clap

const command_name: &str = "dpl";

#[derive(Parser)]
#[command(name = command_name)]
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
