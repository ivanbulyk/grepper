use std::path::Path;

// Used for writing assertions
use assert_cmd::Command;
use assert_cmd::prelude::*;
// Add methods on commands
use predicates::prelude::*;

use grepper::search_file;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("grepper")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn test_find_a_match_and_count() {
    let path = Path::new("tests/inputs/night.txt");
    let res = search_file(path, "The").expect("oh");

    assert_eq!(res, 10);
}