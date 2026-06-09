use std::collections::HashMap;

use crate::devices::soundcore::{
    a3968::{packets::inbound::A3968StateUpdatePacket, state::A3968State},
    common::{
        self,
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        modules::button_configuration::{
            ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings, COMMON_ACTIONS,
            COMMON_ACTIONS_MINIMAL,
        },
        packet::outbound::{RequestState, ToPacket},
        structures::button_configuration::{
            ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
        },
    },
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3968State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<A3968State, A3968StateUpdatePacket>(packet_io).await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3968_sound_modes();
        builder
            .equalizer_with_custom_hear_id_tws(common::modules::equalizer::common_settings())
            .await;
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.tws_status();
        builder.dual_battery(5);
        builder.case_battery_level(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3968StateUpdatePacket::default().to_packet(),
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

    /// State update packet posted by "Hate9" in GitHub issue #170 (a different X20 unit).
    #[tokio::test(start_paused = true)]
    async fn parses_state_update_packet_from_issue_170() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3968,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 1, 5, 5, 0, 0, 48, 49, 46, 54, 53, 48, 49, 46, 54, 53, 51, 57, 54, 56,
                        98, 48, 51, 56, 101, 50, 54, 98, 98, 101, 57, 97, 0, 0, 0, 0, 0, 2, 254,
                        254, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 48, 1, 1, 135, 114, 140, 142, 150, 145, 166, 134,
                        60, 60, 135, 114, 140, 142, 150, 145, 166, 134, 60, 60, 255, 255, 255, 255,
                        1, 151, 151, 140, 143, 151, 145, 165, 134, 60, 0, 151, 151, 140, 143, 151,
                        145, 165, 134, 60, 0, 0, 0, 8, 97, 102, 48, 51, 68, 68, 55, 0, 50, 1, 0, 0,
                        255, 54, 1, 0, 1, 0, 1, 2, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::TwsStatus, "Connected".into()),
            (SettingId::HostDevice, "Left".into()),
            (SettingId::BatteryLevelLeft, "5/5".into()),
            (SettingId::BatteryLevelRight, "5/5".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::FirmwareVersionLeft, "01.65".into()),
            (SettingId::FirmwareVersionRight, "01.65".into()),
            (SettingId::SerialNumber, "3968b038e26bbe9a".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::NoiseCancelingMode, "Manual".into()),
            (SettingId::ManualNoiseCanceling, "Strong".into()),
            (SettingId::AdaptiveNoiseCanceling, "HighNoise".into()),
            (SettingId::WindNoiseSuppression, false.into()),
            (SettingId::WindNoiseDetected, "false".into()),
            (SettingId::TransparencyMode, "VocalMode".into()),
            (
                SettingId::PresetEqualizerProfile,
                Value::OptionalString(None),
            ),
            (SettingId::CaseBatteryLevel, "2/5".into()),
            (SettingId::LeftSinglePress, Some("VolumeDown").into()),
            (SettingId::RightSinglePress, Some("PlayPause").into()),
            (SettingId::LeftDoublePress, Some("VolumeUp").into()),
            (SettingId::RightDoublePress, Some("NextSong").into()),
            (SettingId::LeftLongPress, Some("AmbientSoundMode").into()),
            (SettingId::RightLongPress, Some("AmbientSoundMode").into()),
        ]);
    }
}
