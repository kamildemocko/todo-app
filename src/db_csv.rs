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