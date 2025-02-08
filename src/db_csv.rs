use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};


use crate::models::{DBError, DBPrinter, DBReader, DBRow, DBWriter};
use crate::utils::unix_to_datetime;


pub struct DBCSV {
    path: PathBuf,
}


impl DBReader for DBCSV {
    fn read_all(&self) -> Result<Vec<DBRow>, DBError> {
        let mut reader = self.get_reader()?;
    
        let mut data = reader.deserialize()
            .collect::<Result<Vec<DBRow>, csv::Error>>()
            .map_err(|e| DBError::new_read_error(&e.to_string()))?;

        data.sort();

        Ok(data)
    }

    fn read_one(&self, id: u32) -> Result<Option<DBRow>, DBError> {
        let mut reader = self.get_reader()?;

        for row in reader.deserialize() {
            let record: DBRow = row
                .map_err(|e| DBError::new_read_error(&e.to_string()))?;
            
            if record.id == id {
                return Ok(Some(record))
            }
        }

        Ok(None)
    }

    fn read_last_row(&self) -> Result<Option<DBRow>, DBError> {
        let mut reader = match self.get_reader() {
            Ok(r) => r,
            Err(DBError::EmptyDB) => return Ok(None),
            Err(e) => return Err(e),
        };

        let record = reader.deserialize()
            .last()
            .transpose()
            .map_err(|e| DBError::new_read_error(&e.to_string()))?;
        
        Ok(record)
    }
}

impl DBWriter for DBCSV {
    fn add(&self, r: &DBRow) -> Result<(), crate::models::DBError> {
        let mut writer = match self.get_writer(true) {
            Ok(r) => r,
            Err(DBError::EmptyDB) => {
                self.create_db()?;
                self.get_writer(true)?
            },
            Err(e) => panic!("{}", e),
        };

        writer.serialize(r)
            .map_err(|e| DBError::new_write_error(&e.to_string()))?;

        writer.flush()
            .map_err(|e| DBError::new_write_error(&e.to_string()))?;

        Ok(())
    }

    fn create_db(&self) -> Result<(), DBError> {
        let mut file = File::create(&self.path)
            .map_err(|e| DBError::new_write_error(&e.to_string()))?;
        
        file.write("id;updatedate;task;completed\n".as_bytes())
            .map_err(|e| DBError::new_write_error(&e.to_string()))?;

        file.flush()
            .map_err(|e| DBError::new_write_error(&e.to_string()))?;
        
        Ok(())
    }
    
    fn delete(&self, id: u32) -> Result<(), DBError> {
        // get reader and writer to the temp file
        let mut reader = self.get_reader()?;
        let (mut temp_writer, temp_path) = self.get_temp_writer()?;

        // work
        let mut removed  = 0;
        for result in reader.deserialize() {
            let record: DBRow = result
                .map_err(|e| DBError::new_write_error(&e.to_string()))?;

            if record.id != id {
                temp_writer.serialize(record)
                    .map_err(|e| DBError::new_write_error(&e.to_string()))?;
            } else {
                removed += 1;
            }
        }

        temp_writer.flush()
            .map_err(|e| DBError::new_write_error(&e.to_string()))?;
        
        // rename temp
        fs::remove_file(&self.path)
            .map_err(|e| DBError::new_write_error(&e.to_string()))?;
        fs::rename(temp_path, &self.path)
            .map_err(|e| DBError::new_write_error(&e.to_string()))?;

        if removed == 0 {
            return Err(DBError::new_idnotfound_error())
        }

        Ok(())
    }
    
    fn mark_completion(&self, id: u32, complete: bool) -> Result<(), DBError> {
        match self.read_one(id)? {
            Some(mut v) => {
                v.completed = true;

                let mut writer = self.get_writer(false)?;

                todo!()
            }
            None => {
                println!("Row with ID {} was not found", id);
                return Ok(())
            }
        };
    }
}

impl DBPrinter for DBCSV {
    fn print_header(&self) {
        println!();
        println!("{:>4}\t{:^5}\t{:20}\t{}", "ID", "State", "Date updated", "Task");
        println!("{:>4}\t{:^5}\t{:20}\t{}", "--", "-----", "------------", "----");
    }

    fn print_row(&self, r: &DBRow) {
        let done = if r.completed { "[X]" }  else { "[ ]" };
        let dt = unix_to_datetime(r.updatedate);
        println!("{:>4}\t{:^5}\t{:20}\t{}", r.id, done, dt.format("%Y-%m-%d %H:%M:%S"), r.task);
    }
}

impl DBCSV {
    pub fn new(path: PathBuf) -> Self {
        return DBCSV{
            path: path,
            
        };
    }

    fn get_reader(&self) -> Result<csv::Reader<File>, DBError> {
        if !Path::exists(&self.path) {
            return Err(DBError::new_emptydb_error());
        }

        let reader = csv::ReaderBuilder::new()
            .delimiter(';' as u8)
            .has_headers(true)
            .from_path(&self.path)
            .map_err(|e| DBError::new_read_error(&e.to_string()))?;
        
        return Ok(reader)
    }

    fn get_writer(&self, append: bool) -> Result<csv::Writer<File>, DBError> {
        let file = fs::OpenOptions::new()
            .write(true)
            .append(append)
            .open(&self.path)
            .map_err(|_| DBError::new_emptydb_error())?;

        let writer = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(';' as u8)
            .from_writer(file);

        return Ok(writer)
    }

    fn get_temp_writer(&self) -> Result<(csv::Writer<File>, PathBuf), DBError> {
        let temp_path = self.path.with_extension("tmp");

        let writer = csv::WriterBuilder::new()
            .has_headers(true)
            .delimiter(';' as u8)
            .from_path(&temp_path)
            .map_err(|_| DBError::new_emptydb_error())?;

        Ok((writer, temp_path))
    }

}