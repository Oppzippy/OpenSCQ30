use crate::{
    device_profile::{
        DeviceFeatures, DeviceProfile, NoiseCancelingModeType, SoundModeProfile,
        TransparencyModeType,
    },
    devices::standard::{implementation::StandardImplementation, structures::FirmwareVersion},
    soundcore_device::device_model::DeviceModel,
};

use super::packets::A3931StateUpdatePacket;

pub(crate) const A3931_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        sound_mode: Some(SoundModeProfile {
            noise_canceling_mode_type: NoiseCancelingModeType::None,
            transparency_mode_type: TransparencyModeType::Custom,
        }),
        has_hear_id: false,
        num_equalizer_channels: 2,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: true,
        dynamic_range_compression_min_firmware_version: Some(FirmwareVersion::new(2, 0)),
        has_custom_button_model: true,
        has_wear_detection: false,
        has_touch_tone: true,
        has_auto_power_off: true,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::A3931, DeviceModel::A3935],
    implementation: || StandardImplementation::new::<A3931StateUpdatePacket>(),
};
