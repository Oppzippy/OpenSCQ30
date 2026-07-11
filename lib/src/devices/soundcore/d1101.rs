use std::collections::HashMap;

use crate::{
    devices::soundcore::{
        common::{
            self,
            macros::soundcore_device,
            modules::{
                button_configuration::{
                    ButtonAction, ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
                    COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                },
                dual_battery_level::DualBatteryLevelConfiguration,
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
        d1101::{packets::inbound::D1101StateUpdatePacket, state::D1101State},
    },
    i18n::fl,
};

mod packets;
mod state;

soundcore_device!(
    D1101State,
    async |packet_io| {
        let state_update_packet: packets::inbound::D1101StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(state::D1101State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.equalizer(equalizer::common_settings()).await;

        builder.disable_all_buttons();
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);

        builder.dual_connections();

        builder.low_battery_prompt();
        builder.ldac();
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::half_hour_increments(),
        );

        builder.tws_status();
        builder.dual_battery_level_custom(DualBatteryLevelConfiguration {
            max_level: 10,
            level_offset: 1,
        });
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            D1101StateUpdatePacket::default().to_packet(),
        )])
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<8, 4> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: false,
        order: [
            Button::LeftSinglePress,
            Button::RightSinglePress,
            Button::LeftDoublePress,
            Button::RightDoublePress,
            Button::LeftTriplePress,
            Button::RightTriplePress,
            Button::LeftLongPress,
            Button::RightLongPress,
        ],
        settings: [
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: MULTI_PRESS_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 5,
                press_kind: ButtonPressKind::Triple,
                available_actions: MULTI_PRESS_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: LONG_PRESS_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
        ],
    };

pub const MULTI_PRESS_ACTIONS: &[ButtonAction] = &[
    ButtonAction {
        id: 2,
        name: "PreviousSong",
        localized_name: || fl!("previous-song"),
    },
    ButtonAction {
        id: 3,
        name: "NextSong",
        localized_name: || fl!("next-song"),
    },
    ButtonAction {
        id: 5,
        name: "VoiceAssistant",
        localized_name: || fl!("voice-assistant"),
    },
    ButtonAction {
        id: 6,
        name: "PlayPause",
        localized_name: || fl!("play-pause"),
    },
];

pub const LONG_PRESS_ACTIONS: &[ButtonAction] = &[
    ButtonAction {
        id: 0,
        name: "VolumeUp",
        localized_name: || fl!("volume-up"),
    },
    ButtonAction {
        id: 1,
        name: "VolumeDown",
        localized_name: || fl!("volume-down"),
    },
    ButtonAction {
        id: 5,
        name: "VoiceAssistant",
        localized_name: || fl!("voice-assistant"),
    },
];

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        devices::soundcore::common::{
            device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
            packet,
        },
        settings::{SettingId, Value},
    };

    #[tokio::test(start_paused = true)]
    async fn settings_match_soundcore_app() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1101,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 1, 9, 0, 48, 49, 46, 50, 56, 48, 49, 46, 50, 56, 49, 49, 48, 49, 70, 54,
                        56, 69, 69, 68, 67, 57, 48, 57, 51, 52, 0xfe, 0xfe, 160, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 9, 0x66, 0xf6, 0x32, 0xf3, 0xff, 0xff, 0xf1,
                        0xf0, 0x31, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::ButtonsEnabled, true.into()),
            (SettingId::BatteryLevelLeft, "10/10".into()),
            (SettingId::BatteryLevelRight, "1/10".into()),
            (SettingId::LeftSinglePress, Some("PlayPause").into()),
            (SettingId::LeftDoublePress, Some("PreviousSong").into()),
            (
                SettingId::LeftTriplePress,
                Value::OptionalString(None).into(),
            ),
            (SettingId::LeftLongPress, Some("VolumeDown").into()),
            (SettingId::RightSinglePress, Some("PlayPause").into()),
            (SettingId::RightDoublePress, Some("NextSong").into()),
            (
                SettingId::RightTriplePress,
                Value::OptionalString(None).into(),
            ),
            (SettingId::RightLongPress, Some("VolumeUp").into()),
            (SettingId::DualConnections, true.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::Ldac, false.into()),
            (SettingId::AutoPowerOff, "30m".into()),
            (SettingId::FirmwareVersionLeft, "01.28".into()),
            (SettingId::FirmwareVersionRight, "01.28".into()),
            (SettingId::SerialNumber, "1101F68EEDC90934".into()),
            (SettingId::TwsStatus, "Connected".into()),
            (SettingId::HostDevice, "Left".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn buttons_enabled_is_inverted() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1101,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 1, 9, 0, 48, 49, 46, 50, 56, 48, 49, 46, 50, 56, 49, 49, 48, 49, 70, 54,
                        56, 69, 69, 68, 67, 57, 48, 57, 51, 52, 0xfe, 0xfe, 160, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 9, 0x66, 0xf6, 0x32, 0xf3, 0xff, 0xff, 0xf1,
                        0xf0, 0x31, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([(SettingId::ButtonsEnabled, true.into())]);
        device
            .assert_set_settings_response(
                vec![(SettingId::ButtonsEnabled, false.into())],
                vec![packet::Outbound::new(packet::Command([16, 148]), vec![1])],
            )
            .await;
        device.assert_setting_values([(SettingId::ButtonsEnabled, false.into())]);
        device
            .assert_set_settings_response(
                vec![(SettingId::ButtonsEnabled, true.into())],
                vec![packet::Outbound::new(packet::Command([16, 148]), vec![0])],
            )
            .await;
        device.assert_setting_values([(SettingId::ButtonsEnabled, true.into())]);
    }
}
