use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    Add { value: String },
    Delete { id: u32},
    Complete { id: u32},
    List,
}


pub fn parse_arguments() -> Cli {
    return Cli::parse();
}