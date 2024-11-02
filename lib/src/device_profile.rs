use std::sync::Arc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    devices::{
        a3027::device_profile::A3027_DEVICE_PROFILE,
        a3028::device_profile::A3028_DEVICE_PROFILE,
        a3030::device_profile::A3030_DEVICE_PROFILE,
        a3031::device_profile::A3031_DEVICE_PROFILE,
        a3033::device_profile::A3033_DEVICE_PROFILE,
        a3926::device_profile::A3926_DEVICE_PROFILE,
        a3930::device_profile::A3930_DEVICE_PROFILE,
        a3931::device_profile::A3931_DEVICE_PROFILE,
        a3933::device_profile::A3933_DEVICE_PROFILE,
        a3936::device_profile::A3936_DEVICE_PROFILE,
        a3945::device_profile::A3945_DEVICE_PROFILE,
        a3951::device_profile::A3951_DEVICE_PROFILE,
        standard::structures::{FirmwareVersion, SerialNumber},
    },
    soundcore_device::{
        device::device_implementation::DeviceImplementation, device_model::DeviceModel,
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) struct DeviceProfile {
    pub features: DeviceFeatures,
    pub compatible_models: &'static [DeviceModel],
    pub implementation: fn() -> Arc<dyn DeviceImplementation + Send + Sync>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DeviceFeatures {
    pub sound_mode: Option<SoundModeProfile>,
    pub has_hear_id: bool,
    pub num_equalizer_channels: usize,
    pub num_equalizer_bands: usize,
    pub has_dynamic_range_compression: bool,
    pub has_custom_button_model: bool,
    pub has_wear_detection: bool,
    pub has_touch_tone: bool,
    pub has_auto_power_off: bool,
    pub has_ambient_sound_mode_cycle: bool,
    pub dynamic_range_compression_min_firmware_version: Option<FirmwareVersion>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SoundModeProfile {
    pub noise_canceling_mode_type: NoiseCancelingModeType,
    pub transparency_mode_type: TransparencyModeType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum NoiseCancelingModeType {
    #[default]
    None,
    Basic,
    Custom,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum TransparencyModeType {
    #[default]
    Basic,
    Custom,
}

pub enum ToDeviceProfileError {
    ModelDoesNotExist,
    ProfileDoesNotExist,
}

// TODO decide if this is worth keeping, since it's non-trivial to determine a device's serial
// number without knowing in advance what device it is
#[allow(dead_code)]
const DEVICE_PROFILES: &[&DeviceProfile] = &[
    &A3027_DEVICE_PROFILE,
    &A3028_DEVICE_PROFILE,
    &A3030_DEVICE_PROFILE,
    &A3031_DEVICE_PROFILE,
    &A3033_DEVICE_PROFILE,
    &A3926_DEVICE_PROFILE,
    &A3930_DEVICE_PROFILE,
    &A3931_DEVICE_PROFILE,
    &A3933_DEVICE_PROFILE,
    &A3936_DEVICE_PROFILE,
    &A3945_DEVICE_PROFILE,
    &A3951_DEVICE_PROFILE,
];

#[allow(dead_code)]
impl DeviceProfile {
    pub fn from_serial_number(serial_number: &SerialNumber) -> Option<&'static DeviceProfile> {
        let model = DeviceModel::from_serial_number(serial_number)?;
        Self::from_model(&model)
    }

    pub fn from_model(model: &DeviceModel) -> Option<&'static DeviceProfile> {
        DEVICE_PROFILES
            .iter()
            .find(|profile| profile.compatible_models.iter().any(|m| m == model))
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_device_models_to_profiles_is_one_to_one() {
        let mut seen_models = HashSet::new();
        for profile in DEVICE_PROFILES {
            for model in profile.compatible_models {
                assert!(
                    seen_models.insert(model),
                    "{model} should not appear in multiple profiles",
                );
            }
        }
    }
}
