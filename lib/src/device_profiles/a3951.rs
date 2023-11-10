use super::{DeviceProfile, NoiseCancelingModeType, SoundModeProfile, TransparencyModeType};

pub const A3951_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
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
};
