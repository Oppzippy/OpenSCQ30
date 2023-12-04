use crate::{
    device_profiles::{
        DeviceProfile, NoiseCancelingModeType, SoundModeProfile, TransparencyModeType,
    },
    devices::standard::structures::FirmwareVersion,
};

pub const A3931_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    sound_mode: Some(SoundModeProfile {
        noise_canceling_mode_type: NoiseCancelingModeType::None,
        transparency_mode_type: TransparencyModeType::Custom,
    }),
    has_hear_id: false,
    num_equalizer_channels: 2,
    num_equalizer_bands: 8,
    has_dynamic_range_compression: true,
    dynamic_range_compression_min_firmware_version: Some(FirmwareVersion::new(2, 0)),
    has_custom_button_model: true,
    has_wear_detection: false,
    has_touch_tone: true,
    has_auto_power_off: true,
    custom_dispatchers: None,
};
