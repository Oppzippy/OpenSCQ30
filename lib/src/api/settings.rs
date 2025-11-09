use std::borrow::Cow;

pub use equalizer::*;
use openscq30_i18n::Translate;
use openscq30_i18n_macros::Translate;
pub use range::*;
pub use select::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoEnumIterator, IntoStaticStr, VariantArray};
pub use value::*;

use crate::i18n::fl;

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
    Copy,
    Serialize,
    Deserialize,
    Translate,
    Display,
    EnumString,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum CategoryId {
    General,
    SoundModes,
    Equalizer,
    EqualizerImportExport,
    ButtonConfiguration,
    DeviceInformation,
    Miscellaneous,
    LimitHighVolume,
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
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
// Removing or renaming anything here will break quick presets, so this enum should be append only.
// If something really needs to be renamed, use #[strum(serialize = "...")] to keep the representation the same.
pub enum SettingId {
    AmbientSoundMode,
    TransparencyMode,
    NoiseCancelingMode,
    CustomNoiseCanceling,
    #[translate("preset-profile")]
    PresetEqualizerProfile,
    #[translate("custom-profile")]
    CustomEqualizerProfile,
    VolumeAdjustments,
    LeftSinglePress,
    LeftDoublePress,
    LeftTriplePress,
    LeftLongPress,
    RightSinglePress,
    RightDoublePress,
    RightTriplePress,
    RightLongPress,
    NormalModeInCycle,
    TransparencyModeInCycle,
    NoiseCancelingModeInCycle,
    AdaptiveNoiseCanceling,
    ManualNoiseCanceling,
    ManualTransparency,
    WindNoiseSuppression,
    WindNoiseDetected,
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
    StateUpdatePacket,
    SendPacket,
    MultiSceneNoiseCanceling,
    ExportCustomEqualizerProfiles,
    ExportCustomEqualizerProfilesOutput,
    ImportCustomEqualizerProfiles,
    AutoPowerOff,
    TouchTone,
    ResetButtonsToDefault,
    TransportationMode,
    EnvironmentDetection,
    LimitHighVolume,
    #[translate("db-limit")]
    LimitHighVolumeDbLimit,
    #[translate("db-refresh-rate")]
    LimitHighVolumeRefreshRate,
    CaseBatteryLevel,
    GamingMode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
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
    MultiSelect {
        setting: Select,
        values: Vec<Cow<'static, str>>,
    },
    Equalizer {
        setting: Equalizer,
        value: Vec<i16>,
    },
    Information {
        value: String,
        translated_value: String,
    },
    ImportString {
        confirmation_message: Option<String>,
    },
    Action,
}

impl From<Setting> for Value {
    fn from(setting: Setting) -> Self {
        match setting {
            Setting::Toggle { value, .. } => value.into(),
            Setting::I32Range { value, .. } => value.into(),
            Setting::Select { value, .. } => value.into(),
            Setting::OptionalSelect { value, .. } => value.into(),
            Setting::Equalizer { value, .. } => value.into(),
            Setting::ModifiableSelect { value, .. } => value.into(),
            Setting::Information {
                value,
                translated_value: _,
            } => Cow::<str>::Owned(value).into(),
            Setting::MultiSelect { values, .. } => values.into(),
            Setting::ImportString { .. } => Cow::from("").into(),
            Setting::Action => Value::Bool(false),
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
        Self::OptionalSelect {
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

    pub fn mode(&self) -> SettingMode {
        match self {
            Self::Toggle { .. } => SettingMode::ReadWrite,
            Self::I32Range { .. } => SettingMode::ReadWrite,
            Self::Select { .. } => SettingMode::ReadWrite,
            Self::OptionalSelect { .. } => SettingMode::ReadWrite,
            Self::ModifiableSelect { .. } => SettingMode::ReadWrite,
            Self::MultiSelect { .. } => SettingMode::ReadWrite,
            Self::Equalizer { .. } => SettingMode::ReadWrite,
            Self::Information { .. } => SettingMode::ReadOnly,
            Self::ImportString { .. } => SettingMode::WriteOnly,
            Self::Action { .. } => SettingMode::WriteOnly,
        }
    }
}

pub enum SettingMode {
    ReadWrite,
    ReadOnly,
    WriteOnly,
}

impl SettingMode {
    pub fn is_writable(&self) -> bool {
        match self {
            Self::ReadWrite => true,
            Self::WriteOnly => true,
            Self::ReadOnly => false,
        }
    }

    pub fn is_readable(&self) -> bool {
        match self {
            Self::ReadWrite => true,
            Self::ReadOnly => true,
            Self::WriteOnly => false,
        }
    }
}

pub fn localize_value(setting: Option<&Setting>, value: &Value) -> String {
    match setting {
        Some(
            Setting::Select { setting, .. }
            | Setting::OptionalSelect { setting, .. }
            | Setting::ModifiableSelect { setting, .. },
        ) => match value.try_as_optional_str() {
            Ok(Some(selection)) => setting
                .options
                .iter()
                .position(|option| option == selection)
                .and_then(|index| setting.localized_options.get(index))
                .cloned()
                .unwrap_or_else(|| fl!("none")),
            Ok(None) => fl!("none"),
            Err(_) => value.to_string(),
        },
        _ => value.to_string(),
    }
}

#[derive(thiserror::Error, Debug)]
#[error("setting id {setting_id}")]
pub struct Error {
    pub setting_id: SettingId,
    pub source: Box<dyn std::error::Error + Send + Sync>,
}
