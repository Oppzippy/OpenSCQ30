use std::collections::HashMap;

use crate::{
    devices::soundcore::{
        a3954::{packets::inbound::A3954StateUpdatePacket, state::A3954State},
        common::{
            self,
            macros::soundcore_device,
            modules::{
                button_configuration::{
                    ButtonAction, ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
                    COMMON_ACTIONS,
                },
                case_battery_level::CaseBatteryLevelConfiguration,
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
    },
    i18n::fl,
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3954State,
    async |packet_io| {
        let state_update_packet: packets::inbound::A3954StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(state::A3954State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.a3954_sound_modes();

        builder.a3954_equalizer(equalizer::common_settings()).await;

        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.ambient_sound_mode_cycle();
        builder.reset_button_configuration::<packets::inbound::A3954StateUpdatePacket>(
            RequestState.to_packet(),
        );

        builder.limit_high_volume();

        builder.dual_connections();

        builder.a3954_case_features();
        builder.a3954_case_language();
        builder.ldac();
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::ten_twenty_thirty_sixty(),
        );
        builder.low_battery_prompt();
        builder.a3954_spatial_audio();
        builder.a3954_easy_chat();
        builder.sound_leak_compensation();
        builder.wearing_detection();

        builder.a3954_air_pressure();
        builder.tws_status();
        builder.dual_battery(100);
        builder.case_battery_level_custom(CaseBatteryLevelConfiguration {
            max_level: 10,
            level_offset: 1,
        });
        builder.serial_number_and_dual_firmware_version();
        builder.a3954_case_serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3954StateUpdatePacket::default().to_packet(),
        )])
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<12, 6> =
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
            Button::LeftSlideUp,
            Button::RightSlideUp,
            Button::LeftSlideDown,
            Button::RightSlideDown,
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
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 6,
                press_kind: ButtonPressKind::SlideUp,
                available_actions: SLIDE_UP_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 7,
                press_kind: ButtonPressKind::SlideDown,
                available_actions: SLIDE_DOWN_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
        ],
    };

pub const SLIDE_UP_ACTIONS: &[ButtonAction] = &[ButtonAction {
    id: 0,
    name: "VolumeUp",
    localized_name: || fl!("volume-up"),
}];

pub const SLIDE_DOWN_ACTIONS: &[ButtonAction] = &[ButtonAction {
    id: 1,
    name: "VolumeDown",
    localized_name: || fl!("volume-down"),
}];

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
    async fn parses_known_packet() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3954,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 1, 99, 100, 0, 0, 48, 51, 46, 50, 57, 48, 51, 46, 50, 57, 51, 57, 53,
                        52, 68, 69, 55, 55, 53, 49, 56, 65, 57, 68, 70, 52, 48, 50, 46, 53, 56, 9,
                        0xF4, 0x9D, 0x8A, 0x53, 0x2B, 0xBA, 254, 254, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1,
                        145, 147, 139, 141, 122, 111, 105, 100, 60, 60, 145, 147, 139, 141, 122,
                        111, 105, 100, 60, 60, 0, 0, 0, 0, 1, 145, 147, 139, 130, 122, 133, 133,
                        114, 60, 60, 145, 147, 139, 130, 122, 133, 133, 114, 60, 60, 0, 0, 10,
                        0x66, 0x66, 0x32, 0x33, 0xFF, 0xFF, 0x44, 0x44, 0x33, 2, 6, 0, 0, 255, 1,
                        255, 0, 0, 0, 0, 94, 1, 110, 1, 0, 0, 0, 1, 0, 1, 95, 0, 0, 1, 0, 0, 0, 0,
                        110, 1, 0, 1, 0, 0, 255, 0, 0, 17, 17,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "99/100".into()),
            (SettingId::BatteryLevelRight, "100/100".into()),
            (SettingId::CaseBatteryLevel, "10/10".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::AmbientSoundMode, "Normal".into()),
            (SettingId::AirplaneMode, "ManualUpdate".into()),
            (SettingId::AirPressure, "0.94".into()),
            (SettingId::WindNoiseSuppression, false.into()),
            (SettingId::SpatialAudioMode, "Music".into()),
            (SettingId::SpatialAudioMusicMode, "Fixed".into()),
            (SettingId::LeftSinglePress, Some("PlayPause").into()),
            (SettingId::LeftDoublePress, Some("PreviousSong").into()),
            (
                SettingId::LeftTriplePress,
                Value::OptionalString(None).into(),
            ),
            (SettingId::LeftLongPress, Some("AmbientSoundMode").into()),
            (SettingId::LeftSlideUp, Some("VolumeUp").into()),
            (SettingId::LeftSlideDown, Some("VolumeDown").into()),
            (SettingId::RightSinglePress, Some("PlayPause").into()),
            (SettingId::RightDoublePress, Some("NextSong").into()),
            (SettingId::RightTriplePress, Value::OptionalString(None)),
            (SettingId::RightLongPress, Some("AmbientSoundMode").into()),
            (SettingId::RightSlideUp, Some("VolumeUp").into()),
            (SettingId::RightSlideDown, Some("VolumeDown").into()),
            (SettingId::EasyChat, false.into()),
            (SettingId::EasyChatWaitTime, "5s".into()),
            (SettingId::WearingDetection, false.into()),
            (SettingId::SoundLeakCompensation, false.into()),
            (SettingId::LimitHighVolume, true.into()),
            (SettingId::LimitHighVolumeDbLimit, 95.into()),
            (SettingId::LimitHighVolumeRefreshRate, "RealTime".into()),
            (SettingId::LowBatteryPrompt, false.into()),
            (SettingId::Ldac, false.into()),
            (SettingId::DualConnections, false.into()),
            (SettingId::SpatialAudio, false.into()),
            (SettingId::Atmospheric, false.into()),
            (SettingId::FindDevice, false.into()),
            (SettingId::RemoteCamera, false.into()),
            (SettingId::CaseLanguage, "English".into()),
            (SettingId::AutoPowerOff, "10m".into()),
            (SettingId::FirmwareVersionLeft, "03.29".into()),
            (SettingId::FirmwareVersionRight, "03.29".into()),
            (SettingId::CaseFirmwareVersion, "02.58".into()),
            (SettingId::SerialNumber, "3954DE77518A9DF4".into()),
            (SettingId::CaseSerialNumber, "3954F49D8A532BBA".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_eq_packet() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3954,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 1, 99, 100, 0, 0, 48, 51, 46, 50, 57, 48, 51, 46, 50, 57, 51, 57, 53,
                        52, 68, 69, 55, 55, 53, 49, 56, 65, 57, 68, 70, 52, 48, 50, 46, 53, 56, 9,
                        0xF4, 0x9D, 0x8A, 0x53, 0x2B, 0xBA, 254, 254, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1,
                        145, 147, 139, 141, 122, 111, 105, 100, 60, 60, 145, 147, 139, 141, 122,
                        111, 105, 100, 60, 60, 0, 0, 0, 0, 1, 145, 147, 139, 130, 122, 133, 133,
                        114, 60, 60, 145, 147, 139, 130, 122, 133, 133, 114, 60, 60, 0, 0, 10,
                        0x66, 0x66, 0x32, 0x33, 0xFF, 0xFF, 0x44, 0x44, 0x33, 2, 6, 0, 0, 255, 1,
                        255, 0, 0, 0, 0, 94, 1, 110, 1, 0, 0, 0, 1, 0, 1, 95, 0, 0, 1, 0, 0, 0, 0,
                        110, 1, 0, 1, 0, 0, 255, 0, 0, 17, 17,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        let expected = packet::Outbound::new(
            packet::Command([3, 135]),
            vec![
                254, 254, 0, 0, 180, 120, 120, 120, 120, 120, 120, 60, 120, 120, 180, 120, 120,
                120, 120, 120, 120, 60, 254, 254, // these two 254's should actually be 120s
                0, 0, 0, 145, 147, 139, 141, 122, 111, 105, 100, 60, 60, 145, 147, 139, 141, 122,
                111, 105, 100, 60, 60, 0, 0, 0, 0, 1, 145, 147, 139, 130, 122, 133, 133, 114, 60,
                60, 145, 147, 139, 130, 122, 133, 133, 114, 60, 60, 180, 0, 120, 0, 120, 0, 120, 0,
                120, 0, 120, 0, 120, 0, 60, 0, 120, 0, 120, 0, 180, 0, 120, 0, 120, 0, 120, 0, 120,
                0, 120, 0, 120, 0, 60, 0, 254, 0, 254, 0, 0,
                0, // the last two 254s here should also be 120
            ],
        );

        device
            .assert_set_settings_response(
                vec![(
                    SettingId::VolumeAdjustments,
                    Value::I16Vec(vec![60, 0, 0, 0, 0, 0, 0, -60]),
                )],
                vec![expected],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn changing_eq_disables_spatial_audio() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3954,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 1, 99, 100, 0, 0, 48, 51, 46, 50, 57, 48, 51, 46, 50, 57, 51, 57, 53,
                        52, 68, 69, 55, 55, 53, 49, 56, 65, 57, 68, 70, 52, 48, 50, 46, 53, 56, 9,
                        0xF4, 0x9D, 0x8A, 0x53, 0x2B, 0xBA, 254, 254, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1,
                        145, 147, 139, 141, 122, 111, 105, 100, 60, 60, 145, 147, 139, 141, 122,
                        111, 105, 100, 60, 60, 0, 0, 0, 0, 1, 145, 147, 139, 130, 122, 133, 133,
                        114, 60, 60, 145, 147, 139, 130, 122, 133, 133, 114, 60, 60, 0, 0, 10,
                        0x66, 0x66, 0x32, 0x33, 0xFF, 0xFF, 0x44, 0x44, 0x33, 2, 6, 0, 0, 255, 1,
                        255, 0, 0, 0, 0, 94, 1, 110, 1, 0, 0, 0, 1, 0, 1, 95, 0, 0, 1, 0, 0, 0, 0,
                        110, 1, 0, 1, 0, 0, 255, 0, 0, 17, 17,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device
            .set_settings(vec![(SettingId::SpatialAudio, true.into())])
            .await;
        device.assert_setting_values(vec![(SettingId::SpatialAudio, true.into())]);

        device
            .set_settings(vec![(
                SettingId::PresetEqualizerProfile,
                "SoundcoreSignature".into(),
            )])
            .await;
        device.assert_setting_values(vec![(SettingId::SpatialAudio, false.into())]);
    }
}
