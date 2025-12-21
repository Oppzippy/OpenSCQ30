use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3936::{packets::A3936StateUpdatePacket, state::A3936State},
        common::{
            device::fetch_state_from_state_update_packet,
            macros::soundcore_device,
            modules::{
                button_configuration::{
                    ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
                    COMMON_ACTIONS_MINIMAL, COMMON_ACTIONS_WITH_GAME_MODE,
                },
                equalizer,
            },
            packet::outbound::{RequestState, ToPacket},
            structures::button_configuration::{
                ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
            },
        },
    },
    i18n::fl,
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3936State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3936State, A3936StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3936_sound_modes();
        builder
            .equalizer_with_custom_hear_id_tws(equalizer::common_settings())
            .await;
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.ambient_sound_mode_cycle();
        builder.reset_button_configuration::<A3936StateUpdatePacket>(
            RequestState::default().to_packet(),
        );
        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);
        builder.touch_tone();
        builder.gaming_mode();
        builder.tws_status();
        builder.dual_battery(5);
        builder.case_battery_level(10);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3936StateUpdatePacket::default().to_packet(),
        )])
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<6, 3> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: false,
        order: [
            Button::LeftSinglePress,
            Button::RightSinglePress,
            Button::LeftDoublePress,
            Button::RightDoublePress,
            Button::LeftLongPress,
            Button::RightLongPress,
        ],
        settings: [
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_ACTIONS_MINIMAL,
                disable_mode: ButtonDisableMode::DisablingOneSideDisablesOther,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: COMMON_ACTIONS_WITH_GAME_MODE,
                disable_mode: ButtonDisableMode::DisablingOneSideDisablesOther,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_ACTIONS_WITH_GAME_MODE,
                disable_mode: ButtonDisableMode::DisablingOneSideDisablesOther,
            },
        ],
    };

#[derive(IntoStaticStr, VariantArray)]
#[allow(clippy::enum_variant_names)]
enum AutoPowerOffDuration {
    #[strum(serialize = "30m")]
    ThirtyMinutes,
    #[strum(serialize = "60m")]
    SixtyMinutes,
    #[strum(serialize = "90m")]
    NinetyMinutes,
    #[strum(serialize = "120m")]
    OneHundredTwentyMinutes,
}

impl Translate for AutoPowerOffDuration {
    fn translate(&self) -> String {
        match self {
            Self::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            Self::SixtyMinutes => fl!("x-minutes", minutes = 60),
            Self::NinetyMinutes => fl!("x-minutes", minutes = 90),
            Self::OneHundredTwentyMinutes => fl!("x-minutes", minutes = 120),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        api::settings::SettingId,
        devices::{
            DeviceModel,
            soundcore::common::{
                device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
                packet,
            },
        },
        settings::Value,
    };

    #[tokio::test(start_paused = true)]
    async fn it_parses_settings_correctly() {
        let state_update_packet = packet::Inbound::new(
            packet::inbound::STATE_COMMAND,
            vec![
                1, 1, 5, 3, 1, 1, 48, 52, 46, 49, 57, 48, 52, 46, 49, 57, 51, 57, 51, 54, 97, 52,
                55, 55, 53, 56, 51, 100, 100, 97, 57, 101, 0, 0, 120, 120, 120, 120, 120, 120, 120,
                120, 120, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 255, 0, 255, 255, 255,
                255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0,
                0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 0, 0, 0, 14, 0, 17, 0, 0, 17, 99, 17, 102, 17, 68, 17, 73,
                7, 2, 0x30, 0, 1, 0, 0, 0, 8, 49, 0, 1, 1, 0, 0, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255,
            ],
        );

        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3936,
            HashMap::from([(packet::inbound::STATE_COMMAND, state_update_packet)]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values(vec![
            (SettingId::AmbientSoundMode, Value::String("Normal".into())),
            (
                SettingId::NoiseCancelingMode,
                Value::String("Adaptive".into()),
            ),
            (
                SettingId::TransparencyMode,
                Value::String("FullyTransparent".into()),
            ),
            (SettingId::LeftSinglePress, Value::OptionalString(None)),
            (SettingId::RightSinglePress, Value::OptionalString(None)),
            (
                SettingId::LeftDoublePress,
                Value::OptionalString(Some("NextSong".into())),
            ),
            (
                SettingId::RightDoublePress,
                Value::OptionalString(Some("PlayPause".into())),
            ),
            (
                SettingId::LeftLongPress,
                Value::OptionalString(Some("AmbientSoundMode".into())),
            ),
            (
                SettingId::RightLongPress,
                Value::OptionalString(Some("GamingMode".into())),
            ),
            (
                SettingId::PresetEqualizerProfile,
                Value::OptionalString(Some("SoundcoreSignature".into())),
            ),
            (SettingId::TouchTone, Value::Bool(false)),
            (SettingId::WindNoiseSuppression, Value::Bool(false)),
            (SettingId::GamingMode, Value::Bool(false)),
            (SettingId::AutoPowerOff, Value::String("30m".into())),
        ]);
    }
}
