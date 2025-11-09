use std::collections::HashMap;

use crate::devices::soundcore::{
    a3945::{packets::A3945StateUpdatePacket, state::A3945State},
    common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        modules::button_configuration::{
            ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
            COMMON_ACTIONS_WITHOUT_SOUND_MODES,
        },
        packet::outbound::{RequestState, ToPacket},
        structures::button_configuration::{
            ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
        },
    },
};

mod packets;
mod state;

soundcore_device!(
    A3945State,
    A3945StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3945State, A3945StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.equalizer_tws().await;
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.reset_button_configuration::<A3945StateUpdatePacket>(
            RequestState::default().to_packet(),
        );
        builder.touch_tone();
        builder.gaming_mode();
        builder.wearing_detection();
        builder.tws_status();
        builder.dual_battery(5);
        builder.case_battery_level(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3945StateUpdatePacket::default().to_packet().bytes(),
        )])
    },
);

// Like COMMON_SETTINGS but without sound modes
pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<6, 3> =
    ButtonConfigurationSettings {
        supports_set_all_packet: true,
        ignore_enabled_flag: false,
        order: [
            Button::LeftDoublePress,
            Button::LeftLongPress,
            Button::RightDoublePress,
            Button::RightLongPress,
            Button::LeftSinglePress,
            Button::RightSinglePress,
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
                disable_mode: ButtonDisableMode::NotDisablable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::Single,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::NotDisablable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::Single,
                    action_kind: ActionKind::Single,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::DisablingOneSideDisablesOther,
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
                device::test_utils::TestSoundcoreDevice,
                packet::{self, outbound::ToPacket},
                structures::{EqualizerConfiguration, PresetEqualizerProfile},
            },
        },
    };

    #[tokio::test(start_paused = true)]
    async fn it_remembers_band_9_and_10_values() {
        let state_update_packet = packet::Inbound::new(
            packet::inbound::STATE_COMMAND,
            vec![
                0x00, // host device
                0x01, // tws status
                0x00, 0x00, 0x00, 0x00, // dual battery
                b'0', b'0', b'.', b'0', b'0', // left firmware version
                b'0', b'0', b'.', b'0', b'0', // right firmware version
                b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
                b'0', b'0', // serial number
                0x00, 0x00, // eq profile id
                120, 120, 120, 120, 120, 120, 120, 120, 121, 122, // left eq
                120, 120, 120, 120, 120, 120, 120, 120, 123, 124, // right eq
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, // custom button model
                0x00, // tone switch
                0x00, // wear detection
                0x00, // gaming mode
                0x00, // case battery
                0x00, // bass up
                0x00, // device color
            ],
        );

        let mut device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3945,
            HashMap::from([(packet::inbound::STATE_COMMAND, state_update_packet)]),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(
                    SettingId::PresetEqualizerProfile,
                    Value::OptionalString(Some("TrebleReducer".into())),
                )],
                vec![
                    packet::outbound::SetEqualizer {
                        equalizer_configuration:
                            &EqualizerConfiguration::<2, 10>::new_from_preset_profile(
                                PresetEqualizerProfile::TrebleReducer,
                                [vec![1, 2], vec![3, 4]],
                            ),
                    }
                    .to_packet(),
                ],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn it_parses_settings_correctly() {
        let state_update_packet = packet::Inbound::new(
            packet::inbound::STATE_COMMAND,
            vec![
                0, 1, 4, 5, 0, 0, 48, 48, 46, 48, 48, 48, 48, 46, 48, 48, 48, 48, 48, 48, 48, 48,
                48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 0, 0, 120, 120, 120, 120, 120, 120, 120,
                120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 1, 0x66, 1, 0x55,
                1, 0x33, 1, 0x22, 1, 0x1, 1, 0x0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        );

        let device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3945,
            HashMap::from([(packet::inbound::STATE_COMMAND, state_update_packet)]),
        )
        .await;

        device.assert_setting_values(vec![
            (
                SettingId::LeftSinglePress,
                Value::OptionalString(Some("VolumeDown".into())),
            ),
            (
                SettingId::RightSinglePress,
                Value::OptionalString(Some("VolumeUp".into())),
            ),
            (
                SettingId::LeftDoublePress,
                Value::String("PlayPause".into()),
            ),
            (
                SettingId::RightDoublePress,
                Value::String("NextSong".into()),
            ),
            (
                SettingId::LeftLongPress,
                Value::String("VoiceAssistant".into()),
            ),
            (
                SettingId::RightLongPress,
                Value::String("PreviousSong".into()),
            ),
        ]);
    }
}
