use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about="A simple TODO list manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    /// Adds new task
    Add { value: String },
    /// Deletes task by ID
    Delete { id: u32},
    /// Completes a task by ID
    Complete { id: u32},
    /// Uncompletes a task by ID
    Uncomplete { id: u32},
    /// Prints all tasks
    List,
}


pub fn parse_arguments() -> Cli {
    return Cli::parse();
}