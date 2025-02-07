use std::fmt::{self};

pub trait DBReader {
    fn read_all(&self) -> Result<Vec<DBRow>, DBError>;
    fn read_one(&self, id: u32) -> Result<Option<DBRow>, DBError>;
}

pub trait DBWriter {
    fn append(&self, r: DBRow) -> Result<(), DBError>;
    fn create(&self, r: DBRow) -> Result<(), DBError>;
    fn delete(&self, id: u32) -> Result<(), DBError>;
    fn update(&self, id: u32, r: DBRow) -> Result<(), DBError>;
}

pub trait DBPrinter {
    fn print_header(&self);
    fn print_row(&self, r: &DBRow);

    fn print_all_rows(&self, v: Vec<DBRow>) {
        self.print_header();
        for r in v {
            self.print_row(&r);
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DBRow {
    pub id: u32,
    pub created: i64,
    pub task: String,
    pub completed: bool,
}

#[derive(Debug)]
pub enum DBError {
    ReadError(String),
    WriteError(String),
}

impl std::error::Error for DBError {}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBError::ReadError(msg) => write!(f, "read error: {}", msg),
            DBError::WriteError(msg) => write!(f, "write error: {}", msg),
        }
    }
}

impl DBError {
    pub fn new_read_error(msg: &str) -> DBError {
        return DBError::ReadError(msg.to_string())
    }

    pub fn new_write_error(msg: &str) -> DBError {
        return DBError::WriteError(msg.to_string())
    }
}