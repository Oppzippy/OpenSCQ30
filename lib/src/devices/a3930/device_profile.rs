use crate::{
    device_profile::{AvailableSoundModes, DeviceFeatures, DeviceProfile},
    devices::standard::{implementation::StandardImplementation, structures::AmbientSoundMode},
    soundcore_device::device_model::DeviceModel,
};

use super::packets::A3930StateUpdatePacket;

pub(crate) const A3930_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        available_sound_modes: Some(AvailableSoundModes {
            ambient_sound_modes: &[AmbientSoundMode::Normal, AmbientSoundMode::Transparency],
            transparency_modes: &[],
            noise_canceling_modes: &[],
            custom_noise_canceling: false,
        }),
        has_hear_id: true,
        num_equalizer_channels: 2,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: true,
        dynamic_range_compression_min_firmware_version: None,
        has_button_configuration: false,
        has_wear_detection: false,
        has_touch_tone: false,
        has_auto_power_off: false,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::A3930],
    implementation: || StandardImplementation::new::<A3930StateUpdatePacket>(),
};
