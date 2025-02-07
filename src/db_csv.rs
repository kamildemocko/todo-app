use std::path::{Path, PathBuf};

use crate::models::{DBError, DBReader, DBRow, DBWriter};


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

        todo!()
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

impl DBCSV {
    pub fn new(path: PathBuf) -> Self {
        return DBCSV{
            path: path,
        };
    }
}