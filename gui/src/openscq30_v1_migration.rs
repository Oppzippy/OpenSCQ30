use std::{collections::HashMap, path::PathBuf};

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
