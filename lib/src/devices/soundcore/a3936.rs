use std::collections::HashMap;

use crate::devices::soundcore::{
    a3936::{packets::A3936StateUpdatePacket, state::A3936State},
    common::{
        self,
        macros::soundcore_device,
        modules::{
            button_configuration::{
                ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
                COMMON_ACTIONS_MINIMAL, COMMON_ACTIONS_WITH_GAME_MODE,
            },
            equalizer,
        },
        packet::{
            inbound::TryToPacket,
            outbound::{RequestState, ToPacket},
        },
        structures::button_configuration::{
            ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
        },
    },
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3936State,
    async |packet_io| {
        let state_update_packet: A3936StateUpdatePacket = packet_io
            .send_with_response(&RequestState::default().to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(A3936State::new(
            state_update_packet,
            dual_connections_devices,
        ))
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
        builder.dual_connections();
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::half_hour_increments(),
        );
        builder.ldac();
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

#[cfg(test)]
mod tests {
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

    use super::*;

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
            HashMap::from([
                (packet::inbound::STATE_COMMAND, state_update_packet),
                TestSoundcoreDevice::basic_dual_connections_response(),
            ]),
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
            (
                SettingId::FirmwareVersionLeft,
                Value::String("04.19".into()),
            ),
            (
                SettingId::FirmwareVersionRight,
                Value::String("04.19".into()),
            ),
            (
                SettingId::SerialNumber,
                Value::String("3936a477583dda9e".into()),
            ),
            (SettingId::DualConnections, Value::Bool(true)),
        ]);
    }
}
