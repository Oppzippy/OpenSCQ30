use std::collections::HashMap;

use crate::devices::soundcore::common::{
    self,
    macros::soundcore_device,
    modules::button_configuration::{
        ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings, COMMON_ACTIONS,
    },
    packet::{
        inbound::TryToPacket,
        outbound::{RequestState, ToPacket},
    },
    structures::button_configuration::{
        ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
    },
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    state::A3958State,
    async |packet_io| {
        let state_update_packet: packets::inbound::A3958StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(state::A3958State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3958_sound_modes();
        builder
            .equalizer_with_drc_tws(common::modules::equalizer::common_settings_type_2())
            .await;
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.ambient_sound_mode_cycle();
        builder.reset_button_configuration::<packets::inbound::A3958StateUpdatePacket>(
            RequestState.to_packet(),
        );
        builder.dual_connections();
        builder.surround_sound();
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::ten_twenty_thirty_sixty(),
        );
        builder.touch_tone();
        builder.tws_status();
        builder.low_battery_prompt();
        builder.gaming_mode();

        builder.dual_battery_custom(common::modules::dual_battery::DualBatteryConfiguration {
            max_level: 10,
            level_offset: 1,
        });
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            packets::inbound::A3958StateUpdatePacket::default().to_packet(),
        )])
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<8, 4> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: true,
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

    /// Captured from a Soundcore A30i (A3958) running firmware 01.47, with the Soundcore
    /// Signature equalizer preset, noise canceling in manual mode, and wind noise reduction on.
    const DEFAULT_STATE_UPDATE_PACKET: &[u8] = &[
        0, 1, 4, 0, 255, 255, 48, 49, 46, 52, 55, 48, 49, 46, 52, 55, 51, 57, 53, 56, 66, 54, 52,
        56, 54, 67, 53, 66, 49, 56, 50, 48, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 255, 255, 255, 102, 255, 255, 255, 79, 51, 0, 85, 0, 0,
        1, 255, 0, 50, 0, 1, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    async fn test_device() -> TestSoundcoreDevice {
        TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3958,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    DEFAULT_STATE_UPDATE_PACKET.to_vec(),
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await
    }

    #[tokio::test(start_paused = true)]
    async fn packet_from_github_issue_185_parses() {
        let device = test_device().await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "5/10".into()),
            (SettingId::BatteryLevelRight, "1/10".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::NoiseCancelingMode, "Manual".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::WindNoiseSuppression, true.into()),
            (SettingId::MultiSceneNoiseCanceling, "Transport".into()),
            (
                SettingId::PresetEqualizerProfile,
                Some("SoundcoreSignature").into(),
            ),
            (SettingId::LeftSinglePress, Value::OptionalString(None)),
            (SettingId::RightSinglePress, Value::OptionalString(None)),
            (SettingId::LeftDoublePress, Value::OptionalString(None)),
            (SettingId::RightDoublePress, Some("PlayPause").into()),
            (SettingId::LeftTriplePress, Value::OptionalString(None)),
            (SettingId::RightTriplePress, Value::OptionalString(None)),
            (SettingId::LeftLongPress, Value::OptionalString(None)),
            (SettingId::RightLongPress, Value::OptionalString(None)),
            (SettingId::TouchTone, false.into()),
            (SettingId::AutoPowerOff, "30m".into()),
            (SettingId::LowBatteryPrompt, false.into()),
            (SettingId::SurroundSound, false.into()),
            (SettingId::DualConnections, true.into()),
            (SettingId::SerialNumber, "3958B6486C5B1820".into()),
            (SettingId::FirmwareVersionLeft, "01.47".into()),
            (SettingId::FirmwareVersionRight, "01.47".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_manual_noise_canceling() {
        let mut device = test_device().await;

        device
            .assert_set_settings_response(
                vec![(SettingId::ManualNoiseCanceling, 2.into())],
                vec![packet::Outbound::new(
                    packet::Command([6, 129]),
                    vec![0, 37, 0, 0, 1, 255, 0],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_ambient_sound_mode() {
        let mut device = test_device().await;

        device
            .assert_set_settings_response(
                vec![(SettingId::AmbientSoundMode, "Transparency".into())],
                vec![packet::Outbound::new(
                    packet::Command([6, 129]),
                    vec![1, 85, 1, 0, 1, 255, 0],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_multiple_button_actions() {
        let mut device = test_device().await;

        device
            .assert_set_settings_response_unordered(
                vec![
                    (SettingId::LeftSinglePress, "VolumeUp".into()),
                    (SettingId::RightSinglePress, "VolumeDown".into()),
                ],
                vec![
                    packet::Outbound::new(packet::Command([0x04, 0x81]), vec![0, 2, 0xF0]),
                    packet::Outbound::new(packet::Command([0x04, 0x81]), vec![1, 2, 0xF1]),
                ],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn disable_button() {
        let mut device = test_device().await;

        device
            .assert_set_settings_response_unordered(
                vec![(SettingId::RightDoublePress, "None".into())],
                vec![packet::Outbound::new(
                    packet::Command([0x04, 0x81]),
                    vec![1, 0, 0x6F],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn has_no_gaming_mode_on_old_firmware() {
        let device = test_device().await;

        assert_eq!(device.inner().setting(&SettingId::GamingMode), None);
    }
}
