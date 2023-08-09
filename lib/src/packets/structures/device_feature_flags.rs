use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    #[serde(from = "u32", into = "u32")]
    pub struct DeviceFeatureFlags: u32 {
        const SOUND_MODES               = 1 << 0;
        const NOISE_CANCELING_MODE      = 1 << 1;
        const CUSTOM_NOISE_CANCELING    = 1 << 2;
        const TRANSPARENCY_MODES        = 1 << 3;
        const HEAR_ID                   = 1 << 4;
        const EQUALIZER                 = 1 << 5;
        const CUSTOM_BUTTON_MODEL       = 1 << 6;
        const WEAR_DETECTION            = 1 << 7;
        const TOUCH_TONE                = 1 << 8;
        const AUTO_POWER_OFF            = 1 << 9;
        const TWO_CHANNEL_EQUALIZER     = 1 << 10;
        const DYNAMIC_RANGE_COMPRESSION = 1 << 11;
    }
}

// TODO replace with TryFrom
impl From<u32> for DeviceFeatureFlags {
    fn from(value: u32) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl From<DeviceFeatureFlags> for u32 {
    fn from(value: DeviceFeatureFlags) -> Self {
        value.bits()
    }
}
