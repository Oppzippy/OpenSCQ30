use std::collections::HashMap;

use crate::devices::soundcore::{
    a3948::{packets::inbound::A3948StateUpdatePacket, state::A3948State},
    common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        modules::button_configuration::{
            ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
            COMMON_ACTIONS_WITHOUT_SOUND_MODES,
        },
        packet::outbound::{IntoPacket, RequestState},
        structures::button_configuration::{
            ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
        },
    },
};

mod packets;
mod state;

soundcore_device!(
    A3948State,
    A3948StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3948State, A3948StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.equalizer_with_drc_tws().await;

        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);

        builder.touch_tone();

        builder.serial_number_and_dual_firmware_version();
        builder.tws_status();
        builder.dual_battery(5);
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3948StateUpdatePacket::default().into_packet().bytes(),
        )])
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<6, 3> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        use_enabled_flag_to_disable: false,
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
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
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
        ],
    };

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        devices::soundcore::common::{device::test_utils::TestSoundcoreDevice, packet},
        settings::{SettingId, Value},
    };

    #[tokio::test(start_paused = true)]
    async fn test_new_with_example_state_update_packet() {
        let device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3948,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 0, 5, 255, 0, 0, 50, 49, 46, 53, 54, 0, 0, 0, 0, 0, 51, 57, 52, 56, 55,
                        49, 48, 54, 56, 54, 54, 54, 65, 69, 70, 48, 19, 0, 90, 100, 130, 140, 140,
                        130, 120, 90, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 1, 241, 1, 255, 1,
                        98, 1, 246, 1, 54, 1, 243, 255, 255, 255, 49, 0, 1, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    ],
                ),
            )]),
        )
        .await;
        device.assert_setting_values([
            (SettingId::FirmwareVersionLeft, "21.56".into()),
            (SettingId::FirmwareVersionRight, "".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn disabled_buttons_in_state_update_packet_parse_correctly() {
        let state_update_packet = packet::Inbound::new(
            packet::Command([1, 1]),
            vec![
                0, 1, 5, 5, 0, 0, 50, 52, 46, 53, 54, 50, 52, 46, 53, 54, 51, 57, 52, 56, 49, 57,
                70, 70, 49, 68, 67, 65, 66, 65, 50, 67, 2, 0, 160, 150, 130, 120, 120, 120, 120,
                120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, //
                1, 0xF6, // Left single press
                0, 0xF0, // Right single press (disabled, VolumeUp)
                1, 0x6F, // Left double press
                1, 0x6F, // Right double press (enabled, 0xF meaning disabled)
                1, 0x31, // Left long press
                0, 0x3F, // Right long press (disabled, 0F meaning disabled)
                255, 255, 255, 97, 0, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255,
            ],
        );
        let device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3948,
            HashMap::from([(packet::Command([1, 1]), state_update_packet)]),
        )
        .await;
        device.assert_setting_values([
            (SettingId::RightSinglePress, Value::OptionalString(None)),
            (SettingId::RightDoublePress, Value::OptionalString(None)),
            (SettingId::RightLongPress, Value::OptionalString(None)),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn disabled_buttons_are_moved_to_enabled_state_correctly() {
        let state_update_packet = packet::Inbound::new(
            packet::Command([1, 1]),
            vec![
                0, 1, 5, 5, 0, 0, 50, 52, 46, 53, 54, 50, 52, 46, 53, 54, 51, 57, 52, 56, 49, 57,
                70, 70, 49, 68, 67, 65, 66, 65, 50, 67, 2, 0, 160, 150, 130, 120, 120, 120, 120,
                120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, //
                1, 0xF6, // Left single press
                0, 0xF0, // Right single press (disabled, VolumeUp)
                1, 0x6F, // Left double press
                1, 0x6F, // Right double press (enabled, 0xF meaning disabled)
                1, 0x31, // Left long press
                0, 0x3F, // Right long press (disabled, 0F meaning disabled)
                255, 255, 255, 97, 0, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255,
            ],
        );
        let mut device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3948,
            HashMap::from([(packet::Command([1, 1]), state_update_packet)]),
        )
        .await;
        device
            .assert_set_settings_response_unordered(
                vec![
                    (SettingId::RightSinglePress, "VolumeUp".into()),
                    (SettingId::RightDoublePress, "VolumeUp".into()),
                    (SettingId::RightLongPress, "VolumeUp".into()),
                ],
                vec![
                    // Single
                    packet::Outbound::new(packet::Command([0x04, 0x83]), vec![1, 2, 1]),
                    //Double
                    packet::Outbound::new(packet::Command([0x04, 0x81]), vec![1, 0, 0x60]),
                    // Long
                    packet::Outbound::new(packet::Command([0x04, 0x81]), vec![1, 1, 0x30]),
                    packet::Outbound::new(packet::Command([0x04, 0x83]), vec![1, 1, 1]),
                ],
            )
            .await;
    }
}
