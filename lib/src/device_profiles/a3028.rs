use super::{DeviceProfile, NoiseCancelingModeType, SoundModeProfile, TransparencyModeType};

pub const A3028_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    sound_mode: Some(SoundModeProfile {
        noise_canceling_mode_type: NoiseCancelingModeType::Basic,
        transparency_mode_type: TransparencyModeType::Basic,
    }),
    has_hear_id: false,
    num_equalizer_channels: 1,
    num_equalizer_bands: 8,
    has_dynamic_range_compression: false,
    dynamic_range_compression_min_firmware_version: None,
    has_custom_button_model: false,
    has_wear_detection: false,
    has_touch_tone: false,
    has_auto_power_off: false,
};
