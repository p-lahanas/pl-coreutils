use assert_cmd::Command;
use predicates::prelude::*;

const TEST_BIN: &str = "pl-echo";

#[test]
fn test_no_parameters() {
    let mut cmd = Command::cargo_bin(TEST_BIN).unwrap();

    cmd.assert().success();
    cmd.assert().stdout(predicate::eq("\n"));
}

#[test]
fn invalid_args() {
    let mut cmd = Command::cargo_bin(TEST_BIN).unwrap();
    cmd.arg("-blah");
    cmd.assert().failure();
    cmd.assert().stderr(predicate::str::contains("Usage"));
}

#[test]
fn test_no_trailing_newline() {
    let mut cmd = Command::cargo_bin(TEST_BIN).unwrap();

    cmd.arg("-n").assert().success();
    cmd.assert().stdout(predicate::str::is_empty());
}

#[test]
fn test_single_word() {
    let test_str: &str = "hello";
    let expected_str: &str = "hello\n";
    let mut cmd = Command::cargo_bin(TEST_BIN).unwrap();

    cmd.arg(test_str).assert().success();
    cmd.assert().stdout(predicate::eq(expected_str));
}

#[test]
fn test_multiple_words() {
    let test_str: &str = "hello world";
    let expected_str: &str = "hello world\n";
    let mut cmd = Command::cargo_bin(TEST_BIN).unwrap();

    cmd.arg(test_str).assert().success();
    cmd.assert().stdout(predicate::eq(expected_str));
}
