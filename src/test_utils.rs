#![cfg(test)]

use chrono::{TimeZone, Utc};

use crate::utils::{unix_to_datetime, get_db_storage_path};

#[test]
fn test_unix_to_datetime_conversion() {
    let timestamp = 1739120530;
    let result = unix_to_datetime(timestamp);

    let utc_result = result.with_timezone(&Utc);
    let utc_expected = Utc.timestamp_opt(timestamp, 0).unwrap();

    assert_eq!(utc_expected, utc_result);
}

#[test]
fn test_db_storage_path_partial() {
    let expected = if cfg!(target_os="windows") { 
        "AppData"
    } else if cfg!(target_os="macos") {
        "/Users"
    } else {
        "/home"
    };
    let result = get_db_storage_path();

    assert!(result.to_string_lossy().contains(expected));

}
