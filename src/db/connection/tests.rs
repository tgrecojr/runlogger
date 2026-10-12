//! Tests for locating and opening the run database, in a scratch directory.

use super::*;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A fresh, empty directory under the system temp dir, removed on drop.
struct Scratch(PathBuf);

impl Scratch {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("runlogger-test-{}-{n}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        Scratch(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

// @spec STORE-001
#[test]
fn database_lives_in_runlogger_dir_which_is_created_if_missing() {
    let scratch = Scratch::new();
    let path = db_path_in(Some(scratch.path().to_path_buf())).unwrap();

    assert_eq!(path, scratch.path().join("runlogger").join("runs.db"));
    assert!(scratch.path().join("runlogger").is_dir());
}

// @spec STORE-001, STORE-003
#[test]
fn opening_creates_the_database_file_with_the_schema() {
    let scratch = Scratch::new();
    let path = db_path_in(Some(scratch.path().to_path_buf())).unwrap();
    assert!(!path.exists());

    let conn = init_db(&path).unwrap();
    assert!(path.is_file());
    let tables: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'runs'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(tables, 1);
}

// @spec STORE-002
#[test]
fn missing_data_directory_is_an_error() {
    let err = db_path_in(None).unwrap_err();
    assert_eq!(err.to_string(), "Failed to get data directory");
}

// @spec STORE-002
#[test]
fn unusable_runlogger_directory_fails_startup() {
    let scratch = Scratch::new();
    fs::write(scratch.path().join("runlogger"), "not a directory").unwrap();

    // A file in the way is noticed when the database is opened inside it.
    let startup = db_path_in(Some(scratch.path().to_path_buf())).and_then(|p| init_db(&p));
    let err = startup.expect_err("startup should fail");
    assert_eq!(err.to_string(), "Failed to open database connection");
}

// @spec STORE-002
#[test]
fn unopenable_database_is_an_error() {
    let scratch = Scratch::new();
    let path = db_path_in(Some(scratch.path().to_path_buf())).unwrap();
    fs::create_dir_all(&path).unwrap(); // a directory where the database file should be

    let err = init_db(&path).unwrap_err();
    assert!(
        [
            "Failed to open database connection",
            "Failed to initialize database"
        ]
        .contains(&err.to_string().as_str()),
        "{err:#}"
    );
}
