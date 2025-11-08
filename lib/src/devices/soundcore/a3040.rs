use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3040::{packets::A3040StateUpdatePacket, state::A3040State},
        common::{
            device::fetch_state_from_state_update_packet,
            macros::soundcore_device,
            packet::outbound::{RequestState, ToPacket},
        },
    },
    i18n::fl,
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3040State,
    A3040StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3040State, A3040StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.a3040_sound_modes();

        builder.a3040_equalizer().await;

        // builder.a3040_button_configuration();
        builder.ambient_sound_mode_cycle();

        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);

        builder.limit_high_volume();

        // builder.single_battery_level();
        builder.serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3040StateUpdatePacket::default().to_packet().bytes(),
        )])
    },
);

#[derive(IntoStaticStr, VariantArray)]
#[allow(clippy::enum_variant_names)]
enum AutoPowerOffDuration {
    #[strum(serialize = "30m")]
    ThirtyMinutes,
    #[strum(serialize = "60m")]
    SixtyMinutes,
    #[strum(serialize = "90m")]
    NinetyMinutes,
    #[strum(serialize = "120m")]
    OneHundredTwentyMinutes,
}

impl Translate for AutoPowerOffDuration {
    fn translate(&self) -> String {
        match self {
            Self::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            Self::SixtyMinutes => fl!("x-minutes", minutes = 60),
            Self::NinetyMinutes => fl!("x-minutes", minutes = 90),
            Self::OneHundredTwentyMinutes => fl!("x-minutes", minutes = 120),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        devices::soundcore::common::{device::test_utils::TestSoundcoreDevice, packet},
        settings::{self, SettingId},
    };

    #[tokio::test(start_paused = true)]
    async fn test_with_known_good_packet() {
        let device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3040,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        3, 255, 48, 50, 46, 51, 52, 51, 48, 52, 48, 49, 65, 49, 65, 50, 70, 67, 67,
                        69, 69, 69, 56, 17, 0, 150, 140, 110, 110, 130, 150, 160, 170, 120, 0, 122,
                        122, 118, 119, 121, 122, 122, 126, 120, 0, 4, 4, 7, 7, 0, 0x50, 0, 1, 0, 5,
                        255, 0, 0, 255, 0, 49, 0, 1, 255, 0, 1, 0, 0, 90, 0, 1, 1, 1, 0, 17, 50, 1,
                        1, 143, 143, 128, 138, 150, 150, 143, 143, 60, 60, 143, 143, 128, 138, 150,
                        150, 143, 143, 60, 60, 100, 152, 140, 205, 0, 143, 143, 128, 138, 150, 150,
                        143, 143, 60, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 122, 119, 121, 122,
                        122, 121, 123, 114, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
        )
        .await;

        device.assert_setting_values([
            // (SettingId::BatteryLevel, "3".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::NoiseCancelingMode, "Adaptive".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::TransparencyMode, "TalkMode".into()),
            (SettingId::ManualTransparency, 5.into()),
            (SettingId::WindNoiseSuppression, false.into()),
            (SettingId::LimitHighVolume, false.into()),
            (SettingId::LimitHighVolumeDbLimit, 90.into()),
            (SettingId::LimitHighVolumeRefreshRate, "RealTime".into()),
            (SettingId::NoiseCancelingModeInCycle, true.into()),
            (SettingId::TransparencyModeInCycle, true.into()),
            (SettingId::NormalModeInCycle, true.into()),
            (SettingId::AutoPowerOff, "30m".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_ambient_sound_mode() {
        let mut device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3040,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        3, 255, 48, 50, 46, 51, 52, 51, 48, 52, 48, 49, 65, 49, 65, 50, 70, 67, 67,
                        69, 69, 69, 56, 17, 0, 150, 140, 110, 110, 130, 150, 160, 170, 120, 0, 122,
                        122, 118, 119, 121, 122, 122, 126, 120, 0, 4, 4, 7, 7, 0, 0x50, 0, 1, 0, 5,
                        255, 0, 0, 255, 0, 49, 0, 1, 255, 0, 1, 0, 0, 90, 0, 1, 1, 1, 0, 17, 50, 1,
                        1, 143, 143, 128, 138, 150, 150, 143, 143, 60, 60, 143, 143, 128, 138, 150,
                        150, 143, 143, 60, 60, 100, 152, 140, 205, 0, 143, 143, 128, 138, 150, 150,
                        143, 143, 60, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 122, 119, 121, 122,
                        122, 121, 123, 114, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(SettingId::AmbientSoundMode, "Normal".into())],
                vec![packet::Outbound::new(
                    packet::Command([6, 129]),
                    vec![2, 80, 0, 1, 0, 5],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_equalizer_soundcore_signature() {
        let mut device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3040,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        3, 255, 48, 50, 46, 51, 52, 51, 48, 52, 48, 49, 65, 49, 65, 50, 70, 67, 67,
                        69, 69, 69, 56, 17, 0, 150, 140, 110, 110, 130, 150, 160, 170, 120, 0, 122,
                        122, 118, 119, 121, 122, 122, 126, 120, 0, 4, 4, 7, 7, 0, 0x50, 0, 1, 0, 5,
                        255, 0, 0, 255, 0, 49, 0, 1, 255, 0, 1, 0, 0, 90, 0, 1, 1, 1, 0, 17, 50, 1,
                        1, 143, 143, 128, 138, 150, 150, 143, 143, 60, 60, 143, 143, 128, 138, 150,
                        150, 143, 143, 60, 60, 100, 152, 140, 205, 0, 143, 143, 128, 138, 150, 150,
                        143, 143, 60, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 122, 119, 121, 122,
                        122, 121, 123, 114, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(
                    SettingId::PresetEqualizerProfile,
                    "SoundcoreSignature".into(),
                )],
                vec![packet::Outbound::new(
                    packet::Command([3, 135]),
                    vec![
                        0, 0, 17, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 120, 120, 120,
                        120, 120, 120, 120, 120, 120, 0, 0, 1, 0, 143, 143, 128, 138, 150, 150,
                        143, 143, 60, 60, 143, 143, 128, 138, 150, 150, 143, 143, 60, 60, 100, 152,
                        140, 205, 0, 143, 143, 128, 138, 150, 150, 143, 143, 60, 60, 143, 143, 128,
                        138, 150, 150, 143, 143, 60, 60, 120, 120, 120, 120, 120, 120, 120, 120,
                        120, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 0,
                    ],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_equalizer_custom() {
        let mut device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3040,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        3, 255, 48, 50, 46, 51, 52, 51, 48, 52, 48, 49, 65, 49, 65, 50, 70, 67, 67,
                        69, 69, 69, 56, 17, 0, 150, 140, 110, 110, 130, 150, 160, 170, 120, 0, 122,
                        122, 118, 119, 121, 122, 122, 126, 120, 0, 4, 4, 7, 7, 0, 0x50, 0, 1, 0, 5,
                        255, 0, 0, 255, 0, 49, 0, 1, 255, 0, 1, 0, 0, 90, 0, 1, 1, 1, 0, 17, 50, 1,
                        1, 143, 143, 128, 138, 150, 150, 143, 143, 60, 60, 143, 143, 128, 138, 150,
                        150, 143, 143, 60, 60, 100, 152, 140, 205, 0, 143, 143, 128, 138, 150, 150,
                        143, 143, 60, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 122, 119, 121, 122,
                        122, 121, 123, 114, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(
                    SettingId::VolumeAdjustments,
                    settings::Value::I16Vec(vec![60, 0, 0, 0, 0, 0, 0, -60]),
                )],
                vec![packet::Outbound::new(
                    packet::Command([3, 135]),
                    vec![
                        254, 254, 17, 0, 180, 120, 120, 120, 120, 120, 120, 60, 120, 0, 180, 120,
                        120, 120, 120, 120, 120, 60, 120, 0, 0, 1, 0, 143, 143, 128, 138, 150, 150,
                        143, 143, 60, 60, 143, 143, 128, 138, 150, 150, 143, 143, 60, 60, 100, 152,
                        140, 205, 0, 143, 143, 128, 138, 150, 150, 143, 143, 60, 60, 143, 143, 128,
                        138, 150, 150, 143, 143, 60, 60, 128, 116, 121, 119, 121, 119, 124, 111,
                        120, 0, 128, 116, 121, 119, 121, 119, 124, 111, 120, 0, 0,
                    ],
                )],
            )
            .await;
    }
}
