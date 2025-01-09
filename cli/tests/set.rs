use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_set_ambient_sound_mode() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("set")
        .arg("ambient-sound-mode")
        .arg("noise-canceling");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_set_noise_canceling_mode() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("set").arg("noise-canceling-mode").arg("transport");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_set_equalizer() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("set")
        .arg("equalizer")
        .arg("--")
        .args(["-120", "-60", "0", "0", "0", "60", "120", "135"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}
