use std::{path::Path, process::Command};

use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use tempfile::tempdir;

fn cli(dir: &Path) -> Command {
    let mut cmd = Command::new(get_cargo_bin("openscq30"));
    cmd.env("XDG_CONFIG_HOME", dir.to_str().unwrap());
    cmd
}

fn add_device(dir: &Path, model: &str) {
    let output = cli(dir)
        .arg("paired-devices")
        .arg("add")
        .arg("--mac-address")
        .arg("00:00:00:00:00:00")
        .arg("--model")
        .arg(model)
        .arg("--demo")
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
fn list_settings() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3027");
    assert_cmd_snapshot!(cli(dir.path()).arg("device").arg("list-settings").arg("--mac-address").arg("00:00:00:00:00:00"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    -- soundModes --
    ambientSoundMode: select (["Normal", "Transparency", "NoiseCanceling"])
    noiseCancelingMode: select (["Transport", "Indoor", "Outdoor"])
    -- equalizer --
    presetEqualizerProfile: optional select (["SoundcoreSignature", "Acoustic", "BassBooster", "BassReducer", "Classical", "Podcast", "Dance", "Deep", "Electronic", "Flat", "HipHop", "Jazz", "Latin", "Lounge", "Piano", "Pop", "RnB", "Rock", "SmallSpeakers", "SpokenWord", "TrebleBooster", "TrebleReducer"])
    customEqualizerProfile: modifiable select ([])
    volumeAdjustments: equalizer (bands: [100, 200, 400, 800, 1600, 3200, 6400, 12800], min: -120, max: 134, fractional digits: 1)
    -- equalizerImportExport --
    importCustomProfiles: import string
    exportCustomProfiles: multi select ([])
    exportCustomProfilesOutput: information (read only)
    -- deviceInformation --
    isCharging: information (read only)
    batteryLevel: information (read only)
    serialNumber: information (read only)
    firmwareVersion: information (read only)

    ----- stderr -----
    "#);
}
