use crate::device_profiles::{
    DeviceProfile, NoiseCancelingModeType, SoundModeProfile, TransparencyModeType,
};

// TODO does it support custom noise canceling or transparency modes?
pub const A3926_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    sound_mode: Some(SoundModeProfile {
        noise_canceling_mode_type: NoiseCancelingModeType::Basic,
        transparency_mode_type: TransparencyModeType::Basic,
    }),
    has_hear_id: true,
    num_equalizer_channels: 2,
    num_equalizer_bands: 8,
    has_dynamic_range_compression: false,
    dynamic_range_compression_min_firmware_version: None,
    has_custom_button_model: true,
    has_wear_detection: false,
    has_touch_tone: false,
    has_auto_power_off: false,
};
