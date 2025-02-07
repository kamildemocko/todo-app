use std::fs::File;
use std::path::{Path, PathBuf};

use crate::models::{DBError, DBPrinter, DBReader, DBRow, DBWriter};
use crate::utils::unix_to_datetime;


pub struct DBCSV {
    path: PathBuf,
    reader: csv::Reader<File>,
}


impl DBReader for DBCSV {
    fn read_all(&mut self) -> Result<Vec<DBRow>, DBError> {
        if !Path::exists(&self.path) {
            return Err(DBError::new_read_error("db file path does not exist"));
        }

        // let mut reader = self.get_reader()?;
        let mut reader = self.get_reader()?;
    
        let data = reader.deserialize()
            .collect::<Result<Vec<DBRow>, csv::Error>>()
            .map_err(|e| DBError::new_read_error(&e.to_string()))?;
        
        Ok(data)
    }

    fn read_one(&mut self, id: u32) -> Result<DBRow, DBError> {
        todo!()
    }
}

impl DBWriter for DBCSV {
    fn append(&mut self, r: DBRow) -> Result<(), crate::models::DBError> {
        todo!()
    }

    fn create(&mut self, r: DBRow) -> Result<(), crate::models::DBError> {
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
}

impl DBCSV {
    pub fn new(path: PathBuf) -> Self {
        return DBCSV{
            reader: DBCSV::get_reader2(&path).unwrap(),
            path: path,
            
        };
    }

    fn get_reader(&self) -> Result<csv::Reader<File>, DBError> {
        if !Path::exists(&self.path) {
            return Err(DBError::new_read_error("db file path does not exist"));
        }

        let reader = csv::ReaderBuilder::new()
            .delimiter(';' as u8)
            .has_headers(true)
            .from_path(&self.path)
            .map_err(|e| DBError::new_read_error(&e.to_string()))?;

        return Ok(reader)
    }

    fn get_reader2(path: &PathBuf) -> Result<csv::Reader<File>, DBError> {
        if !Path::exists(path) {
            return Err(DBError::new_read_error("db file path does not exist"));
        }

        let reader = csv::ReaderBuilder::new()
            .delimiter(';' as u8)
            .has_headers(true)
            .from_path(&path)
            .map_err(|e| DBError::new_read_error(&e.to_string()))?;

        return Ok(reader)
    }
}