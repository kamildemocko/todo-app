use std::fmt::{self};

pub trait DBReader {
    fn read_all() -> Vec<DBRow>;
    fn read_one(id: u32) -> DBRow;
}

pub trait DBWriter {
    fn append(r: DBRow) -> Result<(), DBError>;
    fn create(r: DBRow) -> Result<(), DBError>;
}

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
            DBError::ReadError(msg) => write!(f, "Read error: {}", msg),
            DBError::WriteError(msg) => write!(f, "Write error: {}", msg),
        }
    }
}