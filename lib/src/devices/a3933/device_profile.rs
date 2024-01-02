use std::sync::Arc;

use crate::{
    device_profiles::{
        DeviceProfile, NoiseCancelingModeType, SoundModeProfile, TransparencyModeType,
    },
    devices::a3945::device_profile::A3945Dispatcher,
};

pub const A3933_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    sound_mode: Some(SoundModeProfile {
        noise_canceling_mode_type: NoiseCancelingModeType::Basic,
        transparency_mode_type: TransparencyModeType::Custom,
    }),
    has_hear_id: false,
    num_equalizer_channels: 2,
    num_equalizer_bands: 8,
    has_dynamic_range_compression: true,
    dynamic_range_compression_min_firmware_version: None,
    has_custom_button_model: true,
    has_wear_detection: false,
    has_touch_tone: false,
    has_auto_power_off: false,
    has_ambient_sound_mode_cycle: true,
    // The A3933 has the same quirks as the A3945
    custom_dispatchers: Some(|| Arc::new(A3945Dispatcher::default())),
};
