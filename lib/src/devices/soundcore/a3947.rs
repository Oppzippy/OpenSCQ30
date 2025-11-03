use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3947::{packets::A3947StateUpdatePacket, state::A3947State},
        common::{
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
    },
    i18n::fl,
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3947State,
    A3947StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3947State, A3947StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3947_sound_modes();
        builder.a3947_equalizer().await;

        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.reset_button_configuration::<A3947StateUpdatePacket>(
            RequestState::default().to_packet(),
        );

        builder.limit_high_volume();
        builder.touch_tone();
        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);

        builder.serial_number_and_dual_firmware_version();
        builder.tws_status();
        builder.dual_battery(5);
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3947StateUpdatePacket::default().to_packet().bytes(),
        )])
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<8, 4> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        use_enabled_flag_to_disable: false,
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
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 5,
                press_kind: ButtonPressKind::Triple,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
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
        settings::SettingId,
    };

    #[tokio::test(start_paused = true)]
    async fn test_with_known_good_packet() {
        let device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3959,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 2, 4, 0, 0, 48, 53, 46, 56, 56, 48, 53, 46, 56, 56, 51, 57, 52, 55,
                        66, 54, 54, 50, 70, 68, 67, 67, 69, 69, 69, 56, 48, 48, 46, 48, 48, 254,
                        254, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 0, 1, 112, 117, 140, 148, 150, 142, 134, 131,
                        60, 60, 112, 117, 140, 148, 150, 142, 134, 131, 60, 60, 104, 100, 34, 64,
                        0, 112, 117, 140, 148, 150, 142, 134, 131, 60, 60, 112, 117, 140, 148, 150,
                        142, 134, 131, 60, 60, 0, 0, 18, 17, 102, 17, 101, 17, 50, 17, 51, 17, 68,
                        17, 68, 0, 33, 0, 32, 0x35, 0, 0x50, 1, 0, 1, 1, 3, 0, 0, 0, 0, 255, 1, 2,
                        49, 1, 0, 0, 1, 0, 0, 1, 0, 1, 80, 1, 0, 0, 1, 1, 1, 1, 255,
                    ],
                ),
            )]),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "2".into()),
            (SettingId::BatteryLevelRight, "4".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::NoiseCancelingMode, "Manual".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::EnvironmentDetection, true.into()),
            (SettingId::TransportationMode, "Car".into()),
            (SettingId::TransparencyMode, "VocalMode".into()),
            (SettingId::LimitHighVolume, true.into()),
            (SettingId::LimitHighVolumeDbLimit, 80.into()),
            (SettingId::LimitHighVolumeRefreshRate, "10s".into()),
            // TODO the enabled flag for buttons should be entirely ignored
            // (SettingId::LeftSinglePress, Some("PlayPause").into()),
            // (SettingId::LeftDoublePress, Some("PreviousSong").into()),
            // (SettingId::LeftTriplePress, Some("VolumeDown").into()),
            // (SettingId::LeftLongPress, Some("AmbientSoundMode").into()),
            // (SettingId::RightSinglePress, Some("VoiceAssistant").into()),
            // (SettingId::RightDoublePress, Some("NextSong").into()),
            // (SettingId::RightTriplePress, Some("VolumeUp").into()),
            // (SettingId::RightLongPress, Some("AmbientSoundMode").into()),
            (SettingId::TouchTone, true.into()),
            (SettingId::AutoPowerOff, "20m".into()),
        ]);
    }
}
