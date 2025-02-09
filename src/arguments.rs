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
    #[command(alias="a")]
    Add {
        #[arg(trailing_var_arg = true)]
        value: Vec<String>
    },
    /// Deletes task by ID
    #[command(alias="d")]
    Delete { id: u32},
    /// Completes a task by ID
    #[command(alias="c")]
    Complete { id: u32},
    /// Uncompletes a task by ID
    #[command(alias="u")]
    Uncomplete { id: u32},
    /// Prints all tasks
    #[command(alias="l")]
    List,
    #[command(alias="li")]
    ListIncomplete,
    #[command(alias="lc")]
    ListComplete,
}


pub fn parse_arguments() -> Cli {
    return Cli::parse();
}