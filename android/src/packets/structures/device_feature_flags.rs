use openscq30_lib::packets::structures::DeviceFeatureFlags as LibDeviceFeatureFlags;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct DeviceFeatureFlags(LibDeviceFeatureFlags);

impl DeviceFeatureFlags {
    // functions
    #[generate_interface(constructor)]
    pub fn new(bits: i32) -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::from(bits as u32).into()
    }

    #[generate_interface]
    pub fn or(&self, other: &DeviceFeatureFlags) -> DeviceFeatureFlags {
        self.0.union(other.0).into()
    }

    #[generate_interface]
    pub fn contains(&self, other: &DeviceFeatureFlags) -> bool {
        self.0.contains(other.0)
    }

    #[generate_interface]
    pub fn bits(&self) -> i32 {
        self.0.bits() as i32
    }

    // constants
    #[generate_interface]
    pub fn sound_modes() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::SOUND_MODES.into()
    }

    #[generate_interface]
    pub fn noise_canceling_mode() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::NOISE_CANCELING_MODE.into()
    }

    #[generate_interface]
    pub fn custom_noise_canceling() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::CUSTOM_NOISE_CANCELING.into()
    }

    #[generate_interface]
    pub fn transparency_modes() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::TRANSPARENCY_MODES.into()
    }

    #[generate_interface]
    pub fn hear_id() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::HEAR_ID.into()
    }

    #[generate_interface]
    pub fn equalizer() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::EQUALIZER.into()
    }

    #[generate_interface]
    pub fn custom_button_model() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::CUSTOM_BUTTON_MODEL.into()
    }

    #[generate_interface]
    pub fn wear_detection() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::WEAR_DETECTION.into()
    }

    #[generate_interface]
    pub fn touch_tone() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::TOUCH_TONE.into()
    }

    #[generate_interface]
    pub fn auto_power_off() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::AUTO_POWER_OFF.into()
    }

    #[generate_interface]
    pub fn two_channel_equalizer() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::TWO_CHANNEL_EQUALIZER.into()
    }

    #[generate_interface]
    pub fn dynamic_range_compression() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::DYNAMIC_RANGE_COMPRESSION.into()
    }

    #[generate_interface]
    pub fn all() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::all().into()
    }

    #[generate_interface]
    pub fn empty() -> DeviceFeatureFlags {
        LibDeviceFeatureFlags::empty().into()
    }
}

impl From<LibDeviceFeatureFlags> for DeviceFeatureFlags {
    fn from(value: LibDeviceFeatureFlags) -> Self {
        Self(value)
    }
}

impl From<DeviceFeatureFlags> for LibDeviceFeatureFlags {
    fn from(value: DeviceFeatureFlags) -> Self {
        value.0
    }
}
