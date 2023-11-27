use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_get_ambient_sound_mode() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("get").arg("ambient-sound-mode");
    cmd.assert()
        .success()
        .stdout(predicate::eq("normal\n"))
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_get_noise_canceling_mode() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("get").arg("noise-canceling-mode");
    cmd.assert()
        .success()
        .stdout(predicate::eq("indoor\n"))
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_get_equalizer() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("get").arg("equalizer");
    cmd.assert()
        .success()
        .stdout(predicate::eq("0 0 0 0 0 0 0 0\n"))
        .stderr(predicate::str::is_empty());
}
