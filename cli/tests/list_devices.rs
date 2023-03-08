use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_list_devices() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("list-devices");
    cmd.assert()
        .success()
        .stdout(predicate::eq("00:00:00:00:00:00\n"))
        .stderr(predicate::str::is_empty());
}
