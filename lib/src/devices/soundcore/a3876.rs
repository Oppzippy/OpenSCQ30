use std::collections::HashMap;

use crate::{
    devices::soundcore::{
        a3876::{packets::inbound::A3876StateUpdatePacket, state::A3876State},
        common::{
            self,
            macros::soundcore_device,
            modules::{
                button_configuration::{
                    ButtonAction, ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
                    COMMON_ACTIONS_MINIMAL,
                },
                dual_battery_level::DualBatteryLevelConfiguration,
                equalizer::{EqualizerModuleSettings, EqualizerPreset},
            },
            packet::{
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
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3876State,
    async |packet_io| {
        let state_update_packet: packets::inbound::A3876StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(state::A3876State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.a3876_equalizer(equalizer_settings()).await;

        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.reset_button_configuration::<packets::inbound::A3876StateUpdatePacket>(
            RequestState.to_packet(),
        );

        builder.a3876_colorful_lights();

        builder.dual_connections();

        builder.a3876_volume_balance();
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::ten_twenty_thirty_sixty(),
        );
        builder.voice_prompt();
        builder.gaming_mode();

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
            A3876StateUpdatePacket::default().to_packet(),
        )])
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<8, 4> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: true,
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
                available_actions: BUTTON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 5,
                press_kind: ButtonPressKind::Triple,
                available_actions: BUTTON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: BUTTON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
        ],
    };

const BUTTON_ACTIONS: &[ButtonAction] = &[
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
    ButtonAction {
        id: 14,
        name: "Lights",
        localized_name: || fl!("lights-enabled"),
    },
];

pub fn equalizer_settings() -> EqualizerModuleSettings<8, 10, -120, 134, 1> {
    EqualizerModuleSettings {
        presets: vec![
            EqualizerPreset {
                name: "SoundcoreSignature",
                localized_name: || fl!("soundcore-signature"),
                id: 0,
                volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0, 0, -120]),
            },
            EqualizerPreset {
                name: "Balanced",
                localized_name: || fl!("balanced"),
                id: 1,
                volume_adjustments: VolumeAdjustments::new([
                    53, -21, -9, -16, 12, -39, -32, 2, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "BassBooster",
                localized_name: || fl!("bass-booster"),
                id: 2,
                volume_adjustments: VolumeAdjustments::new([40, 30, 10, 0, 0, 0, 0, 0, 0, -120]),
            },
            EqualizerPreset {
                name: "Classical",
                localized_name: || fl!("classical"),
                id: 4,
                volume_adjustments: VolumeAdjustments::new([
                    30, 30, -20, -20, 0, 20, 30, 40, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "SpokenWord",
                localized_name: || fl!("spoken-word"),
                id: 5,
                volume_adjustments: VolumeAdjustments::new([
                    -30, 20, 40, 40, 30, 20, 0, -20, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "Dance",
                localized_name: || fl!("dance"),
                id: 6,
                volume_adjustments: VolumeAdjustments::new([
                    20, -30, -10, 10, 20, 20, 10, -30, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "Deep",
                localized_name: || fl!("deep"),
                id: 7,
                volume_adjustments: VolumeAdjustments::new([
                    20, 10, 30, 30, 20, -20, -40, -50, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "Electronic",
                localized_name: || fl!("electronic"),
                id: 8,
                volume_adjustments: VolumeAdjustments::new([
                    30, 20, -20, 20, 10, 20, 30, 30, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "Flat",
                localized_name: || fl!("flat"),
                id: 9,
                volume_adjustments: VolumeAdjustments::new([
                    -20, -20, -10, 0, 0, 0, -20, -20, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "HipHop",
                localized_name: || fl!("hip-hop"),
                id: 10,
                volume_adjustments: VolumeAdjustments::new([
                    20, 30, -10, -10, 20, -10, 20, 30, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "Jazz",
                localized_name: || fl!("jazz"),
                id: 11,
                volume_adjustments: VolumeAdjustments::new([
                    20, 20, -20, -20, 0, 20, 30, 40, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "Latin",
                localized_name: || fl!("latin"),
                id: 12,
                volume_adjustments: VolumeAdjustments::new([
                    0, 0, -20, -20, -20, 0, 30, 50, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "Lounge",
                localized_name: || fl!("lounge"),
                id: 13,
                volume_adjustments: VolumeAdjustments::new([
                    -10, 20, 40, 30, 0, -20, 20, 10, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "Piano",
                localized_name: || fl!("piano"),
                id: 14,
                volume_adjustments: VolumeAdjustments::new([
                    0, 30, 30, 20, 40, 50, 30, 40, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "Pop",
                localized_name: || fl!("pop"),
                id: 15,
                volume_adjustments: VolumeAdjustments::new([
                    -10, 10, 30, 30, 10, -10, -20, -30, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "RnB",
                localized_name: || fl!("rnb"),
                id: 16,
                volume_adjustments: VolumeAdjustments::new([
                    60, 20, -20, -20, 20, 30, 30, 40, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "Rock",
                localized_name: || fl!("rock"),
                id: 17,
                volume_adjustments: VolumeAdjustments::new([
                    30, 20, -10, -10, 10, 30, 40, 50, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "SmallSpeakers",
                localized_name: || fl!("small-speakers"),
                id: 18,
                volume_adjustments: VolumeAdjustments::new([
                    40, 30, 10, 0, -20, -30, -40, -40, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "Podcast",
                localized_name: || fl!("podcast"),
                id: 19,
                volume_adjustments: VolumeAdjustments::new([
                    -30, -20, 10, 20, 20, 10, 0, -30, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "TrebleBooster",
                localized_name: || fl!("treble-booster"),
                id: 20,
                volume_adjustments: VolumeAdjustments::new([
                    0, 0, -20, 0, -10, -50, 50, 10, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "TrebleReducer",
                localized_name: || fl!("treble-reducer"),
                id: 21,
                volume_adjustments: VolumeAdjustments::new([
                    0, 0, 0, -20, -30, -40, -40, -60, 0, -120,
                ]),
            },
            EqualizerPreset {
                name: "VolumeBooster",
                localized_name: || fl!("volume-booster"),
                id: 30,
                volume_adjustments: VolumeAdjustments::new([
                    20, 30, 40, 50, 60, 60, 50, 40, 0, -120,
                ]),
            },
        ],
        ..common::modules::equalizer::common_settings()
    }
}

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
            DeviceModel::SoundcoreA3876,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 5, 9, 48, 51, 46, 49, 53, 48, 51, 46, 49, 53, 51, 56, 55, 54, 66, 69,
                        52, 65, 57, 54, 50, 55, 54, 67, 49, 52, 255, 255, 255, 255, 255, 255, 0xfe,
                        0xfe, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 9, 0xff, 0xf6, 0xf3, 0x63, 0xff, 0xf5, 0xff,
                        0xf0, 1, 4, 45, 1, 23, 187, 239, 49, 1, 1, 2, 0, 0, 100, 1, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "6/10".into()),
            (SettingId::BatteryLevelRight, "10/10".into()),
            (SettingId::LeftSinglePress, Value::OptionalString(None)),
            (SettingId::LeftDoublePress, Some("NextSong").into()),
            (SettingId::LeftTriplePress, Value::OptionalString(None)),
            (SettingId::LeftLongPress, Value::OptionalString(None)),
            (SettingId::RightSinglePress, Some("PlayPause").into()),
            (SettingId::RightDoublePress, Some("NextSong").into()),
            (SettingId::RightTriplePress, Some("VoiceAssistant").into()),
            (SettingId::RightLongPress, Some("VolumeUp").into()),
            (SettingId::GamingMode, false.into()),
            (SettingId::LightsEnabled, true.into()),
            (SettingId::LightsBrightness, 4.into()),
            (SettingId::AutoLightsOffMinutes, 45.into()),
            (SettingId::LightsColor, 194.44444.into()), // hsv(194.44444, 1.0, 1.0) #00c4ff
            (SettingId::LightsMode, "Breathing".into()),
            (SettingId::VolumeBalance, 0.into()),
            (SettingId::VoicePrompt, true.into()),
            (SettingId::DualConnections, true.into()),
            (SettingId::AutoPowerOff, "30m".into()),
            (SettingId::FirmwareVersionLeft, "03.15".into()),
            (SettingId::FirmwareVersionRight, "03.15".into()),
            (SettingId::SerialNumber, "3876BE4A96276C14".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_custom_eq() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3876,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 5, 9, 48, 51, 46, 49, 53, 48, 51, 46, 49, 53, 51, 56, 55, 54, 66, 69,
                        52, 65, 57, 54, 50, 55, 54, 67, 49, 52, 255, 255, 255, 255, 255, 255, 0xfe,
                        0xfe, 120, 120, 120, 120, 120, 120, 120, 120, 250, 250, 120, 120, 120, 120,
                        120, 120, 120, 120, 250, 250, 9, 0xff, 0xf6, 0xf3, 0x63, 0xff, 0xf5, 0xff,
                        0xf0, 1, 4, 45, 1, 23, 187, 239, 49, 1, 1, 2, 0, 0, 100, 1, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(
                    SettingId::VolumeAdjustments,
                    Value::I16Vec(vec![0, 0, 0, 0, 0, 0, 0, 60]).into(),
                )],
                vec![packet::Outbound::new(
                    packet::Command([2, 131]),
                    vec![
                        254, 254, 120, 120, 120, 120, 120, 120, 120, 180, 120, 120, 120, 120, 120,
                        120, 120, 121, 116, 129, 120, 0,
                    ],
                )],
            )
            .await;
    }
}
