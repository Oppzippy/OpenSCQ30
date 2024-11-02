use crate::{
    device_profile::{
        DeviceFeatures, DeviceProfile, NoiseCancelingModeType, SoundModeProfile,
        TransparencyModeType,
    },
    devices::standard::implementation::StandardImplementation,
    soundcore_device::device_model::DeviceModel,
};

use super::packets::A3031StateUpdatePacket;

pub(crate) const A3031_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        sound_mode: Some(SoundModeProfile {
            noise_canceling_mode_type: NoiseCancelingModeType::Basic,
            transparency_mode_type: TransparencyModeType::Basic,
        }),
        has_hear_id: false,
        num_equalizer_channels: 1,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: false,
        dynamic_range_compression_min_firmware_version: None,
        has_custom_button_model: true,
        has_wear_detection: false,
        has_touch_tone: true,
        has_auto_power_off: true,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::A3031],
    implementation: || StandardImplementation::new::<A3031StateUpdatePacket>(),
};
