use openscq30_lib::packets::structures::DeviceFeatureFlags as LibDeviceFeatureFlags;
use rifgen::rifgen_attr::generate_interface;

pub struct DeviceFeatureFlags {}

impl DeviceFeatureFlags {
    #[generate_interface]
    pub fn sound_modes() -> i32 {
        LibDeviceFeatureFlags::SOUND_MODES.bits() as i32
    }

    #[generate_interface]
    pub fn noise_canceling_mode() -> i32 {
        LibDeviceFeatureFlags::NOISE_CANCELING_MODE.bits() as i32
    }

    #[generate_interface]
    pub fn custom_noise_canceling() -> i32 {
        LibDeviceFeatureFlags::CUSTOM_NOISE_CANCELING.bits() as i32
    }

    #[generate_interface]
    pub fn transparency_modes() -> i32 {
        LibDeviceFeatureFlags::TRANSPARENCY_MODES.bits() as i32
    }

    #[generate_interface]
    pub fn hear_id() -> i32 {
        LibDeviceFeatureFlags::HEAR_ID.bits() as i32
    }

    #[generate_interface]
    pub fn equalizer() -> i32 {
        LibDeviceFeatureFlags::EQUALIZER.bits() as i32
    }

    #[generate_interface]
    pub fn custom_button_model() -> i32 {
        LibDeviceFeatureFlags::CUSTOM_BUTTON_MODEL.bits() as i32
    }

    #[generate_interface]
    pub fn wear_detection() -> i32 {
        LibDeviceFeatureFlags::WEAR_DETECTION.bits() as i32
    }

    #[generate_interface]
    pub fn touch_tone() -> i32 {
        LibDeviceFeatureFlags::TOUCH_TONE.bits() as i32
    }

    #[generate_interface]
    pub fn auto_power_off() -> i32 {
        LibDeviceFeatureFlags::AUTO_POWER_OFF.bits() as i32
    }

    #[generate_interface]
    pub fn two_channel_equalizer() -> i32 {
        LibDeviceFeatureFlags::TWO_CHANNEL_EQUALIZER.bits() as i32
    }
}
