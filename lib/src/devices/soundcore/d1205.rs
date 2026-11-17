use std::collections::HashMap;

use crate::devices::soundcore::{
    common::{
        self,
        macros::soundcore_device,
        modules::{
            button_configuration::{
                ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings, COMMON_ACTIONS,
            },
            case_battery_level::CaseBatteryLevelConfiguration,
            dual_battery::DualBatteryConfiguration,
        },
        packet::{
            inbound::TryToPacket,
            outbound::{RequestState, ToPacket},
        },
        structures::button_configuration::{
            ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
        },
    },
    d1205::{packets::inbound::D1205StateUpdatePacket, state::D1205State},
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    D1205State,
    async |packet_io| {
        let state_update_packet: packets::inbound::D1205StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(state::D1205State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.d1205_sound_modes();
        builder.d1205_equalizer().await;

        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);

        builder.dual_connections();
        builder.ldac();
        builder.d1205_spatial_audio();
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::ten_twenty_thirty_sixty(),
        );
        builder.touch_tone();
        builder.wearing_tone();
        builder.low_battery_prompt();
        builder.wearing_detection();
        builder.gaming_mode();

        builder.tws_status();
        builder.dual_battery_custom(DualBatteryConfiguration {
            max_level: 10,
            level_offset: 1,
        });
        builder.case_battery_level_custom(CaseBatteryLevelConfiguration {
            max_level: 10,
            level_offset: 1,
        });
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            D1205StateUpdatePacket::default().to_packet(),
        )])
    },
);

/// Same button ids and order as the D1202, but single presses accept all common actions.
pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<8, 4> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: false,
        set_button_action_command_override: None,
        setting_id_override: None,
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
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 5,
                press_kind: ButtonPressKind::Triple,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_ACTIONS,
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
        settings::{SettingId, Value},
    };

    /// Captured from a soundcore P42i on firmware 06.72, serial number replaced with zeros
    fn state_update_packet() -> packet::Inbound {
        packet::Inbound::new(
            packet::Command([1, 1]),
            vec![
                0, 1, 9, 9, 0, 0, 48, 54, 46, 55, 50, 48, 54, 46, 55, 50, 49, 50, 48, 53, 48, 48,
                48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 54, 46, 55, 50, 6, 0, 0, 205, 204, 204,
                61, 205, 204, 204, 61, 102, 102, 166, 63, 102, 102, 102, 63, 102, 102, 102, 63, 0,
                0, 96, 64, 154, 153, 57, 64, 154, 153, 57, 64, 0, 0, 0, 0, 0, 0, 3, 1, 0, 0, 64,
                64, 0, 0, 240, 65, 154, 153, 153, 62, 1, 0, 0, 128, 63, 0, 0, 72, 66, 0, 0, 128,
                63, 2, 0, 0, 192, 63, 0, 96, 234, 69, 51, 51, 51, 63, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 10, 246, 246, 97, 99, 240,
                242, 68, 68, 99, 0, 95, 0, 0, 0, 0, 255, 0, 49, 1, 1, 0, 1, 1, 3, 1, 90, 0, 0, 0,
                1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 255, 255, 255, 255,
            ],
        )
    }

    /// Posted in GH-336, custom equalizer profile active
    fn state_update_packet_custom_equalizer() -> packet::Inbound {
        packet::Inbound::new(
            packet::Command([1, 1]),
            vec![
                1, 1, 9, 9, 0, 0, 48, 54, 46, 55, 50, 48, 54, 46, 55, 50, 49, 50, 48, 53, 48, 48,
                55, 70, 49, 68, 53, 48, 48, 54, 51, 54, 48, 54, 46, 55, 50, 7, 254, 254, 0, 0, 192,
                64, 0, 0, 144, 64, 51, 51, 83, 64, 102, 102, 230, 63, 154, 153, 153, 62, 154, 153,
                217, 63, 154, 153, 89, 64, 205, 204, 172, 64, 0, 1, 106, 113, 188, 20, 8, 1, 0, 0,
                192, 64, 0, 0, 200, 66, 51, 51, 51, 63, 1, 0, 0, 144, 64, 0, 0, 72, 67, 51, 51, 51,
                63, 1, 51, 51, 83, 64, 0, 0, 200, 67, 51, 51, 51, 63, 1, 102, 102, 230, 63, 0, 0,
                72, 68, 51, 51, 51, 63, 1, 154, 153, 153, 62, 0, 0, 200, 68, 51, 51, 51, 63, 1,
                154, 153, 217, 63, 0, 0, 72, 69, 51, 51, 51, 63, 1, 154, 153, 89, 64, 0, 0, 200,
                69, 51, 51, 51, 63, 1, 205, 204, 172, 64, 0, 0, 72, 70, 51, 51, 51, 63, 10, 241,
                240, 102, 102, 244, 244, 66, 67, 103, 2, 95, 1, 1, 0, 0, 255, 0, 49, 1, 1, 0, 0, 1,
                0, 0, 90, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 255, 255, 255, 255,
            ],
        )
    }

    async fn device(state_update_packet: packet::Inbound) -> TestSoundcoreDevice {
        TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1205,
            HashMap::from([(packet::Command([1, 1]), state_update_packet)]),
            SoundcoreDeviceConfig::default(),
        )
        .await
    }

    #[tokio::test(start_paused = true)]
    async fn settings_match_soundcore_app() {
        let device = device(state_update_packet()).await;
        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "10/10".into()),
            (SettingId::BatteryLevelRight, "10/10".into()),
            (SettingId::CaseBatteryLevel, "7/10".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::WindNoiseSuppression, false.into()),
            (
                SettingId::PresetEqualizerProfile,
                Some("SoundcoreSignature").into(),
            ),
            (SettingId::LeftSinglePress, Some("PlayPause").into()),
            (SettingId::RightSinglePress, Some("PlayPause").into()),
            (SettingId::LeftDoublePress, Some("VolumeDown").into()),
            (SettingId::RightDoublePress, Some("NextSong").into()),
            (SettingId::LeftTriplePress, Some("VolumeUp").into()),
            (SettingId::RightTriplePress, Some("PreviousSong").into()),
            (SettingId::LeftLongPress, Some("AmbientSoundMode").into()),
            (SettingId::RightLongPress, Some("AmbientSoundMode").into()),
            (SettingId::TouchTone, true.into()),
            (SettingId::WearingTone, true.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::WearingDetection, true.into()),
            (SettingId::GamingMode, false.into()),
            (SettingId::SpatialAudio, false.into()),
            (SettingId::Ldac, false.into()),
            (SettingId::DualConnections, true.into()),
            (SettingId::AutoPowerOff, "60m".into()),
            (SettingId::FirmwareVersionLeft, "06.72".into()),
            (SettingId::FirmwareVersionRight, "06.72".into()),
            (SettingId::SerialNumber, "1205000000000000".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn parses_custom_equalizer() {
        let device = device(state_update_packet_custom_equalizer()).await;
        device.assert_setting_values([
            (
                SettingId::PresetEqualizerProfile,
                Value::OptionalString(None),
            ),
            (
                SettingId::VolumeAdjustments,
                Value::I16Vec(vec![60, 45, 33, 18, 3, 17, 34, 54]),
            ),
            (SettingId::CaseBatteryLevel, "8/10".into()),
            (SettingId::AmbientSoundMode, "Normal".into()),
            (SettingId::LeftSinglePress, Some("VolumeDown").into()),
            (SettingId::RightSinglePress, Some("VolumeUp").into()),
            (SettingId::AutoPowerOff, "10m".into()),
            (SettingId::DualConnections, false.into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_ambient_sound_mode_adaptive() {
        let mut device = device(state_update_packet()).await;
        device
            .assert_set_settings_response(
                vec![(SettingId::AmbientSoundMode, "AdaptiveNoiseCanceling".into())],
                vec![packet::Outbound::new(
                    packet::Command([0x06, 0x81]),
                    vec![0x03, 0x5f, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_wind_noise_suppression() {
        let mut device = device(state_update_packet()).await;
        device
            .assert_set_settings_response(
                vec![(SettingId::WindNoiseSuppression, true.into())],
                vec![packet::Outbound::new(
                    packet::Command([0x06, 0x81]),
                    vec![0x00, 0x5f, 0x00, 0x00, 0x01, 0x00, 0xff, 0x00],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_preset_matches_soundcore_app() {
        let mut device = device(state_update_packet()).await;
        device
            .assert_set_settings_response(
                vec![(SettingId::PresetEqualizerProfile, "PowerfulBass".into())],
                vec![packet::Outbound::new(
                    packet::Command([0x03, 0x8a]),
                    vec![
                        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, //
                        0x01, 0x00, 0x00, 0xc0, 0x40, 0x00, 0x00, 0xa0, 0x42, 0x00, 0x00, 0x00,
                        0x3f, 0x01, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x48, 0x43, 0x00, 0x00,
                        0x20, 0x40, 0x01, 0x00, 0x00, 0xc0, 0xbf, 0x00, 0x80, 0x02, 0x44, 0x66,
                        0x66, 0xa6, 0x3f, 0x01, 0x00, 0x00, 0xc0, 0x3f, 0x00, 0xc0, 0x73, 0x45,
                        0x00, 0x00, 0x00, 0x3f,
                    ],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_custom_equalizer_matches_soundcore_app() {
        let mut device = device(state_update_packet()).await;
        device
            .assert_set_settings_response(
                vec![(
                    SettingId::VolumeAdjustments,
                    Value::I16Vec(vec![1, 1, 13, 9, 9, 35, 29, 29]),
                )],
                vec![packet::Outbound::new(
                    packet::Command([0x03, 0x8a]),
                    vec![
                        0xfe, 0xfe, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, //
                        0x01, 0xcd, 0xcc, 0xcc, 0x3d, 0x00, 0x00, 0xc8, 0x42, 0x33, 0x33, 0x33,
                        0x3f, 0x01, 0xcd, 0xcc, 0xcc, 0x3d, 0x00, 0x00, 0x48, 0x43, 0x33, 0x33,
                        0x33, 0x3f, 0x01, 0x66, 0x66, 0xa6, 0x3f, 0x00, 0x00, 0xc8, 0x43, 0x33,
                        0x33, 0x33, 0x3f, 0x01, 0x66, 0x66, 0x66, 0x3f, 0x00, 0x00, 0x48, 0x44,
                        0x33, 0x33, 0x33, 0x3f, 0x01, 0x66, 0x66, 0x66, 0x3f, 0x00, 0x00, 0xc8,
                        0x44, 0x33, 0x33, 0x33, 0x3f, 0x01, 0x00, 0x00, 0x60, 0x40, 0x00, 0x00,
                        0x48, 0x45, 0x33, 0x33, 0x33, 0x3f, 0x01, 0x9a, 0x99, 0x39, 0x40, 0x00,
                        0x00, 0xc8, 0x45, 0x33, 0x33, 0x33, 0x3f, 0x01, 0x9a, 0x99, 0x39, 0x40,
                        0x00, 0x00, 0x48, 0x46, 0x33, 0x33, 0x33, 0x3f,
                    ],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_spatial_audio() {
        let mut device = device(state_update_packet()).await;
        device
            .assert_set_settings_response(
                vec![(SettingId::SpatialAudio, true.into())],
                vec![packet::Outbound::new(
                    packet::Command([0x10, 0x81]),
                    vec![1, 0, 0],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_button_action() {
        let mut device = device(state_update_packet()).await;
        device
            .assert_set_settings_response(
                vec![(SettingId::RightSinglePress, Some("VolumeUp").into())],
                vec![packet::Outbound::new(
                    packet::Command([0x04, 0x81]),
                    vec![1, 2, 0xf0],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn state_update_packet_round_trip() {
        use crate::devices::soundcore::common::packet::{inbound::TryToPacket, outbound::ToPacket};

        for packet in [
            state_update_packet(),
            state_update_packet_custom_equalizer(),
        ] {
            let parsed: super::packets::inbound::D1205StateUpdatePacket =
                packet.try_to_packet().unwrap();
            assert_eq!(parsed.to_packet().body, packet.body);
        }
    }
}
