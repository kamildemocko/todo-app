use std::path::PathBuf;

use todo_app::arguments::{CliCommands, parse_arguments};
use todo_app::models::{DBError, DBPrinter, DBReader, DBRow, DBWriter};
use todo_app::{db_csv, utils};


fn main() {
    let cli = parse_arguments();

    let db_path: PathBuf = utils::get_db_storage_path();
    let repo = db_csv::DBCSV::new(db_path);

    match &cli.command {
        CliCommands::Add { value} => {
            let last_id = repo.get_next_id();
            let timestamp = chrono::Local::now().timestamp();

            let r = DBRow {
                id: last_id,
                updatedate: timestamp,
                completed: false,
                task: value.join(" ").to_string(),
            };
            match repo.add(&r) {
                Ok(()) => (),
                Err(e) => panic!("{}", e),
            }
            repo.print_one_row(&r);
        },

        CliCommands::Delete { id} => {
            match repo.delete(*id) {
                Ok(_) => {
                    println!("\nRow with ID {} deleted.\n", id);
                }
                Err(DBError::IDNotFound) => println!("\nID was not found.\n"),
                Err(e) => panic!("{}", e),
            }
        },

        CliCommands::Complete { id} => {
            match repo.mark_completion(*id, true) {
                Ok(_) => {
                    let r = repo.read_one(*id).unwrap().unwrap();
                    repo.print_one_row(&r);
                }
                Err(DBError::EmptyDB) => println!("\nNo items stored yet.\n"),
                Err(DBError::IDNotFound) => println!("\nID was not found.\n"),
                Err(e) => panic!("{}", e),
            }
        },

        CliCommands::Uncomplete { id} => {
            match repo.mark_completion(*id, false) {
                Ok(_) => {
                    let r = repo.read_one(*id).unwrap().unwrap();
                    repo.print_one_row(&r);
                }
                Err(DBError::EmptyDB) => println!("\nNo items stored yet.\n"),
                Err(DBError::IDNotFound) => println!("\nID was not found.\n"),
                Err(e) => panic!("{}", e),
            }
        },

        CliCommands::List => {
            match repo.read_all() {
                Ok(rows) => {
                    if rows.len() == 0 {
                        println!("\nNo items stored yet.\n");
                        return;
                    }
                    repo.print_all_rows(rows);
                }
                Err(DBError::EmptyDB) => println!("\nNo items stored yet.\n"),
                Err(e) => panic!("{}", e),
            }
        },

        CliCommands::ListIncomplete => {
            match repo.read_all() {
                Ok(rows) => {
                    if rows.len() == 0 {
                        println!("\nNo items stored yet.\n");
                        return;
                    }
                    repo.print_complete_filter(rows, false);
                }
                Err(DBError::EmptyDB) => println!("\nNo items stored yet.\n"),
                Err(e) => panic!("{}", e),
            }
        },

        CliCommands::ListComplete => {
            match repo.read_all() {
                Ok(rows) => {
                    if rows.len() == 0 {
                        println!("\nNo items stored yet.\n");
                        return;
                    }
                    repo.print_complete_filter(rows, true);
                }
                Err(DBError::EmptyDB) => println!("\nNo items stored yet.\n"),
                Err(e) => panic!("{}", e),
            }
        },
    }
}
