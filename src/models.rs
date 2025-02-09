use std::{cmp::Ordering, fmt::{self}};

pub trait DBReader {
    fn read_all(&self) -> Result<Vec<DBRow>, DBError>;
    fn read_one(&self, id: u32) -> Result<Option<DBRow>, DBError>;
    fn get_next_id(&self) -> u32;
}

pub trait DBWriter {
    fn add(&self, r: &DBRow) -> Result<(), DBError>;
    fn create_db(&self) -> Result<(), DBError>;
    fn delete(&self, id: u32) -> Result<(), DBError>;
    fn mark_completion(&self, id: u32, complete: bool) -> Result<(), DBError>;
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

    fn print_one_row(&self, r: &DBRow) {
        self.print_header();
        self.print_row(&r);
    }

    fn print_complete_filter(&self, v: Vec<DBRow>, completed: bool) {
        self.print_header();
        for r in v {
            if r.completed != completed { continue }

            self.print_row(&r);
        }
    }
}

#[derive(Debug, Eq, serde::Deserialize, serde::Serialize)]
pub struct DBRow {
    pub id: u32,
    pub updatedate: i64,
    pub task: String,
    pub completed: bool,
}

impl PartialEq for DBRow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for DBRow {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for DBRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub enum DBError {
    ReadError(String),
    WriteError(String),
    EmptyDB,
    IDNotFound,
}

impl std::error::Error for DBError {}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBError::ReadError(msg) => write!(f, "read error: {}", msg),
            DBError::WriteError(msg) => write!(f, "write error: {}", msg),
            DBError::EmptyDB => write!(f, "database empty"),
            DBError::IDNotFound => write!(f, "id was not found"),
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

    pub fn new_dbnotexist_error() -> DBError {
        return DBError::EmptyDB
    }

    pub fn new_idnotfound_error() -> DBError {
        return DBError::IDNotFound
    }
}