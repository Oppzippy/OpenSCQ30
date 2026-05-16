use std::collections::HashMap;

use crate::devices::soundcore::{
    a3952::{packets::inbound::A3952StateUpdatePacket, state::A3952State},
    common::{
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
    },
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3952State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3952State, A3952StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.a3952_sound_modes();

        builder
            .equalizer_with_custom_hear_id_tws(equalizer::common_settings())
            .await;

        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.ambient_sound_mode_cycle();
        builder.reset_button_configuration::<packets::inbound::A3952StateUpdatePacket>(
            RequestState::default().to_packet(),
        );

        builder.ldac();
        builder.touch_tone();
        builder.wearing_detection();
        builder.wearing_tone();

        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::half_hour_increments(),
        );

        builder.tws_status();
        builder.dual_battery(5);
        builder.case_battery_level(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3952StateUpdatePacket::default().to_packet(),
        )])
    },
);

const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<6, 3> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: false,
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
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::DisablingOneSideDisablesOther,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::DisablingOneSideDisablesOther,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::DisablingOneSideDisablesOther,
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
        settings::{self, SettingId},
    };

    #[tokio::test(start_paused = true)]
    async fn test_with_packet_from_github_issue_134() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3959,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 4, 4, 0, 0, 48, 52, 46, 57, 54, 48, 52, 46, 57, 54, 51, 57, 53, 50,
                        50, 48, 54, 54, 51, 49, 67, 67, 69, 69, 69, 56, 0, 0, 60, 60, 60, 60, 60,
                        60, 60, 60, 60, 60, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 2, 1,
                        120, 150, 180, 180, 120, 120, 120, 180, 0, 0, 120, 180, 180, 180, 120, 150,
                        180, 180, 0, 0, 101, 86, 138, 244, 0, 120, 150, 180, 180, 120, 120, 120,
                        180, 0, 0, 120, 180, 180, 180, 120, 150, 180, 180, 0, 0, 0, 0, 18, 0, 97,
                        0, 96, 17, 99, 17, 102, 17, 84, 17, 84, 0, 65, 0, 64, 3, 2, 48, 0, 0, 1, 1,
                        1, 1, 0, 4, 0, 0, 0, 0, 1, 0, 0, 1, 0, 58, 255, 255, 255, 255, 255, 255,
                        101, 86, 137, 249, 70, 9,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "4/5".into()),
            (SettingId::BatteryLevelRight, "4/5".into()),
            (SettingId::CaseBatteryLevel, "4/5".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::AmbientSoundMode, "Normal".into()),
            (SettingId::NoiseCancelingMode, "Manual".into()),
            (SettingId::AdaptiveNoiseCanceling, "LowNoise".into()),
            (SettingId::ManualNoiseCanceling, "Strong".into()),
            (SettingId::TransparencyMode, "FullyTransparent".into()),
            (SettingId::WindNoiseSuppression, true.into()),
            (
                SettingId::LeftSinglePress,
                settings::Value::OptionalString(None).into(),
            ),
            (
                SettingId::RightSinglePress,
                settings::Value::OptionalString(None).into(),
            ),
            (SettingId::LeftDoublePress, Some("NextSong").into()),
            (SettingId::RightDoublePress, Some("PlayPause").into()),
            (SettingId::LeftLongPress, Some("AmbientSoundMode").into()),
            (SettingId::RightLongPress, Some("AmbientSoundMode").into()),
            (SettingId::WearingDetection, true.into()),
            (SettingId::WearingTone, false.into()),
            (SettingId::TouchTone, true.into()),
            (SettingId::AutoPowerOff, "30m".into()),
            (SettingId::FirmwareVersionLeft, "04.96".into()),
            (SettingId::FirmwareVersionRight, "04.96".into()),
            (SettingId::Ldac, false.into()),
        ]);
    }
}
