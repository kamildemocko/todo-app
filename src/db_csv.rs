use std::fs::{self, File};
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

    fn get_next_id(&self) -> u32 {
        let mut reader = match self.get_reader() {
            Ok(r) => r,
            Err(_) => return 1,
        };

        // gather all existing IDs
        let mut existing: Vec<u32> = vec![];
        for result in reader.deserialize() {
            let record: DBRow = match result {
                Ok(v) => v,
                Err(_) => continue,
            };
            existing.push(record.id);
        }

        for i in 1..1000 {
            if !existing.contains(&i) {
                return i;
            }
        }

        1
    }
}

impl DBWriter for DBCSV {
    fn add(&self, r: &DBRow) -> Result<(), crate::models::DBError> {
        if self.db_is_empty() { 
            self.create_db()?;
        }

        let mut writer = match self.get_writer(true) {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };

        writer.serialize(r)
            .map_err(|e| DBError::new_write_error(&e.to_string()))?;

        writer.flush()
            .map_err(|e| DBError::new_write_error(&e.to_string()))?;

        Ok(())
    }

    fn create_db(&self) -> Result<(), DBError> {
        fs::create_dir_all(self.path.parent().unwrap())
            .map_err(|_| DBError::new_write_error("cannot create folder structure for DB"))?;

        File::create(&self.path)
            .map_err(|_| DBError::new_write_error("cannot create folder structure for DB"))?;

        let mut writer = self.get_writer(false)?;

        let header = vec!["id", "updatedate", "task", "completed"];
        writer.write_record(&header)
            .map_err(|e| DBError::new_write_error(&e.to_string()))?;

        writer.flush()
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
                v.completed = complete;

                self.delete(id)?;

                self.add(&v)?;
            }
            None => {
                return Err(DBError::new_idnotfound_error());
            }
        };

        Ok(())
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
            return Err(DBError::new_dbnotexist_error());
        }

        let reader = csv::ReaderBuilder::new()
            .delimiter(';' as u8)
            .has_headers(true)
            .from_path(&self.path)
            .map_err(|e| DBError::new_read_error(&e.to_string()))?;
        
        return Ok(reader)
    }

    fn db_is_empty(&self) -> bool {
        let file = match fs::OpenOptions::new()
            .read(true)
            .open(&self.path) {
                Ok(f) => f,
                Err(_) => { return true },
            };

        let size = file.metadata()
            .unwrap()
            .len();

        if size == 0 {
            return true;
        }

        false
    }

    fn get_writer(&self, append: bool) -> Result<csv::Writer<File>, DBError> {
        let file = fs::OpenOptions::new()
            .write(true)
            .append(append)
            .open(&self.path)
            .map_err(|_| DBError::new_dbnotexist_error())?;

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
            .map_err(|_| DBError::new_dbnotexist_error())?;

        Ok((writer, temp_path))
    }

}