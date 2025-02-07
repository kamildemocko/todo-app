use std::path::{Path, PathBuf};

use crate::models::{DBError, DBPrinter, DBReader, DBRow, DBWriter};
use crate::utils::unix_to_datetime;


pub struct DBCSV {
    path: PathBuf
}


impl DBReader for DBCSV {
    fn read_all(&self) -> Result<Vec<DBRow>, DBError> {
        if !Path::exists(&self.path) {
            return Err(DBError::new_read_error("db file path does not exist"));
        }

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(';' as u8)
            .has_headers(true)
            .from_path(&self.path)
            .map_err(|e| DBError::new_read_error(&e.to_string()))?;
    
        let data = reader.deserialize()
            .collect::<Result<Vec<DBRow>, csv::Error>>()
            .map_err(|e| DBError::new_read_error(&e.to_string()))?;
        
        Ok(data)
    }

    fn read_one(&self, id: u32) -> Result<DBRow, DBError> {
        todo!()
    }
}

impl DBWriter for DBCSV {
    fn append(&self, r: DBRow) -> Result<(), crate::models::DBError> {
        todo!()
    }

    fn create(&self, r: DBRow) -> Result<(), crate::models::DBError> {
        todo!()
    }
}

impl DBPrinter for DBCSV {
    fn print_header(&self) {
        println!("{:>4}\t{:^5}\t{:20}\t{}", "ID", "State", "Created", "Task");
        println!("{:>4}\t{:^5}\t{:20}\t{}", "--", "-----", "-------", "----");
    }

    fn print_row(&self, r: &DBRow) {
        let done = if r.completed { "[X]" }  else { "[ ]" };
        let dt = unix_to_datetime(r.created as u64);
        println!("{:>4}\t{:^5}\t{:20}\t{}", r.id, done, dt.format("%Y-%m-%d %H:%M:%S"), r.task);
    }

    fn print_all_rows(&self, v: Vec<DBRow>) {
        self.print_header();
        for r in v {
            self.print_row(&r);
        }
    }
}

impl DBCSV {
    pub fn new(path: PathBuf) -> Self {
        return DBCSV{
            path: path,
        };
    }
}