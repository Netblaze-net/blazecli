use::blazecli::{Cli, Commands, render};
use clap::Parser;



fn main() {
    let cli: Cli = blazecli::Cli::parse();

    

    match cli.command {
        Some(Commands::Generate(args)) => {
            render(args).unwrap();
        }
        None => {
            println!("No command provided. Use `--help` for available commands and options.");
        }
    }
}

