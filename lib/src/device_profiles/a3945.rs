use super::DeviceProfile;

pub const A3945_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    sound_mode: None,
    has_hear_id: false,
    num_equalizer_channels: 2,
    num_equalizer_bands: 10,
    has_dynamic_range_compression: false,
    dynamic_range_compression_min_firmware_version: None,
    has_custom_button_model: true,
    has_wear_detection: false,
    has_touch_tone: false,
    has_auto_power_off: false,
};
