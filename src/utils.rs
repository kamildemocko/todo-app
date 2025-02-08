use chrono::{Local, TimeZone, Utc};

pub fn unix_to_datetime(timestamp: i64) -> chrono::DateTime<Local> {
    Utc.timestamp_opt(timestamp, 0).unwrap()
    .with_timezone(&Local)
}
