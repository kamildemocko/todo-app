mod arguments;
mod models;
mod db_csv;
mod utils;

use std::path::PathBuf;

use arguments::{CliCommands, parse_arguments};
use models::{DBError, DBPrinter, DBReader, DBRow, DBWriter};


fn main() {
    let cli = parse_arguments();

    let db_path: PathBuf = PathBuf::from("db.csv");
    let repo = db_csv::DBCSV::new(db_path);

    match &cli.command {
        CliCommands::Add { value} => {
            let last_id = repo.get_next_id();
            let timestamp = chrono::Local::now().timestamp();

            let r = DBRow {
                id: last_id,
                updatedate: timestamp,
                completed: false,
                task: value.to_string(),
            };
            repo.add(&r).unwrap();
            repo.print_one_row(&r);
        },

        CliCommands::Delete { id} => {
            match repo.delete(*id) {
                Ok(_) => {
                    println!("\nRow with ID {} deleted.", id);
                }
                Err(DBError::IDNotFound) => println!("\nId was not found."),
                Err(e) => panic!("{}", e),
            }
        },

        CliCommands::Complete { id} => {
            match repo.mark_completion(*id, true) {
                Ok(_) => {
                    let r = repo.read_one(*id).unwrap().unwrap();
                    repo.print_one_row(&r);
                }
                Err(DBError::EmptyDB) => println!("\nNo items stored yet."),
                Err(DBError::IDNotFound) => println!("\nId was not found."),
                Err(e) => panic!("{}", e),
            }
        },

        CliCommands::Incomplete { id} => {
            match repo.mark_completion(*id, false) {
                Ok(_) => {
                    let r = repo.read_one(*id).unwrap().unwrap();
                    repo.print_one_row(&r);
                }
                Err(DBError::EmptyDB) => println!("\nNo items stored yet."),
                Err(DBError::IDNotFound) => println!("\nId was not found."),
                Err(e) => panic!("{}", e),
            }
        },

        CliCommands::List => {
            match repo.read_all() {
                Ok(rows) => {
                    if rows.len() == 0 {
                        println!("\nNo items stored yet.");
                        return;
                    }
                    repo.print_all_rows(rows);
                }
                Err(DBError::EmptyDB) => println!("\nNo items stored yet."),
                Err(e) => panic!("{}", e),
            }
        },
    }
}
