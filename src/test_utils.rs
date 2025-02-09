#![cfg(test)]

use chrono::{Local, TimeZone};

use crate::utils::{unix_to_datetime, get_db_storage_path};

#[test]
fn test_unix_to_datetime_conversion() {
    let timestamp = 1739120530;
    let expected = Local.with_ymd_and_hms(2025, 02, 09, 18, 02, 10).unwrap();
    let result = unix_to_datetime(timestamp);

    assert_eq!(result, expected);
}

#[test]
fn test_db_storage_path_partial() {
    let expected = if cfg!(target_os="windows") { "AppData" } else { "/Users" };
    let result = get_db_storage_path();

    assert!(result.as_os_str().to_str().unwrap().contains(expected));

}
