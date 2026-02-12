use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::common::{
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
    i18n::fl,
};

mod modules;
mod packets;
mod state;
pub(crate) mod structures;

soundcore_device!(
    state::A3957State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<
            _,
            state::A3957State,
            packets::inbound::A3957StateUpdatePacket,
        >(packet_io)
        .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3957_sound_modes();
        builder
            .equalizer_with_custom_hear_id_tws(equalizer::common_settings())
            .await;
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.ambient_sound_mode_cycle();
        builder.reset_button_configuration::<packets::inbound::A3957StateUpdatePacket>(
            RequestState::default().to_packet(),
        );

        builder.limit_high_volume();

        builder.a3957_immersive_experience();
        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);
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
            packets::inbound::A3957StateUpdatePacket::default().to_packet(),
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
        devices::soundcore::common::{
            device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
            packet,
        },
        settings::SettingId,
    };

    #[tokio::test(start_paused = true)]
    async fn test_with_liberty5_packet_from_issue_226() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3957,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0x00, 0x01, 0x08, 0x08, 0x00, 0x00, 0x30, 0x33, 0x2e, 0x39, 0x30, 0x30,
                        0x33, 0x2e, 0x39, 0x30, 0x33, 0x39, 0x35, 0x37, 0x46, 0x34, 0x39, 0x44,
                        0x38, 0x41, 0x43, 0x32, 0x30, 0x33, 0x33, 0x34, 0x30, 0x30, 0x2e, 0x30,
                        0x30, 0x02, 0x00, 0x00, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78,
                        0x78, 0x78, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                        0x00, 0x01, 0x91, 0x82, 0x73, 0x77, 0x8b, 0x93, 0x8f, 0x96, 0x00, 0x00,
                        0x91, 0x82, 0x73, 0x77, 0x8b, 0x93, 0x8f, 0x96, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x91, 0x82, 0x73, 0x77, 0x8b, 0x93, 0x8f, 0x96, 0x00,
                        0x00, 0x91, 0x82, 0x73, 0x77, 0x8b, 0x93, 0x8f, 0x96, 0x00, 0x00, 0x00,
                        0x00, 0x0a, 0x66, 0x66, 0x32, 0x33, 0xff, 0xff, 0x44, 0x44, 0x33, 0x00,
                        0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x01, 0x01, 0x00, 0x01, 0x01,
                        0x02, 0x00, 0x5a, 0x00, 0x00, 0x01, 0x00, 0x00, 0xff, 0x01, 0x00, 0x00,
                        0x01, 0x01, 0x00, 0x01, 0xff, 0xff, 0xff, 0x03, 0xff, 0xff,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::FirmwareVersionLeft, "03.90".into()),
            (SettingId::FirmwareVersionRight, "03.90".into()),
            (SettingId::SerialNumber, "3957F49D8AC20334".into()),
            (SettingId::PresetEqualizerProfile, Some("SoundcoreSignature").into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::TouchTone, true.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::ImmersiveExperience, "Disabled".into()),
            (SettingId::AncPersonalizedToEarCanal, true.into()),
        ]);
    }
}
