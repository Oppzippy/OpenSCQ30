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
                equalizer::{EqualizerModuleSettings, EqualizerPreset, InvisibleBandsMode},
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
        custom_preset_id: 0xfefe,
        band_hz: [100, 200, 400, 800, 1600, 3200, 6400, 12800],
        invisible_bands_mode: InvisibleBandsMode::Remember,
        presets: vec![
            EqualizerPreset {
                name: "SoundcoreSignature",
                localized_name: || fl!("soundcore-signature"),
                id: 0x0000,
                volume_adjustments: VolumeAdjustments::new([
                    120, 120, 120, 120, 120, 120, 120, 120, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Balanced",
                localized_name: || fl!("balanced"),
                id: 0x0001,
                volume_adjustments: VolumeAdjustments::new([
                    173, 99, 111, 104, 132, 81, 88, 122, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "BassBooster",
                localized_name: || fl!("bass-booster"),
                id: 0x0002,
                volume_adjustments: VolumeAdjustments::new([
                    160, 150, 130, 120, 120, 120, 120, 120, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Classical",
                localized_name: || fl!("classical"),
                id: 0x0004,
                volume_adjustments: VolumeAdjustments::new([
                    150, 150, 100, 100, 120, 140, 150, 160, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "SpokenWord",
                localized_name: || fl!("spoken-word"),
                id: 0x0005,
                volume_adjustments: VolumeAdjustments::new([
                    90, 140, 160, 160, 150, 140, 120, 100, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Dance",
                localized_name: || fl!("dance"),
                id: 0x0006,
                volume_adjustments: VolumeAdjustments::new([
                    140, 90, 110, 130, 140, 140, 130, 90, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Deep",
                localized_name: || fl!("deep"),
                id: 0x0007,
                volume_adjustments: VolumeAdjustments::new([
                    140, 130, 150, 150, 140, 100, 80, 70, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Electronic",
                localized_name: || fl!("electronic"),
                id: 0x0008,
                volume_adjustments: VolumeAdjustments::new([
                    150, 140, 100, 140, 130, 140, 150, 150, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Flat",
                localized_name: || fl!("flat"),
                id: 0x0009,
                volume_adjustments: VolumeAdjustments::new([
                    100, 100, 110, 120, 120, 120, 100, 100, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "HipHop",
                localized_name: || fl!("hip-hop"),
                id: 0x000a,
                volume_adjustments: VolumeAdjustments::new([
                    140, 150, 110, 110, 140, 110, 140, 150, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Jazz",
                localized_name: || fl!("jazz"),
                id: 0x000b,
                volume_adjustments: VolumeAdjustments::new([
                    140, 140, 100, 100, 120, 140, 150, 160, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Latin",
                localized_name: || fl!("latin"),
                id: 0x000c,
                volume_adjustments: VolumeAdjustments::new([
                    120, 120, 100, 100, 100, 120, 150, 170, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Lounge",
                localized_name: || fl!("lounge"),
                id: 0x000d,
                volume_adjustments: VolumeAdjustments::new([
                    110, 140, 160, 150, 120, 100, 140, 130, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Piano",
                localized_name: || fl!("piano"),
                id: 0x000e,
                volume_adjustments: VolumeAdjustments::new([
                    120, 150, 150, 140, 160, 170, 150, 160, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Pop",
                localized_name: || fl!("pop"),
                id: 0x000f,
                volume_adjustments: VolumeAdjustments::new([
                    110, 130, 150, 150, 130, 110, 100, 90, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "RnB",
                localized_name: || fl!("rnb"),
                id: 0x0010,
                volume_adjustments: VolumeAdjustments::new([
                    180, 140, 100, 100, 140, 150, 150, 160, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Rock",
                localized_name: || fl!("rock"),
                id: 0x0011,
                volume_adjustments: VolumeAdjustments::new([
                    150, 140, 110, 110, 130, 150, 160, 170, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "SmallSpeakers",
                localized_name: || fl!("small-speakers"),
                id: 0x0012,
                volume_adjustments: VolumeAdjustments::new([
                    160, 150, 130, 120, 100, 90, 80, 80, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "Podcast",
                localized_name: || fl!("podcast"),
                id: 0x0013,
                volume_adjustments: VolumeAdjustments::new([
                    90, 100, 130, 140, 140, 130, 120, 90, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "TrebleBooster",
                localized_name: || fl!("treble-booster"),
                id: 0x0014,
                volume_adjustments: VolumeAdjustments::new([
                    120, 120, 100, 120, 110, 70, 170, 130, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "TrebleReducer",
                localized_name: || fl!("treble-reducer"),
                id: 0x0015,
                volume_adjustments: VolumeAdjustments::new([
                    120, 120, 120, 100, 90, 80, 80, 60, 120, 0,
                ]),
            },
            EqualizerPreset {
                name: "VolumeBooster",
                localized_name: || fl!("volume-booster"),
                id: 0x001e,
                volume_adjustments: VolumeAdjustments::new([
                    140, 150, 160, 170, 180, 180, 170, 160, 120, 0,
                ]),
            },
        ],
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
}
