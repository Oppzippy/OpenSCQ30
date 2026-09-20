use std::collections::HashMap;

use crate::devices::soundcore::{
    a3388::{packets::inbound::A3388StateUpdatePacket, state::A3388State},
    common::{
        self,
        macros::soundcore_device,
        modules::{
            button_configuration::{
                ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
                COMMON_ACTIONS_WITHOUT_SOUND_MODES,
            },
            dual_battery_level::DualBatteryLevelConfiguration,
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

mod packets;
mod state;

soundcore_device!(
    A3388State,
    async |packet_io| {
        let state_update_packet: A3388StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(A3388State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder
            .equalizer_with_drc(common::modules::equalizer::common_settings_type_2())
            .await;

        builder.disable_all_buttons();
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.reset_button_configuration::<A3388StateUpdatePacket>(RequestState.to_packet());

        builder.dual_connections();

        builder.touch_tone();
        builder.low_battery_prompt();

        builder.tws_status();
        builder.dual_battery_level_custom(DualBatteryLevelConfiguration {
            max_level: 10,
            level_offset: 0,
        });
        builder.case_battery_level(10);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3388StateUpdatePacket::default().to_packet(),
        )])
    },
);

// The soundcore app offers double and triple press only. Long press is absent from the app and does
// nothing on the device, so its slots are left out.
pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<4, 2> =
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
        ],
    };

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        devices::soundcore::common::{
            device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
            packet,
        },
        settings::SettingId,
    };

    // Read from a real AeroClip on firmware 01.49. The soundcore app showed Podcast as the
    // equalizer preset, and the same button actions.
    fn real_state_update_packet() -> packet::Inbound {
        packet::Inbound::new(
            packet::Command([1, 1]),
            vec![
                0, 1, 8, 8, 48, 49, 46, 52, 57, 48, 49, 46, 52, 57, 51, 51, 56, 56, 56, 56, 48, 69,
                56, 53, 53, 48, 66, 68, 69, 52, 196, 14, 56, 249, 212, 7, 7, 102, 102, 50, 51, 85,
                85, 49, 0, 1, 1, 1, 0, 1, 5, 0, 90, 140, 160, 160, 150, 140, 120, 100, 0, 0, 1, 0,
                0, 255,
            ],
        )
    }

    #[tokio::test(start_paused = true)]
    async fn settings_match_soundcore_app() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3388,
            HashMap::from([
                (packet::Command([1, 1]), real_state_update_packet()),
                (
                    packet::Command([0x0b, 0x01]),
                    packet::Inbound::new(packet::Command([0x0b, 0x01]), vec![0]),
                ),
            ]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "8/10".into()),
            (SettingId::BatteryLevelRight, "8/10".into()),
            (SettingId::CaseBatteryLevel, "7/10".into()),
            (SettingId::TouchTone, true.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::DualConnections, true.into()),
            (SettingId::ButtonsEnabled, true.into()),
            (SettingId::LeftDoublePress, Some("PlayPause").into()),
            (SettingId::RightDoublePress, Some("PlayPause").into()),
            (SettingId::LeftTriplePress, Some("PreviousSong").into()),
            (SettingId::RightTriplePress, Some("NextSong").into()),
            (SettingId::PresetEqualizerProfile, Some("Podcast").into()),
            (SettingId::FirmwareVersionLeft, "01.49".into()),
            (SettingId::FirmwareVersionRight, "01.49".into()),
            (SettingId::SerialNumber, "3388880E8550BDE4".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn packet_from_issue_349_parses() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3388,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 1, 9, 9, 48, 49, 46, 52, 57, 48, 49, 46, 52, 57, 51, 51, 56, 56, 56, 56,
                        48, 69, 56, 53, 50, 69, 50, 67, 53, 67, 184, 253, 116, 252, 48, 5, 7, 6, 6,
                        2, 3, 15, 15, 49, 0, 1, 1, 1, 1, 1, 0, 0, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 1, 0, 0, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "9/10".into()),
            (SettingId::LeftDoublePress, Some("PlayPause").into()),
            (SettingId::RightTriplePress, Some("NextSong").into()),
        ]);
    }
}
