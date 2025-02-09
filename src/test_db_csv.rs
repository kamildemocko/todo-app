#![cfg(test)]

use std::{fs, path::PathBuf};

use tempfile::TempDir;

use crate::{db_csv::DBCSV, models::{DBReader, DBRow, DBWriter}};

struct TestDB {
    _temp_dir: TempDir,
    _path: PathBuf,
    db: DBCSV,
}

impl TestDB {
    fn new() -> Self {
        let tempdir = TempDir::new().unwrap();
        let db_path = tempdir.path().join("test.csv");

        return TestDB { _temp_dir: tempdir, _path: db_path.to_owned(), db: DBCSV::new(db_path) }
    }
}

#[test]
fn test_db_read_all_empty_is_err() {
    let test_db = TestDB::new();
    let result = test_db.db.read_all();
    assert!(result.is_err())
}

#[test]
fn test_db_create_ok() {
    let test_db = TestDB::new();
    let result = test_db.db.create_db();

    assert!(result.is_ok());
    assert!(fs::exists(test_db._path).is_ok());
}

#[test]
fn test_db_add_is_ok() {
    let test_db = TestDB::new();

    let r1 = DBRow{
        id: 1,
        updatedate: 1739126402,
        task: "test1".to_string(),
        completed: false,
    };

    let result = test_db.db.add(&r1);
    assert!(result.is_ok())
}

#[test]
fn test_db_read_one_is_not_found() {
    let test_db = TestDB::new();

    let r1 = DBRow{
        id: 1,
        updatedate: 1739126402,
        task: "test1".to_string(),
        completed: false,
    };
    test_db.db.add(&r1).unwrap();

    let result = test_db.db.read_one(2);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}


#[test]
fn test_db_read_one_ok() {
    let test_db = TestDB::new();

    let r1 = DBRow{
        id: 1,
        updatedate: 1739126402,
        task: "test1".to_string(),
        completed: false,
    };

    test_db.db.add(&r1).unwrap();

    let result = test_db.db.read_one(1);
    assert!(result.is_ok());
    assert!(result.unwrap().unwrap().task == "test1");
}

#[test]
fn test_db_read_all_ok() {
    let test_db = TestDB::new();

    let r1 = DBRow{
        id: 1,
        updatedate: 1739126402,
        task: "test1".to_string(),
        completed: false,
    };
    let r2 = DBRow{
        id: 1,
        updatedate: 1739126602,
        task: "test2".to_string(),
        completed: true,
    };

    test_db.db.add(&r1).unwrap();
    test_db.db.add(&r2).unwrap();

    let result = test_db.db.read_all();
    assert!(result.is_ok());

    let unw = result.unwrap();
    assert!(unw.len() == 2);
    assert!(unw[1].task == "test2".to_string());
}

#[test]
fn test_db_delete_is_ok() {
    let test_db = TestDB::new();

    let r1 = DBRow{
        id: 1,
        updatedate: 1739126402,
        task: "test1".to_string(),
        completed: false,
    };
    test_db.db.add(&r1).unwrap();

    assert!(test_db.db.delete(1).is_ok());
    assert!(test_db.db.read_one(1).unwrap().is_none());
}

#[test]
fn test_db_mark_completion_true_is_ok() {
    let test_db = TestDB::new();

    let r1 = DBRow{
        id: 1,
        updatedate: 1739126402,
        task: "test1".to_string(),
        completed: false,
    };
    test_db.db.add(&r1).unwrap();

    assert!(test_db.db.mark_completion(1, true).is_ok());
    assert!(test_db.db.read_one(1).unwrap().unwrap().completed);
}

#[test]
fn test_db_mark_completion_false_is_ok() {
    let test_db = TestDB::new();

    let r1 = DBRow{
        id: 1,
        updatedate: 1739126402,
        task: "test1".to_string(),
        completed: true,
    };
    test_db.db.add(&r1).unwrap();

    assert!(test_db.db.mark_completion(1, false).is_ok());
    assert!(!test_db.db.read_one(1).unwrap().unwrap().completed);
}

#[test]
fn test_db_get_next_id_is_ok() {
    let test_db = TestDB::new();

    assert!(test_db.db.get_next_id() == 1);

    let r1 = DBRow{
        id: 1,
        updatedate: 1739126402,
        task: "test1".to_string(),
        completed: true,
    };
    test_db.db.add(&r1).unwrap();

    assert!(test_db.db.get_next_id() == 2);

    let r3 = DBRow{
        id: 3,
        updatedate: 1739126402,
        task: "test3".to_string(),
        completed: true,
    };
    test_db.db.add(&r3).unwrap();

    assert!(test_db.db.get_next_id() == 2);
}