mod arguments;
mod models;
mod db_csv;
mod utils;

use std::path::PathBuf;

use arguments::{CliCommands, parse_arguments};
use models::{DBPrinter, DBReader, DBRow, DBWriter};


fn main() {
    let cli = parse_arguments();

    let db_path: PathBuf = PathBuf::from("db.csv");
    let mut repo = db_csv::DBCSV::new(db_path);

    match &cli.command {
        CliCommands::Add { value} => {
            println!("value add is {}", value);
            let r = DBRow {
                id: 3,
                created: 1657113606,
                completed: false,
                task: "adding one from main.rs".to_string(),
            };
            repo.append(&r).unwrap();
            repo.print_one_row(&r);
        },
        CliCommands::Delete { id} => {
            println!("value del is {}", id);
        },
        CliCommands::Complete { id} => {
            println!("value del is {}", id);
        },
        CliCommands::List => {
            println!("list was called");
            let all_items = repo.read_all()
                .unwrap_or_else(|e| panic!("cannot read the database: {}", e));
            repo.print_all_rows(all_items);
        },
    }
}
