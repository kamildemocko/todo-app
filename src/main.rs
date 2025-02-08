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
            let last_id = match repo.read_last_row().unwrap() {
                Some(v) => v.id,
                None => 0,
            };
            let timestamp = chrono::Local::now().timestamp();

            let r = DBRow {
                id: last_id + 1,
                created: timestamp,
                completed: false,
                task: value.to_string(),
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
