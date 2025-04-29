use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, bail};
use openscq30_lib::api::{
    device::OpenSCQ30Device,
    settings::{self, Setting, SettingId, Value},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LegacyConfig {
    equalizer_custom_profiles: HashMap<String, LegacyEqualizerProfile>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LegacyEqualizerProfile {
    // Not renamed to volume_adjustments to keep backwards compatibility with old settings files
    pub volume_offsets: Vec<i16>,
}

#[derive(Debug, thiserror::Error)]
pub enum FetchProfilesError {
    #[error("there is no legacy config file")]
    NoLegacyConfig,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    TOMLError(#[from] toml::de::Error),
}

pub async fn all_equalizer_profiles(
    config_dir: PathBuf,
) -> Result<HashMap<String, LegacyEqualizerProfile>, FetchProfilesError> {
    let path = config_dir.join("config.toml");
    let legacy_config_toml = match tokio::fs::read_to_string(path).await {
        Ok(text) => Ok(text),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => Err(FetchProfilesError::NoLegacyConfig),
            _ => Err(err.into()),
        },
    }?;
    let legacy_config: LegacyConfig = toml::from_str(&legacy_config_toml)?;
    Ok(legacy_config.equalizer_custom_profiles)
}

pub async fn migrate_legacy_profile(
    device: &(dyn OpenSCQ30Device + Send + Sync),
    name: String,
    volume_adjustments: Vec<i16>,
) -> anyhow::Result<()> {
    let Setting::Equalizer {
        values: old_volume_adjustments,
        ..
    } = device
        .setting(&SettingId::VolumeAdjustments)
        .ok_or_else(|| anyhow!("device does not have VolumeAdjustments setting"))?
    else {
        bail!("VolumeAdjustments is not an equalizer");
    };
    // By setting the volume adjustments, saving the profile, and reverting all in one go,
    // we won't send any packets, since the end result will be equivalent to what we started with,
    // and that is checked before performing any actions.
    device
        .set_setting_values(vec![
            (SettingId::VolumeAdjustments, volume_adjustments.into()),
            (
                SettingId::CustomEqualizerProfile,
                Value::ModifiableSelectCommand(settings::ModifiableSelectCommand::Add(name.into())),
            ),
            (SettingId::VolumeAdjustments, old_volume_adjustments.into()),
        ])
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use macaddr::MacAddr6;
    use openscq30_lib::{
        api::{
            OpenSCQ30Session,
            settings::{Setting, SettingId, Value},
        },
        devices::DeviceModel,
        storage::PairedDevice,
    };

    use crate::openscq30_v1_migration::migrate_legacy_profile;

    #[tokio::test]
    async fn test_migrate_legacy_profile() {
        let config_dir = tempfile::tempdir().unwrap();
        let session = OpenSCQ30Session::new(config_dir.path().join("database.sqlite").to_owned())
            .await
            .unwrap();
        session
            .pair(PairedDevice {
                mac_address: MacAddr6::nil(),
                model: DeviceModel::SoundcoreA3027,
                is_demo: true,
            })
            .await
            .unwrap();
        let device = session.connect(MacAddr6::nil()).await.unwrap();

        let Setting::ModifiableSelect { setting, .. } =
            device.setting(&SettingId::CustomEqualizerProfile).unwrap()
        else {
            panic!("CustomProfile is not a ModifiableSelect");
        };
        assert!(setting.options.is_empty());
        migrate_legacy_profile(device.as_ref(), "Test Profile".to_string(), vec![1; 8])
            .await
            .unwrap();
        let Setting::ModifiableSelect { setting, .. } =
            device.setting(&SettingId::CustomEqualizerProfile).unwrap()
        else {
            panic!("CustomProfile is not a ModifiableSelect");
        };
        assert_eq!(
            setting.options,
            vec![Cow::from("Test Profile")],
            "custom profile should be added to the dropdown",
        );

        let Setting::Equalizer { values, .. } =
            device.setting(&SettingId::VolumeAdjustments).unwrap()
        else {
            panic!("VolumeAdjustments is not an Equalizer");
        };
        assert_eq!(
            values,
            vec![0; 8],
            "the migrated custom profile should not be activated"
        );

        device
            .set_setting_values(vec![(
                SettingId::CustomEqualizerProfile,
                Value::String("Test Profile".into()),
            )])
            .await
            .unwrap();
        let Setting::Equalizer { values, .. } =
            device.setting(&SettingId::VolumeAdjustments).unwrap()
        else {
            panic!("VolumeAdjustments is not an Equalizer");
        };
        assert_eq!(
            values, [1; 8],
            "the migrated profile should have the correct volume adjustments"
        );
    }
}
