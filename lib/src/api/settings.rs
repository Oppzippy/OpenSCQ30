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
    DualConnections,
    Case,
    Lights,
    Alarms,
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
#[strum(use_phf, serialize_all = "camelCase")]
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
    SinglePress,
    DoublePress,
    LeftSinglePress,
    LeftDoublePress,
    LeftTriplePress,
    LeftLongPress,
    LeftSlideUp,
    LeftSlideDown,
    RightSinglePress,
    RightDoublePress,
    RightTriplePress,
    RightLongPress,
    RightSlideUp,
    RightSlideDown,
    LeftDoublePressDuringCall,
    RightDoublePressDuringCall,
    LeftLongPressDuringCall,
    RightLongPressDuringCall,
    NormalModeInCycle,
    TransparencyModeInCycle,
    NoiseCancelingModeInCycle,
    AdaptiveNoiseCanceling,
    ManualNoiseCanceling,
    ManualTransparency,
    AirplaneMode,
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
    SoundLeakCompensation,
    SurroundSound,
    AutoPlayPause,
    WearingTone,
    TouchLock,
    LowBatteryPrompt,
    WearingDetection,
    Volume,
    VoicePrompt,
    AncPersonalizedToEarCanal,
    ImmersiveExperience,
    PowerOff,
    SideTone,
    DolbyAudio,
    Ldac,
    DualConnections,
    DualConnectionsDevices,
    Atmospheric,
    RemoteCamera,
    FindDevice,
    SpatialAudio,
    SpatialAudioMode,
    SpatialAudioMusicMode,
    CaseLanguage,
    CaseSerialNumber,
    CaseFirmwareVersion,
    AirPressure,
    EasyChat,
    EasyChatWaitTime,
    RealTimeAdaptiveNoiseCanceling,
    VolumeBalance,
    LightsEnabled,
    LightsBrightness,
    LightsColor,
    LightsMode,
    AutoLightsOffMinutes,
    ButtonsEnabled,
    AutoPowerOffPrompt,
    ListeningModePrompt,
    NoiseCanceling,
    IncomingCallsDuringBluetoothMode,
    NoiseCancelingPrompt,
    AutoStopTimer,
    AutoStopTimerDuration,
    AutoSwitchOnceAsleep,
    ListeningMode,
    DefaultListeningMode,
    CreateAlarm,
    // Ideally the variants below would be generated with a macro, but a declarative macro can't be
    // used to generate enum variants, so it would require something more complicated
    // Alarm 1
    #[translate("delete-alarm-n", number = 1)]
    DeleteAlarm1,
    #[translate("alarm-n-enabled", number = 1)]
    Alarm1Enabled,
    #[translate("alarm-n-time", number = 1)]
    Alarm1Time,
    #[translate("alarm-n-repeat", number = 1)]
    Alarm1Repeat,
    #[translate("alarm-n-wake-up-tune", number = 1)]
    Alarm1WakeUpTune,
    #[translate("alarm-n-volume", number = 1)]
    Alarm1Volume,
    #[translate("alarm-n-snooze-duration-minutes", number = 1)]
    Alarm1SnoozeDuration,
    // Alarm 2
    #[translate("delete-alarm-n", number = 2)]
    DeleteAlarm2,
    #[translate("alarm-n-enabled", number = 2)]
    Alarm2Enabled,
    #[translate("alarm-n-time", number = 2)]
    Alarm2Time,
    #[translate("alarm-n-repeat", number = 2)]
    Alarm2Repeat,
    #[translate("alarm-n-wake-up-tune", number = 2)]
    Alarm2WakeUpTune,
    #[translate("alarm-n-volume", number = 2)]
    Alarm2Volume,
    #[translate("alarm-n-snooze-duration-minutes", number = 2)]
    Alarm2SnoozeDuration,
    // Alarm 3
    #[translate("delete-alarm-n", number = 3)]
    DeleteAlarm3,
    #[translate("alarm-n-enabled", number = 3)]
    Alarm3Enabled,
    #[translate("alarm-n-time", number = 3)]
    Alarm3Time,
    #[translate("alarm-n-repeat", number = 3)]
    Alarm3Repeat,
    #[translate("alarm-n-wake-up-tune", number = 3)]
    Alarm3WakeUpTune,
    #[translate("alarm-n-volume", number = 3)]
    Alarm3Volume,
    #[translate("alarm-n-snooze-duration-minutes", number = 3)]
    Alarm3SnoozeDuration,
    // Alarm 4
    #[translate("delete-alarm-n", number = 4)]
    DeleteAlarm4,
    #[translate("alarm-n-enabled", number = 4)]
    Alarm4Enabled,
    #[translate("alarm-n-time", number = 4)]
    Alarm4Time,
    #[translate("alarm-n-repeat", number = 4)]
    Alarm4Repeat,
    #[translate("alarm-n-wake-up-tune", number = 4)]
    Alarm4WakeUpTune,
    #[translate("alarm-n-volume", number = 4)]
    Alarm4Volume,
    #[translate("alarm-n-snooze-duration-minutes", number = 4)]
    Alarm4SnoozeDuration,
    // Alarm 5
    #[translate("delete-alarm-n", number = 5)]
    DeleteAlarm5,
    #[translate("alarm-n-enabled", number = 5)]
    Alarm5Enabled,
    #[translate("alarm-n-time", number = 5)]
    Alarm5Time,
    #[translate("alarm-n-repeat", number = 5)]
    Alarm5Repeat,
    #[translate("alarm-n-wake-up-tune", number = 5)]
    Alarm5WakeUpTune,
    #[translate("alarm-n-volume", number = 5)]
    Alarm5Volume,
    #[translate("alarm-n-snooze-duration-minutes", number = 5)]
    Alarm5SnoozeDuration,
    FindDeviceLeft,
    FindDeviceRight,
    SleepMode,
}

impl SettingId {
    /// Settings that can cause harm if changed by accident, such as ringing an earbud while it is being worn. Frontends
    /// should ask for confirmation before enabling these, and they are excluded from quick presets.
    pub fn requires_confirmation(&self) -> bool {
        matches!(self, Self::FindDeviceLeft | Self::FindDeviceRight)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    MultiSelectWithRemove {
        setting: Select,
        values: Vec<Cow<'static, str>>,
    },
    Equalizer {
        setting: Equalizer,
        read_only: bool,
        value: Vec<i16>,
    },
    PresetEqualizerProfileSelect {
        equalizer: Equalizer,
        select: Select,
        /// Each index corresponds to the same index in select.options
        presets: Vec<Vec<i16>>,
        value: Option<Cow<'static, str>>,
    },
    Information {
        value: String,
        translated_value: String,
    },
    ImportString {
        confirmation_message: Option<String>,
    },
    HueColorPicker {
        /// in degrees, 0 to 360
        hue: f32,
    },
    Action,
    TimeOfDay {
        minutes_after_midnight: i32,
    },
}

impl From<Setting> for Value {
    fn from(setting: Setting) -> Self {
        match setting {
            Setting::Toggle { value, .. } => value.into(),
            Setting::I32Range { value, .. } => value.into(),
            Setting::Select { value, .. } => value.into(),
            Setting::OptionalSelect { value, .. } => value.into(),
            Setting::Equalizer { value, .. } => value.into(),
            Setting::PresetEqualizerProfileSelect { value, .. } => value.into(),
            Setting::ModifiableSelect { value, .. } => value.into(),
            Setting::Information {
                value,
                translated_value: _,
            } => Cow::<str>::Owned(value).into(),
            Setting::MultiSelect { values, .. } => values.into(),
            Setting::MultiSelectWithRemove { values, .. } => values.into(),
            Setting::ImportString { .. } => Cow::from("").into(),
            Setting::HueColorPicker { hue } => hue.into(),
            Setting::Action => Self::Bool(false),
            Setting::TimeOfDay {
                minutes_after_midnight: minutes_since_midnight,
            } => Self::I32(minutes_since_midnight),
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
            Self::MultiSelectWithRemove { .. } => SettingMode::ReadWrite,
            Self::Equalizer { read_only, .. } => {
                if *read_only {
                    SettingMode::ReadOnly
                } else {
                    SettingMode::ReadWrite
                }
            }
            Self::PresetEqualizerProfileSelect { .. } => SettingMode::ReadWrite,
            Self::Information { .. } => SettingMode::ReadOnly,
            Self::ImportString { .. } => SettingMode::WriteOnly,
            Self::HueColorPicker { .. } => SettingMode::ReadWrite,
            Self::Action { .. } => SettingMode::WriteOnly,
            Self::TimeOfDay { .. } => SettingMode::ReadWrite,
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
        Some(Setting::Information {
            translated_value, ..
        }) => translated_value.to_owned(),
        _ => value.to_string(),
    }
}

#[derive(thiserror::Error, Debug)]
#[error("setting id {setting_id}")]
pub struct Error {
    pub setting_id: SettingId,
    pub source: Box<dyn std::error::Error + Send + Sync>,
}
