use crate::{
    device_profile::{
        DeviceFeatures, DeviceProfile, NoiseCancelingModeType, SoundModeProfile,
        TransparencyModeType,
    },
    devices::standard::implementation::StandardImplementation,
    soundcore_device::device_model::DeviceModel,
};

use super::packets::A3951StateUpdatePacket;

pub(crate) const A3951_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        sound_mode: Some(SoundModeProfile {
            noise_canceling_mode_type: NoiseCancelingModeType::Custom,
            transparency_mode_type: TransparencyModeType::Custom,
        }),
        has_hear_id: true,
        num_equalizer_channels: 2,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: true,
        dynamic_range_compression_min_firmware_version: None,
        has_custom_button_model: true,
        has_wear_detection: true,
        has_touch_tone: true,
        has_auto_power_off: false,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::A3951],
    implementation: || StandardImplementation::new::<A3951StateUpdatePacket>(),
};
