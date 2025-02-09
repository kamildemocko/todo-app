use std::{env, path::PathBuf};

use chrono::{Local, TimeZone, Utc};

pub fn unix_to_datetime(timestamp: i64) -> chrono::DateTime<Local> {
    Utc.timestamp_opt(timestamp, 0).unwrap()
    .with_timezone(&Local)
}

pub fn get_db_storage_path() -> PathBuf {
    if cfg!(target_os="windows") {
        PathBuf::from(env::var("APPDATA").unwrap()).join("todo-app").join("db.csv")
    } else {
        PathBuf::from(env::var("HOME").unwrap()).join(".local/share").join("todo-app").join("db.csv")
    }
}
