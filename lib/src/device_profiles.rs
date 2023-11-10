mod a3027;
mod a3028;
mod a3033;
mod a3926;
mod a3930;
mod a3931;
mod a3933;
mod a3945;
mod a3951;

pub use a3027::*;
pub use a3028::*;
pub use a3033::*;
pub use a3926::*;
pub use a3930::*;
pub use a3931::*;
pub use a3933::*;
pub use a3945::*;
pub use a3951::*;
use serde::{Deserialize, Serialize};

use crate::packets::structures::FirmwareVersion;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProfile {
    pub sound_mode: Option<SoundModeProfile>,
    pub has_hear_id: bool,
    pub num_equalizer_channels: usize,
    pub num_equalizer_bands: usize,
    pub has_dynamic_range_compression: bool,
    pub has_custom_button_model: bool,
    pub has_wear_detection: bool,
    pub has_touch_tone: bool,
    pub has_auto_power_off: bool,
    pub dynamic_range_compression_min_firmware_version: Option<FirmwareVersion>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundModeProfile {
    pub noise_canceling_mode_type: NoiseCancelingModeType,
    pub transparency_mode_type: TransparencyModeType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NoiseCancelingModeType {
    #[default]
    None,
    Basic,
    Custom,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransparencyModeType {
    #[default]
    Basic,
    Custom,
}
