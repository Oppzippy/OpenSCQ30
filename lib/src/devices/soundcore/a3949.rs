use std::collections::HashMap;

use crate::devices::soundcore::{
    a3949::{packets::inbound::A3949StateUpdatePacket, state::A3949State},
    common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        modules::{
            button_configuration::{
                ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
                COMMON_ACTIONS_WITHOUT_SOUND_MODES,
            },
            equalizer,
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
    A3949State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3949State, A3949StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder
            .equalizer_with_drc_tws(equalizer::common_settings())
            .await;

        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.reset_button_configuration::<A3949StateUpdatePacket>(
            RequestState::default().to_packet(),
        );

        builder.gaming_mode();
        builder.touch_tone();

        builder.serial_number_and_dual_firmware_version();
        builder.tws_status();
        builder.dual_battery(5);
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3949StateUpdatePacket::default().to_packet(),
        )])
    },
);

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
        devices::soundcore::common::{
            device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
            packet,
        },
        settings::{SettingId, Value},
    };

    #[tokio::test(start_paused = true)]
    async fn example_packet_matches_soundcore_app() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3949,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 1, 3, 4, 0, 0, 49, 52, 46, 52, 51, 49, 52, 46, 52, 51, 51, 57, 52, 57,
                        66, 57, 65, 69, 67, 55, 50, 67, 57, 67, 49, 56, 0, 0, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 1, 0x00, 1,
                        0x11, 1, 0x22, 1, 0x33, 1, 0x55, 1, 0x66, 255, 255, 255, 97, 0, 1, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;
        device.assert_setting_values([
            (SettingId::FirmwareVersionLeft, "14.43".into()),
            (SettingId::FirmwareVersionRight, "14.43".into()),
            (SettingId::SerialNumber, "3949B9AEC72C9C18".into()),
            (SettingId::TouchTone, true.into()),
            (SettingId::GamingMode, false.into()),
            (
                SettingId::PresetEqualizerProfile,
                Some("SoundcoreSignature").into(),
            ),
            (SettingId::LeftSinglePress, Some("VolumeUp").into()),
            (SettingId::RightSinglePress, Some("VolumeDown").into()),
            (SettingId::LeftDoublePress, Some("PreviousSong").into()),
            (SettingId::RightDoublePress, Some("NextSong").into()),
            (SettingId::LeftLongPress, Some("VoiceAssistant").into()),
            (SettingId::RightLongPress, Some("PlayPause").into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn p20i_packet_matches_soundcore_app() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3949,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 0, 255, 4, 0, 0, 0, 0, 0, 0, 0, 49, 52, 46, 52, 51, 51, 57, 52, 57, 66,
                        57, 65, 69, 67, 55, 50, 67, 57, 67, 49, 56, 0, 0, 120, 120, 120, 120, 120,
                        120, 120, 120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 1, 255, 1, 255, 1,
                        245, 1, 102, 1, 242, 1, 51, 255, 255, 255, 97, 0, 1, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;
        device.assert_setting_values([
            (SettingId::FirmwareVersionRight, "14.43".into()),
            (SettingId::SerialNumber, "3949B9AEC72C9C18".into()),
            (
                SettingId::RightSinglePress,
                Value::OptionalString(None).into(),
            ),
            (SettingId::RightDoublePress, Some("PlayPause").into()),
            (SettingId::RightLongPress, Some("NextSong").into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn r50i_matches_soundcore_app() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3949,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 1, 3, 3, 0, 0, 49, 57, 46, 52, 51, 49, 57, 46, 52, 51, 51, 57, 52, 57,
                        51, 70, 50, 52, 57, 69, 50, 68, 66, 54, 70, 52, 0, 0, 120, 120, 120, 120,
                        120, 120, 120, 120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 1, 246, 1,
                        246, 1, 98, 1, 99, 1, 51, 1, 51, 255, 255, 255, 49, 0, 0, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;
        device.assert_setting_values([
            (SettingId::FirmwareVersionLeft, "19.43".into()),
            (SettingId::FirmwareVersionRight, "19.43".into()),
            (SettingId::SerialNumber, "39493F249E2DB6F4".into()),
            (SettingId::TouchTone, false.into()),
            (SettingId::GamingMode, false.into()),
            (
                SettingId::PresetEqualizerProfile,
                Some("SoundcoreSignature").into(),
            ),
            (SettingId::LeftSinglePress, Some("PlayPause").into()),
            (SettingId::RightSinglePress, Some("PlayPause").into()),
            (SettingId::LeftDoublePress, Some("PreviousSong").into()),
            (SettingId::RightDoublePress, Some("NextSong").into()),
            (SettingId::LeftLongPress, Some("NextSong").into()),
            (SettingId::RightLongPress, Some("NextSong").into()),
        ]);
    }
}
