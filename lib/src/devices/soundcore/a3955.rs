use std::collections::HashMap;

use crate::devices::soundcore::common::{
    self,
    device::fetch_state_from_state_update_packet,
    macros::soundcore_device,
    modules::{
        button_configuration::{
            ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings, COMMON_ACTIONS,
        },
        equalizer,
    },
    packet::outbound::{RequestState, ToPacket},
    structures::button_configuration::{
        ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
    },
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    state::A3955State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<
            _,
            state::A3955State,
            packets::inbound::A3955StateUpdatePacket,
        >(packet_io)
        .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3955_sound_modes();
        builder
            .equalizer_with_custom_hear_id_tws(equalizer::common_settings())
            .await;
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.ambient_sound_mode_cycle();
        builder.reset_button_configuration::<packets::inbound::A3955StateUpdatePacket>(
            RequestState::default().to_packet(),
        );

        builder.limit_high_volume();

        builder.a3955_immersive_experience();
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::ten_twenty_thirty_sixty(),
        );
        builder.touch_tone();
        builder.low_battery_prompt();

        builder.tws_status();
        builder.dual_battery(5);
        builder.case_battery_level(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            packets::inbound::A3955StateUpdatePacket::default().to_packet(),
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
    async fn test_with_packet_from_firmware_1_6_1() {
        //packet from github issue 159
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3955,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0x00, 0x01, 0x05, 0x05, 0x00, 0x01, 0x30, 0x31, 0x2e, 0x36, 0x31, 0x30,
                        0x31, 0x2e, 0x36, 0x31, 0x33, 0x39, 0x35, 0x35, 0x39, 0x38, 0x34, 0x37,
                        0x34, 0x34, 0x36, 0x36, 0x66, 0x35, 0x37, 0x33, 0x30, 0x2e, 0x31, 0x2e,
                        0x38, 0x03, 0x08, 0x00, 0x96, 0x8c, 0x64, 0x8c, 0x82, 0x8c, 0x96, 0x96,
                        0x78, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                        0xff, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00,
                        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                        0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
                        0x00, 0x0a, 0xff, 0xff, 0x63, 0x66, 0xf4, 0xff, 0x44, 0x44, 0x35, 0x00,
                        0x51, 0x01, 0x02, 0x01, 0xff, 0x01, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
                        0x01, 0x6f, 0x00, 0x01, 0x01, 0x5f, 0x00, 0x01, 0x02, 0x02, 0x01, 0xff,
                        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x67, 0x89,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "5/5".into()),
            (SettingId::BatteryLevelRight, "5/5".into()),
            (SettingId::CaseBatteryLevel, "3/5".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "Yes".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::NoiseCancelingMode, "MultiScene".into()),
            (SettingId::MultiSceneNoiseCanceling, "Outdoor".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::AdaptiveNoiseCanceling, "Weak".into()),
            (SettingId::WindNoiseSuppression, true.into()),
            (SettingId::TransparencyMode, "VocalMode".into()),
            (SettingId::LeftSinglePress, Value::OptionalString(None)),
            (SettingId::LeftDoublePress, Some("NextSong").into()),
            (SettingId::LeftTriplePress, Some("AmbientSoundMode").into()),
            (SettingId::LeftLongPress, Some("AmbientSoundMode").into()),
            (SettingId::RightSinglePress, Value::OptionalString(None)),
            (SettingId::RightDoublePress, Some("PlayPause").into()),
            (SettingId::RightTriplePress, Value::OptionalString(None)),
            (SettingId::RightLongPress, Some("AmbientSoundMode").into()),
            (SettingId::LimitHighVolume, true.into()),
            (SettingId::LimitHighVolumeDbLimit, 95.into()),
            (SettingId::LimitHighVolumeRefreshRate, "RealTime".into()),
            (SettingId::TouchTone, false.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::AutoPowerOff, "30m".into()),
            (SettingId::FirmwareVersionLeft, "01.61".into()),
            (SettingId::FirmwareVersionRight, "01.61".into()),
            (SettingId::SerialNumber, "395598474466f573".into()),
            (SettingId::PresetEqualizerProfile, Some("Electronic").into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn immersive_experience() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3955,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0x00, 0x01, 0x05, 0x05, 0x00, 0x01, 0x30, 0x31, 0x2e, 0x36, 0x31, 0x30,
                        0x31, 0x2e, 0x36, 0x31, 0x33, 0x39, 0x35, 0x35, 0x39, 0x38, 0x34, 0x37,
                        0x34, 0x34, 0x36, 0x36, 0x66, 0x35, 0x37, 0x33, 0x30, 0x2e, 0x31, 0x2e,
                        0x38, 0x03, 0x08, 0x00, 0x96, 0x8c, 0x64, 0x8c, 0x82, 0x8c, 0x96, 0x96,
                        0x78, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                        0xff, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00,
                        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                        0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
                        0x00, 0x0a, 0xff, 0xff, 0x63, 0x66, 0xf4, 0xff, 0x44, 0x44, 0x35, 0x00,
                        0x51, 0x01, 0x02, 0x01, 0xff, 0x01, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
                        0x01, 0x6f, 0x00, 0x01, 0x01, 0x5f, 0x00, 0x01, 0x02, 0x02, 0x01, 0xff,
                        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x67, 0x89,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([(SettingId::ImmersiveExperience, "Disabled".into())]);
        device
            .assert_set_settings_response(
                vec![(SettingId::ImmersiveExperience, "MovieMode".into())],
                vec![packet::Outbound::new(packet::Command([18, 129]), vec![2])],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn state_update_packet_from_github_issue_241() {
        // no assertions, just tests that it doesn't error due to bad state update packet
        TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3955,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 0, 5, 5, 255, 0, 255, 255, 255, 255, 255, 48, 49, 46, 55, 50, 51, 57,
                        53, 53, 57, 56, 52, 55, 52, 52, 98, 48, 55, 52, 56, 53, 48, 46, 49, 46, 56,
                        5, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 5, 0, 138, 145, 142, 142, 123, 110, 102,
                        97, 60, 60, 138, 145, 142, 142, 123, 110, 102, 97, 60, 60, 105, 150, 134,
                        241, 0, 138, 145, 142, 142, 123, 110, 102, 97, 60, 60, 138, 145, 142, 142,
                        123, 110, 102, 97, 60, 60, 0, 0, 10, 255, 255, 99, 102, 255, 255, 68, 68,
                        51, 2, 81, 0, 0, 1, 255, 0, 0, 0, 0, 0, 255, 255, 0, 49, 1, 1, 0, 85, 0, 1,
                        2, 1, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;
    }
}
