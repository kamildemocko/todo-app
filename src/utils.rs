use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use std::time::{Duration, UNIX_EPOCH};

pub fn unix_to_datetime(timestamp: i64) -> chrono::DateTime<Utc> {
    Utc.timestamp_opt(timestamp, 0).unwrap()
}
