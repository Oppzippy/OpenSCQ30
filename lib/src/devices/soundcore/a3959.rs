use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        modules::button_configuration::{
            ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings, COMMON_ACTIONS,
        },
        packet::outbound::{RequestState, ToPacket},
        structures::button_configuration::{
            ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
        },
    },
    i18n::fl,
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    state::A3959State,
    packets::inbound::A3959State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, state::A3959State, packets::inbound::A3959State>(
            packet_io,
        )
        .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3959_sound_modes();
        builder.equalizer_with_drc_tws().await;
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.ambient_sound_mode_cycle();
        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);
        builder.touch_tone();
        builder.tws_status();
        builder.dual_battery(10);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            packets::inbound::A3959State::default().to_packet().bytes(),
        )])
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<8, 4> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        use_enabled_flag_to_disable: true,
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

#[derive(IntoStaticStr, VariantArray)]
#[allow(clippy::enum_variant_names)]
enum AutoPowerOffDuration {
    #[strum(serialize = "10m")]
    TenMinutes,
    #[strum(serialize = "20m")]
    TwentyMinutes,
    #[strum(serialize = "30m")]
    ThirtyMinutes,
    #[strum(serialize = "60m")]
    SixtyMinutes,
}

impl Translate for AutoPowerOffDuration {
    fn translate(&self) -> String {
        match self {
            Self::TenMinutes => fl!("x-minutes", minutes = 10),
            Self::TwentyMinutes => fl!("x-minutes", minutes = 20),
            Self::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            Self::SixtyMinutes => fl!("x-minutes", minutes = 60),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        devices::soundcore::common::{device::test_utils::TestSoundcoreDevice, packet},
        settings::{SettingId, Value},
    };

    #[tokio::test(start_paused = true)]
    async fn test_with_packet_from_github_issue_149() {
        let device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3959,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 5, 6, 255, 255, 48, 49, 46, 54, 52, 48, 49, 46, 54, 52, 51, 57, 53,
                        57, 68, 69, 68, 54, 54, 57, 50, 68, 66, 54, 70, 52, 254, 254, 101, 120,
                        161, 171, 171, 152, 144, 179, 120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
                        241, 240, 102, 102, 242, 243, 68, 68, 51, 0, 85, 0, 0, 1, 255, 1, 49, 1, 1,
                        0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "5".into()),
            (SettingId::BatteryLevelRight, "6".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::AdaptiveNoiseCanceling, "5/5".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::MultiSceneNoiseCanceling, "Outdoor".into()),
            (SettingId::WindNoiseSuppression, true.into()),
            (SettingId::LeftSinglePress, Some("VolumeDown").into()),
            (SettingId::LeftDoublePress, Some("PlayPause").into()),
            (SettingId::LeftTriplePress, Some("PreviousSong").into()),
            (SettingId::LeftLongPress, Some("AmbientSoundMode").into()),
            (SettingId::RightSinglePress, Some("VolumeUp").into()),
            (SettingId::RightDoublePress, Some("PlayPause").into()),
            (SettingId::RightTriplePress, Some("NextSong").into()),
            (SettingId::RightLongPress, Some("AmbientSoundMode").into()),
            (SettingId::TouchTone, true.into()),
            (SettingId::AutoPowerOff, "10m".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn test_with_packet_from_github_issue_149_modified_to_disable_tws() {
        let device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3959,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 0, 5, 6, 255, 255, 48, 49, 46, 54, 52, 48, 49, 46, 54, 52, 51, 57, 53,
                        57, 68, 69, 68, 54, 54, 57, 50, 68, 66, 54, 70, 52, 254, 254, 101, 120,
                        161, 171, 171, 152, 144, 179, 120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
                        241, 240, 102, 102, 242, 243, 68, 68, 51, 0, 85, 0, 0, 1, 255, 1, 49, 1, 1,
                        0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
        )
        .await;

        device.assert_setting_values([
            (SettingId::LeftSinglePress, Value::OptionalString(None)),
            (SettingId::LeftDoublePress, Some("PlayPause").into()),
            (SettingId::LeftTriplePress, Value::OptionalString(None)),
            (SettingId::LeftLongPress, Some("AmbientSoundMode").into()),
            (SettingId::RightSinglePress, Value::OptionalString(None)),
            (SettingId::RightDoublePress, Some("PlayPause").into()),
            (SettingId::RightTriplePress, Value::OptionalString(None)),
            (SettingId::RightLongPress, Some("AmbientSoundMode").into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn test_with_other_packet_from_github_issue_149() {
        // assert that it successfully connects (it will panic otherwise)
        let _device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3959,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 1, 6, 6, 255, 255, 48, 49, 46, 54, 53, 48, 49, 46, 54, 53, 51, 57, 53,
                        57, 57, 48, 49, 66, 69, 55, 50, 67, 57, 67, 49, 56, 14, 0, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 255,
                        255, 99, 102, 255, 255, 68, 68, 55, 0, 85, 0, 0, 1, 255, 1, 49, 1, 1, 1, 1,
                        2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
        )
        .await;
    }

    #[tokio::test(start_paused = true)]
    async fn test_set_multiple_button_actions() {
        let mut device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3959,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 5, 6, 255, 255, 48, 49, 46, 54, 52, 48, 49, 46, 54, 52, 51, 57, 53,
                        57, 68, 69, 68, 54, 54, 57, 50, 68, 66, 54, 70, 52, 254, 254, 101, 120,
                        161, 171, 171, 152, 144, 179, 120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
                        241, 240, 102, 102, 242, 243, 68, 68, 51, 0, 85, 0, 0, 1, 255, 1, 49, 1, 1,
                        0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
        )
        .await;
        device
            .assert_set_settings_response_unordered(
                vec![
                    (SettingId::LeftSinglePress, "VolumeUp".into()),
                    (SettingId::RightSinglePress, "VolumeDown".into()),
                ],
                vec![
                    packet::Outbound::new(packet::Command([0x04, 0x81]), vec![0, 2, 0xF0]),
                    packet::Outbound::new(packet::Command([0x04, 0x81]), vec![1, 2, 0xF1]),
                ],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn test_set_equalizer_configuration() {
        let mut device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3959,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 0, 9, 255, 255, 48, 49, 46, 54, 52, 48, 49, 46, 54, 52, 51, 57, 53,
                        57, 68, 69, 68, 54, 54, 57, 50, 68, 66, 54, 70, 52, 254, 254, 101, 120,
                        161, 171, 171, 152, 144, 60, 120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
                        241, 240, 102, 102, 242, 243, 68, 68, 51, 0, 0x55, 0, 0, 1, 255, 1, 49, 1,
                        1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(
                    SettingId::VolumeAdjustments,
                    Value::I16Vec(vec![-19, 0, 41, 51, 51, 32, 13, -35]),
                )],
                vec![packet::Outbound::new(
                    packet::Command([0x2, 0x83]),
                    vec![
                        254, 254, 101, 120, 161, 171, 171, 152, 133, 85, 120, 120, 118, 119, 124,
                        123, 124, 121, 123, 114, 120, 120,
                    ],
                )],
            )
            .await;
    }
}
