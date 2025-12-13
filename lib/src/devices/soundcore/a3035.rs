use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3035::{packets::inbound::A3035StateUpdatePacket, state::A3035State},
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
    A3035State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3035State, A3035StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.a3035_sound_modes();

        builder.a3035_equalizer().await;

        builder.a3035_button_configuration();
        builder.ambient_sound_mode_cycle();

        builder.limit_high_volume();

        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);
        builder.auto_play_pause();
        builder.a3035_battery_alert();
        builder.a3035_ambient_sound_mode_voice_prompt();

        builder.single_battery_level(5);
        builder.serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3035StateUpdatePacket::default().to_packet(),
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
        devices::soundcore::common::{
            device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
            packet,
        },
        settings::SettingId,
    };

    #[tokio::test(start_paused = true)]
    async fn test_with_known_good_packet() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3035,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        5, 255, 48, 54, 46, 56, 55, 51, 48, 51, 53, 55, 48, 53, 48, 50, 56, 56, 65,
                        57, 68, 70, 52, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 30,
                        255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 4, 4, 7, 3, 0, 0x50, 0, 0,
                        1, 5, 0, 1, 0, 0, 0, 49, 1, 0, 1, 0, 1, 2, 0, 90, 0, 1, 1, 0, 0, 0,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::NoiseCancelingMode, "Custom".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::ManualTransparency, 5.into()),
            (SettingId::WindNoiseSuppression, true.into()),
            (SettingId::LimitHighVolume, false.into()),
            (SettingId::LimitHighVolumeDbLimit, 90.into()),
            (SettingId::LimitHighVolumeRefreshRate, "RealTime".into()),
            (SettingId::DoublePress, Some("BassUp").into()),
            (SettingId::NoiseCancelingModeInCycle, true.into()),
            (SettingId::TransparencyModeInCycle, true.into()),
            (SettingId::NormalModeInCycle, false.into()),
            (SettingId::AutoPowerOff, "90m".into()),
            (SettingId::AutoPlayPause, true.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::VoicePrompt, true.into()),
            (
                SettingId::PresetEqualizerProfile,
                Some("SoundcoreSignature").into(),
            ),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_custom_transparency() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3035,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        5, 255, 48, 54, 46, 56, 55, 51, 48, 51, 53, 55, 48, 53, 48, 50, 56, 56, 65,
                        57, 68, 70, 52, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 30,
                        255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 4, 4, 7, 3, 0, 0x50, 0, 0,
                        1, 5, 0, 1, 0, 0, 0, 49, 1, 0, 1, 0, 1, 2, 0, 90, 0, 1, 1, 0, 0, 0,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(SettingId::ManualTransparency, 3.into())],
                vec![
                    packet::Outbound::new(packet::Command([6, 129]), vec![1, 81, 1, 0, 1, 5]),
                    packet::Outbound::new(packet::Command([6, 129]), vec![1, 81, 1, 0, 1, 3]),
                    packet::Outbound::new(packet::Command([6, 129]), vec![0, 81, 0, 0, 1, 3]),
                ],
            )
            .await;
    }
}
