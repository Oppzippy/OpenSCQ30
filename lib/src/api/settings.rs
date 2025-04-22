use std::borrow::Cow;

pub use equalizer::*;
use openscq30_i18n::Translate;
use openscq30_i18n_macros::Translate;
pub use range::*;
pub use select::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoEnumIterator, IntoStaticStr, VariantArray};
pub use value::*;

mod equalizer;
mod range;
mod select;
mod value;

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Hash,
    Clone,
    Serialize,
    Deserialize,
    Translate,
    Display,
    EnumString,
)]
pub enum CategoryId {
    General,
    SoundModes,
    Equalizer,
    ButtonConfiguration,
    DeviceInformation,
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Hash,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    EnumString,
    Translate,
    Display,
    VariantArray,
    IntoStaticStr,
)]
// Removing or renaming anything here will break quick presets, so this enum should be append only.
// If something really needs to be renamed, use #[strum(serialize = "...")] to keep the representation the same.
pub enum SettingId {
    AmbientSoundMode,
    TransparencyMode,
    NoiseCancelingMode,
    CustomNoiseCanceling,
    PresetProfile,
    CustomProfile,
    VolumeAdjustments,
    LeftSinglePress,
    LeftDoublePress,
    LeftLongPress,
    RightSinglePress,
    RightDoublePress,
    RightLongPress,
    NormalModeInCycle,
    TransparencyModeInCycle,
    NoiseCancelingModeInCycle,
    AdaptiveNoiseCanceling,
    ManualNoiseCanceling,
    WindNoiseSuppression,
    AdaptiveNoiseCancelingSensitivityLevel,
    IsCharging,
    BatteryLevel,
    IsChargingLeft,
    BatteryLevelLeft,
    IsChargingRight,
    BatteryLevelRight,
    SerialNumber,
    FirmwareVersion,
    FirmwareVersionLeft,
    FirmwareVersionRight,
    TwsStatus,
    HostDevice,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "setting")]
pub enum Setting {
    Toggle {
        value: bool,
    },
    I32Range {
        setting: Range<i32>,
        value: i32,
    },
    // Select/OptionalSelect is just a hint about whether None is an acceptable value or not.
    // The backing data is still Option<u16> for both and should be treated the same by the backend.
    Select {
        setting: Select,
        value: Cow<'static, str>,
    },
    OptionalSelect {
        setting: Select,
        value: Option<Cow<'static, str>>,
    },
    /// Allows the user to add/remove items from the select
    ModifiableSelect {
        setting: Select,
        value: Option<Cow<'static, str>>,
    },
    Equalizer {
        setting: Equalizer,
        values: Vec<i16>,
    },
    Information {
        text: String,
        translated_text: String,
    },
}

impl From<Setting> for Value {
    fn from(setting: Setting) -> Self {
        match setting {
            Setting::Toggle { value, .. } => value.into(),
            Setting::I32Range { value, .. } => value.into(),
            Setting::Select { value, .. } => value.into(),
            Setting::OptionalSelect { value, .. } => value.into(),
            Setting::Equalizer { values: value, .. } => value.into(),
            Setting::ModifiableSelect { value, .. } => value.into(),
            Setting::Information {
                text,
                translated_text: _,
            } => Cow::<str>::Owned(text).into(),
        }
    }
}

impl Setting {
    pub(crate) fn select_from_enum_all_variants<T>(value: T) -> Self
    where
        T: PartialEq + Into<&'static str> + IntoEnumIterator + Translate,
    {
        Self::Select {
            setting: Select::from_enum(T::iter()),
            value: Cow::Borrowed(value.into()),
        }
    }

    pub(crate) fn optional_select_from_enum_all_variants<T>(value: Option<T>) -> Self
    where
        T: PartialEq + Into<&'static str> + IntoEnumIterator + Translate,
    {
        Setting::OptionalSelect {
            setting: Select::from_enum(T::iter()),
            value: value.map(|v| Cow::Borrowed(v.into())),
        }
    }

    pub(crate) fn select_from_enum<T>(variants: &[T], value: T) -> Self
    where
        for<'a> &'a T: PartialEq + Into<&'static str>,
        T: Into<&'static str> + Translate,
    {
        Self::Select {
            setting: Select::from_enum(variants),
            value: Cow::Borrowed(value.into()),
        }
    }
}
