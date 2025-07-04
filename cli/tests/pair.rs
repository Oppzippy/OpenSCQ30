use std::{path::Path, process::Command};

use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use tempfile::tempdir;

fn cli(dir: &Path) -> Command {
    let mut cmd = Command::new(get_cargo_bin("openscq30"));
    cmd.env("XDG_CONFIG_HOME", dir.to_str().unwrap());
    cmd
}

#[test]
fn add_device() {
    let dir = tempdir().unwrap();
    assert_cmd_snapshot!(
        cli(dir.path())
            .arg("paired-devices")
            .arg("add")
            .arg("--mac-address")
            .arg("00:00:00:00:00:00")
            .arg("--model")
            .arg("SoundcoreA3027"),
            @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Paired

    ----- stderr -----
    "
    );
    assert_cmd_snapshot!(cli(dir.path()).arg("paired-devices").arg("list"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Device Model  	MAC Address      	Demo Mode
    SoundcoreA3027	00:00:00:00:00:00	No       

    ----- stderr -----
    ");
}

#[test]
fn add_demo_device() {
    let dir = tempdir().unwrap();
    assert_cmd_snapshot!(
        cli(dir.path())
            .arg("paired-devices")
            .arg("add")
            .arg("--mac-address")
            .arg("00:00:00:00:00:00")
            .arg("--model")
            .arg("SoundcoreA3027")
            .arg("--demo"),
            @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Paired

    ----- stderr -----
    "
    );
    assert_cmd_snapshot!(cli(dir.path()).arg("paired-devices").arg("list"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Device Model  	MAC Address      	Demo Mode
    SoundcoreA3027	00:00:00:00:00:00	Yes      

    ----- stderr -----
    ");
}

#[test]
fn remove_device() {
    let dir = tempdir().unwrap();
    assert_cmd_snapshot!(
        cli(dir.path())
            .arg("paired-devices")
            .arg("add")
            .arg("--mac-address")
            .arg("00:00:00:00:00:00")
            .arg("--model")
            .arg("SoundcoreA3027"),
            @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Paired

    ----- stderr -----
    "
    );
    assert_cmd_snapshot!(
        cli(dir.path())
            .arg("paired-devices")
            .arg("remove")
            .arg("--mac-address")
            .arg("00:00:00:00:00:00"),
            @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Unpaired

    ----- stderr -----
    "
    );
    assert_cmd_snapshot!(cli(dir.path()).arg("paired-devices").arg("list"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Device Model	MAC Address	Demo Mode

    ----- stderr -----
    ");
}
