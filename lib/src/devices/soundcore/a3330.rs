use std::collections::HashMap;

use crate::{
    devices::soundcore::{
        a3330::{packets::inbound::A3330StateUpdatePacket, state::A3330State},
        common::{
            self,
            macros::soundcore_device,
            modules::{
                button_configuration::{
                    ButtonAction, ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
                    COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                },
                dual_battery_level::DualBatteryLevelConfiguration,
                equalizer::{EqualizerModuleSettings, EqualizerPreset},
            },
            packet::{
                self,
                inbound::TryToPacket,
                outbound::{RequestState, ToPacket},
            },
            structures::{
                VolumeAdjustments,
                button_configuration::{
                    ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
                },
            },
        },
    },
    i18n::fl,
    settings::SettingId,
};

mod packets;
mod state;

soundcore_device!(
    A3330State,
    async |packet_io| {
        let state_update_packet: packets::inbound::A3330StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(state::A3330State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.equalizer_with_drc(equalizer_settings()).await;

        builder.disable_all_buttons();
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.button_configuration(&CALL_BUTTON_CONFIGURATION_SETTINGS);
        builder.reset_button_configuration::<packets::inbound::A3330StateUpdatePacket>(
            RequestState.to_packet(),
        );

        builder.dual_connections();

        builder.touch_tone();
        builder.low_battery_prompt();

        builder.tws_status();
        builder.dual_battery_level_custom(DualBatteryLevelConfiguration {
            max_level: 5,
            level_offset: 0,
        });
        builder.case_battery_level(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3330StateUpdatePacket::default().to_packet(),
        )])
    },
);

fn equalizer_settings() -> EqualizerModuleSettings<8, 10, -120, 134, 1> {
    common::modules::equalizer::common_settings_with_presets(vec![EqualizerPreset {
        name: "SoundcoreSignature",
        localized_name: || fl!("soundcore-signature"),
        id: 0,
        volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0, 0, -120]),
    }])
}

const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<6, 3> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: true,
        set_button_action_command_override: None,
        setting_id_override: None,
        order: [
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
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 5,
                press_kind: ButtonPressKind::Triple,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
        ],
    };

const CALL_BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<4, 2> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: true,
        set_button_action_command_override: Some(packet::Command([4, 135])),
        setting_id_override: Some([
            SettingId::LeftDoublePressDuringCall,
            SettingId::RightDoublePressDuringCall,
            SettingId::LeftLongPressDuringCall,
            SettingId::RightLongPressDuringCall,
        ]),
        order: [
            Button::LeftDoublePress,
            Button::RightDoublePress,
            Button::LeftLongPress,
            Button::RightLongPress,
        ],
        settings: [
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: DURING_CALL_BUTTON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: DURING_CALL_BUTTON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
        ],
    };

pub const DURING_CALL_BUTTON_ACTIONS: &[ButtonAction] = &[
    ButtonAction {
        id: 0,
        name: "DeclineCall",
        localized_name: || fl!("decline-call"),
    },
    ButtonAction {
        id: 1,
        name: "AnswerCall",
        localized_name: || fl!("answer-call"),
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
            DeviceModel::SoundcoreA3330,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 5, 5, 48, 52, 46, 50, 55, 48, 52, 46, 50, 55, 51, 51, 51, 48, 57, 56,
                        52, 55, 52, 52, 55, 57, 102, 54, 50, 57, 48, 46, 49, 46, 51, 2, 7, 0x61,
                        0x60, 0x33, 0x33, 0xF6, 0xF6, 0x31, 0, 1, 1, 0, 0, 0, 254, 254, 131, 135,
                        136, 131, 124, 120, 120, 120, 120, 120, 4, 1, 1, 0, 0, 255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "5/5".into()),
            (SettingId::BatteryLevelRight, "5/5".into()),
            (SettingId::CaseBatteryLevel, "2/5".into()),
            (SettingId::LeftDoublePress, Some("VolumeDown").into()),
            (SettingId::LeftTriplePress, Some("NextSong").into()),
            (SettingId::LeftLongPress, Some("PlayPause").into()),
            (
                SettingId::LeftDoublePressDuringCall,
                Some("AnswerCall").into(),
            ),
            (
                SettingId::LeftLongPressDuringCall,
                Some("DeclineCall").into(),
            ),
            (SettingId::RightDoublePress, Some("VolumeUp").into()),
            (SettingId::RightTriplePress, Some("NextSong").into()),
            (SettingId::RightLongPress, Some("PlayPause").into()),
            (
                SettingId::RightDoublePressDuringCall,
                Some("AnswerCall").into(),
            ),
            (
                SettingId::RightLongPressDuringCall,
                Some("DeclineCall").into(),
            ),
            (SettingId::ButtonsEnabled, true.into()),
            (SettingId::DualConnections, false.into()),
            (SettingId::TouchTone, true.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::FirmwareVersionLeft, "04.27".into()),
            (SettingId::FirmwareVersionRight, "04.27".into()),
            (SettingId::SerialNumber, "333098474479f629".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_button_action_during_call() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3330,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 5, 5, 48, 52, 46, 50, 55, 48, 52, 46, 50, 55, 51, 51, 51, 48, 57, 56,
                        52, 55, 52, 52, 55, 57, 102, 54, 50, 57, 48, 46, 49, 46, 51, 2, 7, 0x61,
                        0x60, 0x33, 0x33, 0xF6, 0xF6, 0x31, 0, 1, 1, 0, 0, 0, 254, 254, 131, 135,
                        136, 131, 124, 120, 120, 120, 120, 120, 4, 1, 1, 0, 0, 255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device
            .assert_set_settings_response_unordered(
                vec![
                    (
                        SettingId::LeftLongPressDuringCall,
                        Value::OptionalString(None),
                    ),
                    (SettingId::RightDoublePress, Some("NextSong").into()),
                ],
                vec![
                    // left long press during call
                    packet::Outbound::new(packet::Command([4, 135]), vec![0, 1, 0x0f]),
                    // right double press
                    packet::Outbound::new(packet::Command([4, 129]), vec![1, 0, 0x63]),
                ],
            )
            .await;
    }
}
