use assert_cmd::Command;

#[test]
fn fails_no_args() {
    let mut cmd = Command::cargo_bin("pl-echo").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}
