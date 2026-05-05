use std::collections::HashMap;

use crate::{
    devices::soundcore::{
        a3909::{packets::inbound::A3909StateUpdatePacket, state::A3909State},
        common::{
            macros::soundcore_device,
            modules::button_configuration::{
                ButtonAction, ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
            },
            packet::{
                inbound::{SerialNumberAndFirmwareVersion, TryToPacket},
                outbound::{RequestSerialNumberAndFirmwareVersion, RequestState, ToPacket},
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
    A3909State,
    async |packet_io| {
        let state_update_packet: A3909StateUpdatePacket = packet_io
            .send_with_response(&RequestState::default().to_packet())
            .await?
            .try_to_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersion = packet_io
            .send_with_response(&RequestSerialNumberAndFirmwareVersion::default().to_packet())
            .await?
            .try_to_packet()?;
        Ok(A3909State::new(state_update_packet, sn_and_firmware))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.a3909_equalizer().await;

        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);

        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestState::COMMAND,
                A3909StateUpdatePacket::default().to_packet(),
            ),
            (
                RequestSerialNumberAndFirmwareVersion::COMMAND,
                SerialNumberAndFirmwareVersion::default().to_packet(),
            ),
        ])
    },
);

const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<4, 2> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: false,
        order: [
            Button::LeftDoublePress,
            Button::LeftLongPress,
            Button::RightDoublePress,
            Button::RightLongPress,
        ],
        settings: [
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::Single,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: BUTTON_ACTIONS,
                disable_mode: ButtonDisableMode::NotDisablable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::Single,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: BUTTON_ACTIONS,
                disable_mode: ButtonDisableMode::NotDisablable,
            },
        ],
    };

pub const BUTTON_ACTIONS: &[ButtonAction] = &[
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
];

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        devices::soundcore::common::{
            device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
            packet,
        },
        settings::{self, SettingId},
    };

    #[tokio::test(start_paused = true)]
    async fn test_with_packet_from_issue_264() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3957,
            HashMap::from([
                (
                    packet::Command([1, 1]),
                    packet::Inbound::new(
                        packet::Command([1, 1]),
                        vec![
                            1, 0, 255, 5, 0, 0, 254, 254, 1, 1, 1, 12, 12, 12, 15, 12, 12, 12, 12,
                            12, 12, 12, 12, 12, 12, 12, 12, 105, 142, 168, 103, 2, 1, 3, 0, 0,
                        ],
                    ),
                ),
                (
                    packet::Command([1, 5]),
                    packet::Inbound::new(
                        packet::Command([1, 5]),
                        vec![
                            0x30, 0x31, 0x2e, 0x32, 0x38, 0x30, 0x31, 0x2e, 0x32, 0x38, 0x33, 0x39,
                            0x30, 0x39, 0x39, 0x38, 0x35, 0x32, 0x33, 0x44, 0x38, 0x43, 0x42, 0x45,
                            0x41, 0x46,
                        ],
                    ),
                ),
            ]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::FirmwareVersionLeft, "01.28".into()),
            (SettingId::FirmwareVersionRight, "01.28".into()),
            (SettingId::SerialNumber, "390998523D8CBEAF".into()),
            (
                SettingId::PresetEqualizerProfile,
                settings::Value::OptionalString(None),
            ),
            (
                SettingId::VolumeAdjustments,
                settings::Value::I16Vec(vec![0; 8]),
            ),
            (SettingId::BatteryLevelLeft, "255/5".into()), // TODO this should instead be None
            (SettingId::BatteryLevelRight, "5/5".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::LeftDoublePress, "PreviousSong".into()),
            (SettingId::RightDoublePress, "NextSong".into()),
            (SettingId::LeftLongPress, "VolumeDown".into()),
            (SettingId::RightLongPress, "VolumeUp".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn test_equalizer_presets_have_proper_volume_adjustments() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3957,
            HashMap::from([
                (
                    packet::Command([1, 1]),
                    packet::Inbound::new(
                        packet::Command([1, 1]),
                        vec![
                            1, 0, 255, 5, 0, 0, 6, 0, 1, 1, 1, 12, 12, 12, 15, 12, 12, 12, 12, 12,
                            12, 12, 12, 12, 12, 12, 12, 105, 142, 168, 103, 2, 1, 3, 0, 0,
                        ],
                    ),
                ),
                (
                    packet::Command([1, 5]),
                    packet::Inbound::new(
                        packet::Command([1, 5]),
                        vec![
                            0x30, 0x31, 0x2e, 0x32, 0x38, 0x30, 0x31, 0x2e, 0x32, 0x38, 0x33, 0x39,
                            0x30, 0x39, 0x39, 0x38, 0x35, 0x32, 0x33, 0x44, 0x38, 0x43, 0x42, 0x45,
                            0x41, 0x46,
                        ],
                    ),
                ),
            ]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([(
            SettingId::VolumeAdjustments,
            settings::Value::I16Vec(vec![2, -3, -1, 1, 2, 2, 1, -3]),
        )]);
    }
}
