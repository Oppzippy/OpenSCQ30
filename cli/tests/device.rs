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

fn set_and_get(dir: &Path, setting: &str, value: &str) -> Command {
    let mut command = cli(dir);
    command
        .arg("device")
        .arg("exec")
        .arg("--mac-address")
        .arg("00:00:00:00:00:00")
        .arg("--set")
        .arg(format!("{setting}={value}"))
        .arg("--get")
        .arg(setting);
    command
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

#[test]
fn setting_toggle() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3959");
    assert_cmd_snapshot!(set_and_get(dir.path(), "windNoiseSuppression", "true"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID          	Value
    windNoiseSuppression	true 

    ----- stderr -----
    ");
}

#[test]
fn setting_toggle_invalid() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3959");
    assert_cmd_snapshot!(set_and_get(dir.path(), "windNoiseSuppression", "invalid"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: provided string was not `true` or `false`
    ");
}

#[test]
fn setting_i32_range() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customNoiseCanceling", "2"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID          	Value
    customNoiseCanceling	2    

    ----- stderr -----
    ");
}

#[test]
fn setting_i32_range_invalid() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customNoiseCanceling", "invalid"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: invalid digit found in string
    ");
}

#[test]
fn setting_i32_range_out_of_range() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customNoiseCanceling", "100"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: 100 is out of the expected range 0..=10
    ");
}

#[test]
fn setting_select() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3028");
    assert_cmd_snapshot!(set_and_get(dir.path(), "ambientSoundMode", "normal"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID      	Value 
    ambientSoundMode	Normal

    ----- stderr -----
    ");
}

#[test]
fn setting_select_invalid() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3028");
    assert_cmd_snapshot!(set_and_get(dir.path(), "ambientSoundMode", "invalid"), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: invalid is not a valid option. Expected one of: ["Normal", "Transparency", "NoiseCanceling"]
    "#);
}

#[test]
fn setting_optional_select() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    // select
    assert_cmd_snapshot!(set_and_get(dir.path(), "leftSinglePress", "ambientSoundMode"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID     	Value           
    leftSinglePress	AmbientSoundMode

    ----- stderr -----
    ");
    // deselect
    assert_cmd_snapshot!(set_and_get(dir.path(), "leftSinglePress", ""), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID     	Value
    leftSinglePress	     

    ----- stderr -----
    ");
}

#[test]
fn setting_optional_select_invalid() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "leftSinglePress", "invalid"), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: invalid is not a valid option. Expected one of: ["VolumeUp", "VolumeDown", "PreviousSong", "NextSong", "AmbientSoundMode", "VoiceAssistant", "PlayPause", "GameMode"]
    "#);
}

#[test]
fn setting_modifiable_select() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customEqualizerProfile", "+test profile"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID            	Value
    customEqualizerProfile	     

    ----- stderr -----
    ");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customEqualizerProfile", "test profile"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID            	Value       
    customEqualizerProfile	test profile

    ----- stderr -----
    ");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customEqualizerProfile", "-test profile"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID            	Value
    customEqualizerProfile	     

    ----- stderr -----
    ");
}

#[test]
fn setting_modifiable_select_exact_match() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    let mut command = cli(dir.path());
    command
        .arg("device")
        .arg("exec")
        .arg("--mac-address")
        .arg("00:00:00:00:00:00")
        .arg("--set")
        .arg("customEqualizerProfile=+Test")
        .arg("--set")
        .arg("volumeAdjustments=1,0,0,0,0,0,0,0")
        .arg("--set")
        .arg("customEqualizerProfile=+tesT");

    assert_cmd_snapshot!(command, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    OK

    ----- stderr -----
    ");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customEqualizerProfile", "tesT"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID            	Value
    customEqualizerProfile	tesT 

    ----- stderr -----
    ");
}

#[test]
fn setting_modifiable_select_case_insensitive_match() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customEqualizerProfile", "+Test"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID            	Value
    customEqualizerProfile	     

    ----- stderr -----
    ");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customEqualizerProfile", "tesT"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID            	Value
    customEqualizerProfile	Test 

    ----- stderr -----
    ");
}

#[test]
fn setting_modifiable_select_ambiguity() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    let mut command = cli(dir.path());
    command
        .arg("device")
        .arg("exec")
        .arg("--mac-address")
        .arg("00:00:00:00:00:00")
        .arg("--set")
        .arg("customEqualizerProfile=+Test")
        .arg("--set")
        .arg("volumeAdjustments=1,0,0,0,0,0,0,0")
        .arg("--set")
        .arg("customEqualizerProfile=+tesT");

    assert_cmd_snapshot!(command, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    OK

    ----- stderr -----
    ");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customEqualizerProfile", "test"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: test is ambiguous, could refer to Test or tesT
    ");
}

#[test]
fn setting_modifiable_select_invalid() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customEqualizerProfile", "invalid"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: invalid is not a valid option. Expected one of: []
    ");
}

#[test]
fn setting_modifiable_select_remove_invalid() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "customEqualizerProfile", "-invalid"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID            	Value
    customEqualizerProfile	     

    ----- stderr -----
    ");
}

#[test]
fn setting_multi_select() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    let mut command = cli(dir.path());
    command
        .arg("device")
        .arg("exec")
        .arg("--mac-address")
        .arg("00:00:00:00:00:00")
        .arg("--set")
        .arg("customEqualizerProfile=+test")
        .arg("--set")
        .arg("volumeAdjustments=1,0,0,0,0,0,0,0")
        .arg("--set")
        .arg("customEqualizerProfile=+has,comma and\"quote")
        .arg("--set")
        .arg("volumeAdjustments=2,0,0,0,0,0,0,0")
        .arg("--set")
        .arg("customEqualizerProfile=+other not selected");
    assert_cmd_snapshot!(command, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    OK

    ----- stderr -----
    ");
    assert_cmd_snapshot!(set_and_get(
        dir.path(),
        "exportCustomProfiles",
        r#"test,"has,comma and""quote""#
    ), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID          	Value                      
    exportCustomProfiles	"has,comma and""quote",test

    ----- stderr -----
    "#);
}

#[test]
fn setting_multi_select_no_selection() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "exportCustomProfiles", ""), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID          	Value
    exportCustomProfiles	     

    ----- stderr -----
    ");
}

#[test]
fn setting_multi_select_invalid() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "exportCustomProfiles", "invalid"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: invalid is not a valid option. Expected one of: []
    ");
}

#[test]
fn setting_equalizer() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "volumeAdjustments", "10,-20,30,-40,50,-60,70,-80"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID       	Value                               
    volumeAdjustments	[10, -20, 30, -40, 50, -60, 70, -80]

    ----- stderr -----
    ");
}

#[test]
fn setting_equalizer_too_few_values() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "volumeAdjustments", "10,-20,30,-40,50,-60,70"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: wanted 8 bands, got 7
    ");
}

#[test]
fn setting_equalizer_too_many_values() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "volumeAdjustments", "10,-20,30,-40,50,-60,70,-80,90"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: wanted 8 bands, got 9
    ");
}

#[test]
fn setting_equalizer_out_of_range() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "volumeAdjustments", "1000,0,0,0,0,0,0,0"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: 100 Hz band value 1000 is outside of expected range -120 to 134
    ");
}

#[test]
fn setting_equalizer_invalid() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    assert_cmd_snapshot!(set_and_get(dir.path(), "volumeAdjustments", "invalid"), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: invalid digit found in string
    ");
}

#[test]
fn setting_information_get() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    let mut command = cli(dir.path());
    command
        .arg("device")
        .arg("exec")
        .arg("--mac-address")
        .arg("00:00:00:00:00:00")
        .arg("--get")
        .arg("serialNumber");
    assert_cmd_snapshot!(command, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID  	Value           
    serialNumber	0000000000000000

    ----- stderr -----
    ");
}

#[test]
fn setting_information_set() {
    // TODO improve error message and say what setting id caused the problem
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    let mut command = cli(dir.path());
    command
        .arg("device")
        .arg("exec")
        .arg("--mac-address")
        .arg("00:00:00:00:00:00")
        .arg("--set")
        .arg("serialNumber=0123456789ABCDEF");
    assert_cmd_snapshot!(command, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: can't set value of read only information setting
    ");
}

#[test]
fn setting_import_string_set() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    let mut command = cli(dir.path());
    command
        .arg("device")
        .arg("exec")
        .arg("--mac-address")
        .arg("00:00:00:00:00:00")
        .arg("--set")
        .arg(r#"importCustomProfiles=[{"name": "test profile", "volumeAdjustments": [0,1,2,3,4,5,6,7]}]"#)
        .arg("--set")
        .arg("customEqualizerProfile=test profile")
        .arg("--get")
        .arg("customEqualizerProfile");
    assert_cmd_snapshot!(command, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID            	Value       
    customEqualizerProfile	test profile

    ----- stderr -----
    ");
}

#[test]
fn setting_import_string_set_invalid() {
    // TODO improve error message
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    let mut command = cli(dir.path());
    command
        .arg("device")
        .arg("exec")
        .arg("--mac-address")
        .arg("00:00:00:00:00:00")
        .arg("--set")
        .arg(r#"importCustomProfiles=invalid"#);
    assert_cmd_snapshot!(command, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: expected value at line 1 column 1

    Caused by:
        expected value at line 1 column 1
    ");
}

#[test]
fn setting_import_string_get() {
    let dir = tempdir().unwrap();
    add_device(dir.path(), "SoundcoreA3951");
    let mut command = cli(dir.path());
    command
        .arg("device")
        .arg("exec")
        .arg("--mac-address")
        .arg("00:00:00:00:00:00")
        .arg("--get")
        .arg("importCustomProfiles");
    assert_cmd_snapshot!(command, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Setting ID          	Value
    importCustomProfiles	     

    ----- stderr -----
    ");
}
