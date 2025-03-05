use crate::{
    device_profile::{DeviceFeatures, DeviceProfile},
    devices::standard::{
        implementation::StandardImplementation,
        macros::soundcore_device,
        modules::sound_modes::AvailableSoundModes,
        structures::{AmbientSoundMode, NoiseCancelingMode},
    },
    soundcore_device::device_model::DeviceModel,
};

use super::{packets::A3028StateUpdatePacket, state::A3028State};

pub(crate) const A3028_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        available_sound_modes: Some(crate::device_profile::AvailableSoundModes {
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
        has_wear_detection: false,
        has_touch_tone: false,
        has_auto_power_off: false,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::SoundcoreA3028],
    implementation: || StandardImplementation::new::<A3028StateUpdatePacket>(),
};

soundcore_device!(A3028Device with A3028State initialized by A3028StateUpdatePacket => {
    state_update();
    sound_modes(AvailableSoundModes {
        ambient_sound_modes: vec![
            AmbientSoundMode::Normal,
            AmbientSoundMode::Transparency,
            AmbientSoundMode::NoiseCanceling,
        ],
        transparency_modes: vec![],
        noise_canceling_modes: vec![
            NoiseCancelingMode::Transport,
            NoiseCancelingMode::Indoor,
            NoiseCancelingMode::Outdoor,
        ],
    });
    equalizer(mono);
});
