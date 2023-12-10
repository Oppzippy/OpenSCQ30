use openscq30_lib::{
    device_profiles::{
        DeviceProfile as LibDeviceProfile, NoiseCancelingModeType as LibNoiseCancelingModeType,
        SoundModeProfile as LibSoundModeProfile, TransparencyModeType as LibTransparencyModeType,
    },
    devices::standard::{
        state::DeviceState as LibDeviceState,
        structures::{
            AmbientSoundMode as LibAmbientSoundMode, BasicHearId as LibBasicHearId,
            Battery as LibBattery, ButtonAction as LibButtonAction,
            CustomButtonModel as LibCustomButtonModel, CustomHearId as LibCustomHearId,
            CustomNoiseCanceling as LibCustomNoiseCanceling, DualBattery as LibDualBattery,
            EqualizerConfiguration as LibEqualizerConfiguration,
            FirmwareVersion as LibFirmwareVersion, HearId as LibHearId,
            HearIdMusicType as LibHearIdMusicType, HearIdType as LibHearIdType,
            NoTwsButtonAction as LibNoTwsButtonAction, NoiseCancelingMode as LibNoiseCancelingMode,
            PresetEqualizerProfile as LibPresetEqualizerProfile, SingleBattery as LibSingleBattery,
            SoundModes as LibSoundModes, StereoVolumeAdjustments as LibStereoVolumeAdjustments,
            TransparencyMode as LibTransparencyMode, TwsButtonAction as LibTwsButtonAction,
            VolumeAdjustments,
        },
    },
};

use crate::protobuf::*;

impl From<LibDeviceState> for DeviceState {
    fn from(value: LibDeviceState) -> Self {
        Self {
            device_profile: value.device_profile.into(),
            battery: value.battery.into(),
            equalizer_configuration: value.equalizer_configuration.into(),
            sound_modes: value.sound_modes.map(Into::into),
            age_range: value.age_range.map(|age_range| age_range.0.into()),
            gender: value.gender.map(|gender| gender.0.into()),
            hear_id: value.hear_id.map(Into::into),
            firmware_version: value.firmware_version.map(Into::into),
            custom_button_model: value.custom_button_model.map(Into::into),
            serial_number: value
                .serial_number
                .map(|serial_number| serial_number.to_string()),
        }
    }
}

impl From<LibDeviceProfile> for DeviceProfile {
    fn from(value: LibDeviceProfile) -> Self {
        Self {
            sound_mode: value.sound_mode.map(Into::into),
            has_hear_id: value.has_hear_id,
            num_equalizer_channels: value.num_equalizer_channels as u32,
            num_equalizer_bands: value.num_equalizer_bands as u32,
            has_dynamic_range_compression: value.has_dynamic_range_compression,
            has_custom_button_model: value.has_custom_button_model,
            has_wear_detection: value.has_wear_detection,
            has_touch_tone: value.has_touch_tone,
            has_auto_power_off: value.has_auto_power_off,
            dynamic_range_compression_min_firmware_version: value
                .dynamic_range_compression_min_firmware_version
                .map(Into::into),
        }
    }
}

impl From<LibSoundModeProfile> for SoundModeProfile {
    fn from(value: LibSoundModeProfile) -> Self {
        Self {
            noise_canceling_mode_type: NoiseCancelingModeType::from(
                value.noise_canceling_mode_type,
            )
            .into(),
            transparency_mode_type: TransparencyModeType::from(value.transparency_mode_type).into(),
        }
    }
}

impl From<LibNoiseCancelingModeType> for NoiseCancelingModeType {
    fn from(value: LibNoiseCancelingModeType) -> Self {
        match value {
            LibNoiseCancelingModeType::None => Self::NoiseCancelingModeNone,
            LibNoiseCancelingModeType::Basic => Self::NoiseCancelingModeBasic,
            LibNoiseCancelingModeType::Custom => Self::NoiseCancelingModeCustom,
        }
    }
}

impl From<LibTransparencyModeType> for TransparencyModeType {
    fn from(value: LibTransparencyModeType) -> Self {
        match value {
            LibTransparencyModeType::Basic => Self::TransparencyModeBasic,
            LibTransparencyModeType::Custom => Self::TransparencyModeCustom,
        }
    }
}

impl From<LibFirmwareVersion> for FirmwareVersion {
    fn from(value: LibFirmwareVersion) -> Self {
        Self {
            major: value.major().into(),
            minor: value.minor().into(),
        }
    }
}

impl From<LibBattery> for Battery {
    fn from(value: LibBattery) -> Self {
        match value {
            LibBattery::SingleBattery(battery) => Battery {
                battery: Some(battery::Battery::SingleBattery(battery.into())),
            },
            LibBattery::DualBattery(battery) => Battery {
                battery: Some(battery::Battery::DualBattery(battery.into())),
            },
        }
    }
}

impl From<LibSingleBattery> for SingleBattery {
    fn from(value: LibSingleBattery) -> Self {
        Self {
            is_charging: value.is_charging.into(),
            level: value.level.0.into(),
        }
    }
}

impl From<LibDualBattery> for DualBattery {
    fn from(value: LibDualBattery) -> Self {
        Self {
            left: value.left.into(),
            right: value.right.into(),
        }
    }
}

impl From<LibEqualizerConfiguration> for EqualizerConfiguration {
    fn from(value: LibEqualizerConfiguration) -> Self {
        Self {
            preset_profile: value
                .preset_profile()
                .map(|lib_preset_profile| PresetEqualizerProfile::from(lib_preset_profile).into()),
            volume_adjustments: value.volume_adjustments().adjustments().to_vec(),
        }
    }
}

impl From<LibPresetEqualizerProfile> for PresetEqualizerProfile {
    fn from(value: LibPresetEqualizerProfile) -> Self {
        match value {
            LibPresetEqualizerProfile::SoundcoreSignature => Self::SoundcoreSignature,
            LibPresetEqualizerProfile::Acoustic => Self::Acoustic,
            LibPresetEqualizerProfile::BassBooster => Self::BassBooster,
            LibPresetEqualizerProfile::BassReducer => Self::BassReducer,
            LibPresetEqualizerProfile::Classical => Self::Classical,
            LibPresetEqualizerProfile::Podcast => Self::Podcast,
            LibPresetEqualizerProfile::Dance => Self::Dance,
            LibPresetEqualizerProfile::Deep => Self::Deep,
            LibPresetEqualizerProfile::Electronic => Self::Electronic,
            LibPresetEqualizerProfile::Flat => Self::Flat,
            LibPresetEqualizerProfile::HipHop => Self::HipHop,
            LibPresetEqualizerProfile::Jazz => Self::Jazz,
            LibPresetEqualizerProfile::Latin => Self::Latin,
            LibPresetEqualizerProfile::Lounge => Self::Lounge,
            LibPresetEqualizerProfile::Piano => Self::Piano,
            LibPresetEqualizerProfile::Pop => Self::Pop,
            LibPresetEqualizerProfile::RnB => Self::Rnb,
            LibPresetEqualizerProfile::Rock => Self::Rock,
            LibPresetEqualizerProfile::SmallSpeakers => Self::SmallSpeakers,
            LibPresetEqualizerProfile::SpokenWord => Self::SpokenWord,
            LibPresetEqualizerProfile::TrebleBooster => Self::TrebleBooster,
            LibPresetEqualizerProfile::TrebleReducer => Self::TrebleReducer,
        }
    }
}

impl From<EqualizerConfiguration> for LibEqualizerConfiguration {
    fn from(value: EqualizerConfiguration) -> Self {
        match value.preset_profile {
            Some(preset_profile_id) => {
                let preset_profile = PresetEqualizerProfile::try_from(preset_profile_id).unwrap();
                LibEqualizerConfiguration::new_from_preset_profile(preset_profile.into())
            }
            None => LibEqualizerConfiguration::new_custom_profile(
                VolumeAdjustments::new(value.volume_adjustments.into_iter()).unwrap(),
            ),
        }
    }
}

impl From<PresetEqualizerProfile> for LibPresetEqualizerProfile {
    fn from(value: PresetEqualizerProfile) -> Self {
        match value {
            PresetEqualizerProfile::SoundcoreSignature => Self::SoundcoreSignature,
            PresetEqualizerProfile::Acoustic => Self::Acoustic,
            PresetEqualizerProfile::BassBooster => Self::BassBooster,
            PresetEqualizerProfile::BassReducer => Self::BassReducer,
            PresetEqualizerProfile::Classical => Self::Classical,
            PresetEqualizerProfile::Podcast => Self::Podcast,
            PresetEqualizerProfile::Dance => Self::Dance,
            PresetEqualizerProfile::Deep => Self::Deep,
            PresetEqualizerProfile::Electronic => Self::Electronic,
            PresetEqualizerProfile::Flat => Self::Flat,
            PresetEqualizerProfile::HipHop => Self::HipHop,
            PresetEqualizerProfile::Jazz => Self::Jazz,
            PresetEqualizerProfile::Latin => Self::Latin,
            PresetEqualizerProfile::Lounge => Self::Lounge,
            PresetEqualizerProfile::Piano => Self::Piano,
            PresetEqualizerProfile::Pop => Self::Pop,
            PresetEqualizerProfile::Rnb => Self::RnB,
            PresetEqualizerProfile::Rock => Self::Rock,
            PresetEqualizerProfile::SmallSpeakers => Self::SmallSpeakers,
            PresetEqualizerProfile::SpokenWord => Self::SpokenWord,
            PresetEqualizerProfile::TrebleBooster => Self::TrebleBooster,
            PresetEqualizerProfile::TrebleReducer => Self::TrebleReducer,
        }
    }
}

impl From<SoundModes> for LibSoundModes {
    fn from(value: SoundModes) -> Self {
        Self {
            ambient_sound_mode: LibAmbientSoundMode::from(
                AmbientSoundMode::try_from(value.ambient_sound_mode).unwrap(),
            ),
            noise_canceling_mode: LibNoiseCancelingMode::from(
                NoiseCancelingMode::try_from(value.noise_canceling_mode).unwrap(),
            ),
            transparency_mode: LibTransparencyMode::from(
                TransparencyMode::try_from(value.transparency_mode).unwrap(),
            ),
            custom_noise_canceling: LibCustomNoiseCanceling::new(
                value.custom_noise_canceling as u8,
            ),
        }
    }
}

impl From<AmbientSoundMode> for LibAmbientSoundMode {
    fn from(value: AmbientSoundMode) -> Self {
        match value {
            AmbientSoundMode::NoiseCanceling => Self::NoiseCanceling,
            AmbientSoundMode::Transparency => Self::Transparency,
            AmbientSoundMode::Normal => Self::Normal,
        }
    }
}

impl From<NoiseCancelingMode> for LibNoiseCancelingMode {
    fn from(value: NoiseCancelingMode) -> Self {
        match value {
            NoiseCancelingMode::Transport => Self::Transport,
            NoiseCancelingMode::Outdoor => Self::Outdoor,
            NoiseCancelingMode::Indoor => Self::Indoor,
            NoiseCancelingMode::Custom => Self::Custom,
        }
    }
}

impl From<TransparencyMode> for LibTransparencyMode {
    fn from(value: TransparencyMode) -> Self {
        match value {
            TransparencyMode::FullyTransparent => Self::FullyTransparent,
            TransparencyMode::VocalMode => Self::VocalMode,
        }
    }
}

impl From<LibSoundModes> for SoundModes {
    fn from(value: LibSoundModes) -> Self {
        Self {
            ambient_sound_mode: AmbientSoundMode::from(value.ambient_sound_mode).into(),
            noise_canceling_mode: NoiseCancelingMode::from(value.noise_canceling_mode).into(),
            transparency_mode: TransparencyMode::from(value.transparency_mode).into(),
            custom_noise_canceling: value.custom_noise_canceling.value().into(),
        }
    }
}

impl From<LibAmbientSoundMode> for AmbientSoundMode {
    fn from(value: LibAmbientSoundMode) -> Self {
        match value {
            LibAmbientSoundMode::NoiseCanceling => Self::NoiseCanceling,
            LibAmbientSoundMode::Transparency => Self::Transparency,
            LibAmbientSoundMode::Normal => Self::Normal,
        }
    }
}

impl From<LibNoiseCancelingMode> for NoiseCancelingMode {
    fn from(value: LibNoiseCancelingMode) -> Self {
        match value {
            LibNoiseCancelingMode::Transport => Self::Transport,
            LibNoiseCancelingMode::Outdoor => Self::Outdoor,
            LibNoiseCancelingMode::Indoor => Self::Indoor,
            LibNoiseCancelingMode::Custom => Self::Custom,
        }
    }
}

impl From<LibTransparencyMode> for TransparencyMode {
    fn from(value: LibTransparencyMode) -> Self {
        match value {
            LibTransparencyMode::FullyTransparent => Self::FullyTransparent,
            LibTransparencyMode::VocalMode => Self::VocalMode,
        }
    }
}

impl From<LibHearId> for HearId {
    fn from(value: LibHearId) -> Self {
        match value {
            LibHearId::Basic(hear_id) => Self {
                hear_id: Some(hear_id::HearId::Basic(hear_id.into())),
            },
            LibHearId::Custom(hear_id) => Self {
                hear_id: Some(hear_id::HearId::Custom(hear_id.into())),
            },
        }
    }
}

impl From<LibBasicHearId> for BasicHearId {
    fn from(value: LibBasicHearId) -> Self {
        Self {
            is_enabled: value.is_enabled,
            volume_adjustments: value.volume_adjustments.into(),
            time: value.time,
        }
    }
}

impl From<LibCustomHearId> for CustomHearId {
    fn from(value: LibCustomHearId) -> Self {
        Self {
            is_enabled: value.is_enabled,
            volume_adjustments: value.volume_adjustments.into(),
            time: value.time,
            hear_id_type: value.hear_id_type.0.into(),
            hear_id_music_type: value.hear_id_music_type.0.into(),
            custom_volume_adjustments: value.custom_volume_adjustments.map(Into::into),
        }
    }
}

impl From<LibStereoVolumeAdjustments> for StereoVolumeAdjustments {
    fn from(value: LibStereoVolumeAdjustments) -> Self {
        Self {
            left: value.left.adjustments().to_vec(),
            right: value.right.adjustments().to_vec(),
        }
    }
}

impl From<HearId> for LibHearId {
    fn from(value: HearId) -> Self {
        match value.hear_id.unwrap() {
            hear_id::HearId::Basic(hear_id) => Self::Basic(hear_id.into()),
            hear_id::HearId::Custom(hear_id) => Self::Custom(hear_id.into()),
        }
    }
}

impl From<BasicHearId> for LibBasicHearId {
    fn from(value: BasicHearId) -> Self {
        Self {
            is_enabled: value.is_enabled,
            volume_adjustments: value.volume_adjustments.into(),
            time: value.time,
        }
    }
}

impl From<CustomHearId> for LibCustomHearId {
    fn from(value: CustomHearId) -> Self {
        Self {
            is_enabled: value.is_enabled,
            volume_adjustments: value.volume_adjustments.into(),
            time: value.time,
            hear_id_type: LibHearIdType(value.hear_id_type as u8),
            hear_id_music_type: LibHearIdMusicType(value.hear_id_music_type as u8),
            custom_volume_adjustments: value.custom_volume_adjustments.map(Into::into),
        }
    }
}

impl From<StereoVolumeAdjustments> for LibStereoVolumeAdjustments {
    fn from(value: StereoVolumeAdjustments) -> Self {
        Self {
            left: VolumeAdjustments::new(value.left.into_iter()).unwrap(),
            right: VolumeAdjustments::new(value.right.into_iter()).unwrap(),
        }
    }
}

impl From<LibCustomButtonModel> for CustomButtonModel {
    fn from(value: LibCustomButtonModel) -> Self {
        Self {
            left_single_click: value.left_single_click.into(),
            left_double_click: value.left_double_click.into(),
            left_long_press: value.left_long_press.into(),
            right_single_click: value.right_single_click.into(),
            right_double_click: value.right_double_click.into(),
            right_long_press: value.right_long_press.into(),
        }
    }
}

impl From<LibNoTwsButtonAction> for NoTwsButtonAction {
    fn from(value: LibNoTwsButtonAction) -> Self {
        Self {
            is_enabled: value.is_enabled,
            action: ButtonAction::from(value.action).into(),
        }
    }
}

impl From<LibTwsButtonAction> for TwsButtonAction {
    fn from(value: LibTwsButtonAction) -> Self {
        Self {
            is_enabled: value.is_enabled,
            tws_connected_action: ButtonAction::from(value.tws_connected_action).into(),
            tws_disconnected_action: ButtonAction::from(value.tws_disconnected_action).into(),
        }
    }
}

impl From<LibButtonAction> for ButtonAction {
    fn from(value: LibButtonAction) -> Self {
        match value {
            LibButtonAction::VolumeUp => Self::VolumeUp,
            LibButtonAction::VolumeDown => Self::VolumeDown,
            LibButtonAction::PreviousSong => Self::PreviousSong,
            LibButtonAction::NextSong => Self::NextSong,
            LibButtonAction::Trans => Self::Trans,
            LibButtonAction::VoiceAssistant => Self::VoiceAssistant,
            LibButtonAction::PlayPause => Self::PlayPause,
        }
    }
}

impl From<CustomButtonModel> for LibCustomButtonModel {
    fn from(value: CustomButtonModel) -> Self {
        Self {
            left_single_click: value.left_single_click.into(),
            left_double_click: value.left_double_click.into(),
            left_long_press: value.left_long_press.into(),
            right_single_click: value.right_single_click.into(),
            right_double_click: value.right_double_click.into(),
            right_long_press: value.right_long_press.into(),
        }
    }
}

impl From<NoTwsButtonAction> for LibNoTwsButtonAction {
    fn from(value: NoTwsButtonAction) -> Self {
        Self {
            is_enabled: value.is_enabled,
            action: ButtonAction::try_from(value.action).unwrap().into(),
        }
    }
}

impl From<TwsButtonAction> for LibTwsButtonAction {
    fn from(value: TwsButtonAction) -> Self {
        Self {
            is_enabled: value.is_enabled,
            tws_connected_action: ButtonAction::try_from(value.tws_connected_action)
                .unwrap()
                .into(),
            tws_disconnected_action: ButtonAction::try_from(value.tws_disconnected_action)
                .unwrap()
                .into(),
        }
    }
}

impl From<ButtonAction> for LibButtonAction {
    fn from(value: ButtonAction) -> Self {
        match value {
            ButtonAction::VolumeUp => Self::VolumeUp,
            ButtonAction::VolumeDown => Self::VolumeDown,
            ButtonAction::PreviousSong => Self::PreviousSong,
            ButtonAction::NextSong => Self::NextSong,
            ButtonAction::Trans => Self::Trans,
            ButtonAction::VoiceAssistant => Self::VoiceAssistant,
            ButtonAction::PlayPause => Self::PlayPause,
        }
    }
}
