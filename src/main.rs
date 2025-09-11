mod ast;
mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use std::fs;

fn main() -> anyhow::Result<()> {
    // parse CLI args
    let cli = Cli::parse();
    let cmd_name = cli::COMMAND_NAME;
    match cli.command {
        // interactive mode
        Some(Commands::Repl) => {
            println!(
                "{} interactive mode (not yet implemented)...",
                cmd_name.to_uppercase()
            )
            // TODO: implement interactive loop
        }

        // explain command word
        Some(Commands::Explain { file: _ }) => {
            // let source = fs::read_to_string(&file);
            println!(
                "{} Explain command (not yet implemented)...",
                cmd_name.to_uppercase()
            );
            // let ast = parser::parse(&source);
            // println!("AST for {}:\n{:#?}", file, ast);
        }

        None => {
            if let Some(file) = cli.file {
                // let source = fs::read_to_string(&file);
                println!(
                    "{} run command (not yet implemented)...",
                    cmd_name.to_uppercase()
                );
                // let ast = parser::parse(&source);
                // interpreter::interpret(ast)?;
            } else {
                eprintln!("Usage: dpl <file.dpl> | dpl repl | dpl explain <file.dpl>");
            }
        }
    }
    Ok(())
}
