use crate::{
    device_profile::{AvailableSoundModes, DeviceFeatures, DeviceProfile},
    devices::standard::{
        implementation::StandardImplementation,
        structures::{AmbientSoundMode, NoiseCancelingMode},
    },
    soundcore_device::device_model::DeviceModel,
};

use super::packets::A3027StateUpdatePacket;

pub(crate) const A3027_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        available_sound_modes: Some(AvailableSoundModes {
            ambient_sound_modes: &[
                AmbientSoundMode::Normal,
                AmbientSoundMode::Transparency,
                AmbientSoundMode::NoiseCanceling,
            ],
            transparency_modes: &[],
            noise_canceling_modes: &[
                NoiseCancelingMode::Transport,
                NoiseCancelingMode::Indoor,
                NoiseCancelingMode::Outdoor,
            ],
            custom_noise_canceling: false,
        }),
        has_hear_id: false,
        num_equalizer_channels: 1,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: false,
        dynamic_range_compression_min_firmware_version: None,
        has_button_configuration: false,
        has_wear_detection: true,
        has_touch_tone: false,
        has_auto_power_off: false,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::A3027, DeviceModel::A3030],
    implementation: || StandardImplementation::new::<A3027StateUpdatePacket>(),
};
