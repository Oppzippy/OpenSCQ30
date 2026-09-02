use std::collections::HashMap;

use crate::{
    devices::soundcore::{
        a3944::{packets::inbound::A3944StateUpdatePacket, state::A3944State},
        common::{
            self,
            device::fetch_state_from_state_update_packet,
            macros::soundcore_device,
            modules::{
                button_configuration::{
                    ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
                    COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                },
                equalizer::{EqualizerModuleSettings, EqualizerPreset},
            },
            packet::outbound::{RequestState, ToPacket},
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

mod packets;
mod state;

soundcore_device!(
    A3944State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<A3944State, A3944StateUpdatePacket>(packet_io).await
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.equalizer_with_drc(equalizer_settings()).await;

        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);

        builder.touch_tone();

        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3944StateUpdatePacket::default().to_packet(),
        )])
    },
);

fn equalizer_settings() -> EqualizerModuleSettings<8, 10, -120, 134, 1> {
    let mut settings = common::modules::equalizer::common_settings_with_presets(vec![
        EqualizerPreset {
            name: "SoundcoreSignature",
            localized_name: || fl!("soundcore-signature"),
            id: 0,
            volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0, 0, -120]),
        },
        EqualizerPreset {
            name: "Acoustic",
            localized_name: || fl!("acoustic"),
            id: 1,
            volume_adjustments: VolumeAdjustments::new([40, 10, 20, 20, 40, 40, 40, 20, 0, -120]),
        },
        EqualizerPreset {
            name: "BassBooster",
            localized_name: || fl!("bass-booster"),
            id: 2,
            volume_adjustments: VolumeAdjustments::new([50, 50, 30, 10, 10, 20, -10, -20, 0, -120]),
        },
        EqualizerPreset {
            name: "BassReducer",
            localized_name: || fl!("bass-reducer"),
            id: 3,
            volume_adjustments: VolumeAdjustments::new([-40, -30, -10, 0, 0, 0, 0, 0, 0, -120]),
        },
        EqualizerPreset {
            name: "Classical",
            localized_name: || fl!("classical"),
            id: 4,
            volume_adjustments: VolumeAdjustments::new([30, 30, -20, -20, 0, 20, 30, 40, 0, -120]),
        },
        EqualizerPreset {
            name: "Podcast",
            localized_name: || fl!("podcast"),
            id: 5,
            volume_adjustments: VolumeAdjustments::new([-30, 20, 40, 40, 30, 20, 0, -20, 0, -120]),
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
            volume_adjustments: VolumeAdjustments::new([30, 20, -20, 20, 10, 20, 30, 30, 0, -120]),
        },
        EqualizerPreset {
            name: "Flat",
            localized_name: || fl!("flat"),
            id: 9,
            volume_adjustments: VolumeAdjustments::new([-20, -20, -10, 0, 0, 0, -20, -20, 0, -120]),
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
            volume_adjustments: VolumeAdjustments::new([20, 20, -20, -20, 0, 20, 30, 40, 0, -120]),
        },
        EqualizerPreset {
            name: "Latin",
            localized_name: || fl!("latin"),
            id: 12,
            volume_adjustments: VolumeAdjustments::new([0, 0, -20, -20, -20, 0, 30, 50, 0, -120]),
        },
        EqualizerPreset {
            name: "Lounge",
            localized_name: || fl!("lounge"),
            id: 13,
            volume_adjustments: VolumeAdjustments::new([-10, 20, 40, 30, 0, -20, 20, 10, 0, -120]),
        },
        EqualizerPreset {
            name: "Piano",
            localized_name: || fl!("piano"),
            id: 14,
            volume_adjustments: VolumeAdjustments::new([0, 30, 30, 20, 40, 50, 30, 40, 0, -120]),
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
            volume_adjustments: VolumeAdjustments::new([60, 20, -20, -20, 20, 30, 30, 40, 0, -120]),
        },
        EqualizerPreset {
            name: "Rock",
            localized_name: || fl!("rock"),
            id: 17,
            volume_adjustments: VolumeAdjustments::new([30, 20, -10, -10, 10, 30, 40, 50, 0, -120]),
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
            name: "SpokenWord",
            localized_name: || fl!("spoken-word"),
            id: 19,
            volume_adjustments: VolumeAdjustments::new([-30, -20, 10, 20, 20, 10, 0, -30, 0, -120]),
        },
        EqualizerPreset {
            name: "TrebleBooster",
            localized_name: || fl!("treble-booster"),
            id: 20,
            volume_adjustments: VolumeAdjustments::new([
                -20, -20, -20, -10, 10, 20, 20, 40, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "TrebleReducer",
            localized_name: || fl!("treble-reducer"),
            id: 21,
            volume_adjustments: VolumeAdjustments::new([0, 0, 0, -20, -30, -40, -40, -60, 0, -120]),
        },
    ]);
    settings.custom_preset_id = None;
    settings
}

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<6, 3> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: true,
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
                    enabled_flag_kind: EnabledFlagKind::Single,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::Single,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::Single,
                    action_kind: ActionKind::Single,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
        ],
    };

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        api::settings::{SettingId, Value},
        devices::{
            DeviceModel,
            soundcore::common::{
                device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
                packet,
            },
        },
    };

    #[tokio::test(start_paused = true)]
    async fn it_parses_settings_correctly() {
        let state_update_packet = packet::Inbound::new(
            packet::inbound::STATE_COMMAND,
            vec![
                0, 1, 5, 5, 0, 0, 48, 50, 46, 50, 55, 48, 50, 46, 50, 55, 51, 57, 52, 52, 70, 53,
                52, 68, 49, 56, 67, 65, 66, 65, 50, 67, 0, 0, 120, 120, 120, 120, 120, 120, 120,
                120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 1, 0, 1, 0, 1, 102, 1, 102, 1, 50,
                1, 51, 255, 255, 255, 54, 0, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255,
            ],
        );

        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3944,
            HashMap::from([(packet::inbound::STATE_COMMAND, state_update_packet)]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values(vec![
            (
                SettingId::PresetEqualizerProfile,
                Value::OptionalString(Some("SoundcoreSignature".into())),
            ),
            (
                SettingId::LeftSinglePress,
                Value::OptionalString(Some("VolumeUp".into())),
            ),
            (
                SettingId::RightSinglePress,
                Value::OptionalString(Some("VolumeUp".into())),
            ),
            (
                SettingId::LeftDoublePress,
                Value::OptionalString(Some("PlayPause".into())),
            ),
            (
                SettingId::RightDoublePress,
                Value::OptionalString(Some("PlayPause".into())),
            ),
            (
                SettingId::LeftLongPress,
                Value::OptionalString(Some("PreviousSong".into())),
            ),
            (
                SettingId::RightLongPress,
                Value::OptionalString(Some("NextSong".into())),
            ),
            (SettingId::TouchTone, true.into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_eq_packet_matches() {
        let state_update_packet = packet::Inbound::new(
            packet::inbound::STATE_COMMAND,
            vec![
                0, 1, 5, 5, 0, 0, 48, 50, 46, 50, 55, 48, 50, 46, 50, 55, 51, 57, 52, 52, 70, 53,
                52, 68, 49, 56, 67, 65, 66, 65, 50, 67, 0, 0, 120, 120, 120, 120, 120, 120, 120,
                120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 1, 0, 1, 0, 1, 102, 1, 102, 1, 50,
                1, 51, 255, 255, 255, 54, 0, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255,
            ],
        );

        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3944,
            HashMap::from([(packet::inbound::STATE_COMMAND, state_update_packet)]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(
                    SettingId::PresetEqualizerProfile,
                    Value::OptionalString(Some("Acoustic".into())),
                )],
                vec![packet::Outbound::new(
                    packet::Command([2, 131]),
                    vec![
                        1, 0, 160, 130, 140, 140, 160, 160, 160, 140, 120, 0, 125, 118, 123, 120,
                        124, 122, 124, 121, 120, 0,
                    ],
                )],
            )
            .await;
    }
}
