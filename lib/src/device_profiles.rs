use serde::{Deserialize, Serialize};

use crate::{
    devices::standard::structures::FirmwareVersion,
    soundcore_device::device::device_command_dispatcher::DeviceCommandDispatcher,
};

// TODO remove deserialize
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

    #[serde(skip)]
    pub custom_dispatchers: Option<fn() -> Box<dyn DeviceCommandDispatcher>>,
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
