use chrono::{DateTime, Utc};
use std::time::{Duration, UNIX_EPOCH};

pub fn unix_to_datetime(timestamp: u64) -> chrono::DateTime<Utc> {
    let systime = UNIX_EPOCH + Duration::from_secs(timestamp);
    DateTime::<Utc>::from(systime)
}