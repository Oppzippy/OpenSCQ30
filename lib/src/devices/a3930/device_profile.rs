use crate::{
    device_profile::{DeviceFeatures, DeviceProfile},
    devices::standard::{
        implementation::StandardImplementation, macros::soundcore_device,
        modules::sound_modes::AvailableSoundModes, structures::AmbientSoundMode,
    },
    soundcore_device::device_model::DeviceModel,
};

use super::{packets::A3930StateUpdatePacket, state::A3930State};

pub(crate) const A3930_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        available_sound_modes: Some(crate::device_profile::AvailableSoundModes {
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
    compatible_models: &[DeviceModel::SoundcoreA3930],
    implementation: || StandardImplementation::new::<A3930StateUpdatePacket>(),
};

soundcore_device!(A3930Device with A3930State initialized by A3930StateUpdatePacket => {
    state_update();
    sound_modes(AvailableSoundModes {
        ambient_sound_modes: vec![AmbientSoundMode::Normal, AmbientSoundMode::Transparency],
        transparency_modes: Vec::new(),
        noise_canceling_modes: Vec::new(),
    });
    equalizer_with_custom_hear_id();
    button_configuration();
});
