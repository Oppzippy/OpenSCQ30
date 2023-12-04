use crate::device_profiles::DeviceProfile;

pub const A3033_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    sound_mode: None,
    has_hear_id: false,
    num_equalizer_channels: 1,
    num_equalizer_bands: 8,
    has_dynamic_range_compression: false,
    dynamic_range_compression_min_firmware_version: None,
    has_custom_button_model: false,
    has_wear_detection: false,
    has_touch_tone: false,
    has_auto_power_off: false,
    custom_dispatchers: None,
};
