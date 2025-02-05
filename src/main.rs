mod arguments;
mod models;
mod db_csv;

use arguments::{CliCommands, parse_arguments};

fn main() {
    let cli = parse_arguments();

    match &cli.command {
        CliCommands::Add { value} => {
            println!("value add is {}", value);
        },
        CliCommands::Delete { id} => {
            println!("value del is {}", id);
        },
        CliCommands::Complete { id} => {
            println!("value del is {}", id);
        },
        CliCommands::List => {
            println!("list was called");
        },
    }
}
