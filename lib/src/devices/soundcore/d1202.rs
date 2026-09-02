use std::collections::HashMap;

use crate::devices::soundcore::{
    common::{
        self,
        macros::soundcore_device,
        modules::{
            button_configuration::{
                ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings, COMMON_ACTIONS,
                COMMON_ACTIONS_MINIMAL,
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
    d1202::{packets::inbound::D1202StateUpdatePacket, state::D1202State},
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    D1202State,
    async |packet_io| {
        let state_update_packet: packets::inbound::D1202StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(state::D1202State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.d1202_sound_modes();

        builder
            .d1202_equalizer(common::modules::equalizer::common_settings_type_2())
            .await;

        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.reset_button_configuration::<packets::inbound::D1202StateUpdatePacket>(
            RequestState.to_packet(),
        );

        builder.limit_high_volume();

        builder.dual_connections();

        builder.d1202_spatial_audio();
        builder.ldac();
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::ten_twenty_thirty_sixty(),
        );
        builder.touch_tone();
        builder.low_battery_prompt();

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
            D1202StateUpdatePacket::default().to_packet(),
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
                available_actions: COMMON_ACTIONS_MINIMAL,
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

    #[tokio::test(start_paused = true)]
    async fn settings_match_soundcore_app() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1202,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 9, 9, 0, 0, 48, 49, 46, 56, 51, 48, 49, 46, 56, 51, 49, 50, 48, 50,
                        51, 52, 48, 57, 67, 57, 66, 52, 48, 51, 67, 69, 48, 49, 46, 56, 51, 3, 0,
                        0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 0, 0, 0, 2,
                        120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 0, 0, 10, 0xff, 0xff, 0x63, 0x66, 0xff, 0xff,
                        0x44, 0x44, 51, 0, 0x50, 0, 2, 0, 0, 0, 0, 50, 1, 1, 0, 1, 1, 2, 0, 90, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "10/10".into()),
            (SettingId::BatteryLevelRight, "10/10".into()),
            (SettingId::CaseBatteryLevel, "4/10".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::AdaptiveNoiseCanceling, "Weak".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::MultiSceneNoiseCanceling, "Transport".into()),
            (SettingId::WindNoiseSuppression, false.into()),
            (SettingId::RealTimeAdaptiveNoiseCanceling, false.into()),
            (
                SettingId::LeftSinglePress,
                Value::OptionalString(None).into(),
            ),
            (SettingId::LeftDoublePress, Some("NextSong").into()),
            (
                SettingId::LeftTriplePress,
                Value::OptionalString(None).into(),
            ),
            (SettingId::LeftLongPress, Some("AmbientSoundMode").into()),
            (
                SettingId::RightSinglePress,
                Value::OptionalString(None).into(),
            ),
            (SettingId::RightDoublePress, Some("PlayPause").into()),
            (
                SettingId::RightTriplePress,
                Value::OptionalString(None).into(),
            ),
            (SettingId::RightLongPress, Some("AmbientSoundMode").into()),
            (SettingId::LimitHighVolume, false.into()),
            (SettingId::LimitHighVolumeDbLimit, 90.into()),
            (SettingId::LimitHighVolumeRefreshRate, "RealTime".into()),
            (SettingId::DualConnections, true.into()),
            (SettingId::TouchTone, true.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::DualConnections, true.into()),
            (SettingId::AutoPowerOff, "30m".into()),
            (SettingId::Ldac, false.into()),
            (SettingId::FirmwareVersionLeft, "01.83".into()),
            (SettingId::FirmwareVersionRight, "01.83".into()),
            (SettingId::SerialNumber, "12023409C9B403CE".into()),
            (SettingId::SpatialAudio, false.into()),
            (SettingId::SpatialAudioMode, "Music".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn enable_spatial_audio() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1202,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 9, 9, 0, 0, 48, 49, 46, 56, 51, 48, 49, 46, 56, 51, 49, 50, 48, 50,
                        51, 52, 48, 57, 67, 57, 66, 52, 48, 51, 67, 69, 48, 49, 46, 56, 51, 3, 0,
                        0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 0, 0, 0, 2,
                        120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 0, 0, 10, 0xff, 0xff, 0x63, 0x66, 0xff, 0xff,
                        0x44, 0x44, 51, 0, 0x50, 0, 2, 0, 0, 0, 0, 50, 1, 1, 0, 1, 1, 2, 0, 90, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(SettingId::SpatialAudio, true.into())],
                vec![packet::Outbound::new(
                    packet::Command([16, 129]),
                    vec![1, 0, 0],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_preset_eq() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1202,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 9, 9, 0, 0, 48, 49, 46, 56, 51, 48, 49, 46, 56, 51, 49, 50, 48, 50,
                        51, 52, 48, 57, 67, 57, 66, 52, 48, 51, 67, 69, 48, 49, 46, 56, 51, 3, 0,
                        0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 0, 0, 0, 2,
                        120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 0, 0, 10, 0xff, 0xff, 0x63, 0x66, 0xff, 0xff,
                        0x44, 0x44, 51, 0, 0x50, 0, 2, 0, 0, 0, 0, 50, 1, 1, 0, 1, 1, 2, 0, 90, 0,
                        1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(SettingId::PresetEqualizerProfile, "Acoustic".into())],
                vec![packet::Outbound::new(
                    packet::Command([3, 135]),
                    vec![
                        1, 0, 0, 0, 160, 130, 140, 140, 160, 160, 160, 140, 120, 0, 160, 130, 140,
                        140, 160, 160, 160, 140, 120, 0, 0, 0, 0, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 0,
                        0, 0, 2, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 120, 160, 0, 130, 0, 140, 0, 140, 0, 160, 0,
                        160, 0, 160, 0, 140, 0, 120, 0, 0, 0, 160, 0, 130, 0, 140, 0, 140, 0, 160,
                        0, 160, 0, 160, 0, 140, 0, 120, 0, 0, 0, 0, 0,
                    ],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn disable_spatial_audio() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1202,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 9, 9, 0, 0, 48, 49, 46, 56, 51, 48, 49, 46, 56, 51, 49, 50, 48, 50,
                        51, 52, 48, 57, 67, 57, 66, 52, 48, 51, 67, 69, 48, 49, 46, 56, 51, 3, 0,
                        0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 0, 0, 0, 2,
                        120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 0, 0, 10, 0xff, 0xff, 0x63, 0x66, 0xff, 0xff,
                        0x44, 0x44, 51, 0, 0x50, 0, 2, 0, 0, 0, 0, 50, 1, 1, 0, 1, 1, 2, 0, 90, 0,
                        1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(SettingId::SpatialAudio, false.into())],
                vec![packet::Outbound::new(
                    packet::Command([3, 135]),
                    vec![
                        0, 0, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 0, 0, 0, 0, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 0,
                        0, 0, 2, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 120, 120, 0, 120, 0, 120, 0, 120, 0, 120, 0,
                        120, 0, 120, 0, 120, 0, 120, 0, 0, 0, 120, 0, 120, 0, 120, 0, 120, 0, 120,
                        0, 120, 0, 120, 0, 120, 0, 120, 0, 0, 0, 0, 0,
                    ],
                )],
            )
            .await;
    }
}
