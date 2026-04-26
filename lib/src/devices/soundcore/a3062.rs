use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3062::{packets::inbound::A3062StateUpdatePacket, state::A3062State},
        common::{
            self,
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
    A3062State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3062State, A3062StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3062_sound_modes();

        builder.a3062_equalizer().await;

        builder.a3062_button_configuration();
        builder.ambient_sound_mode_cycle();

        builder.limit_high_volume();

        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);

        builder.a3062_dolby_audio();
        builder.low_battery_prompt();
        builder.a3062_side_tone();
        builder.a3062_ambient_sound_mode_voice_prompt();

        builder.single_battery_custom(
            common::modules::single_battery::SingleBatteryConfiguration {
                max_level: 10,
                level_offset: 1,
            },
        );
        builder.serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3062StateUpdatePacket::default().to_packet(),
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
    async fn test_with_packet_from_github_issue_194() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3062,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        4, 255, 48, 51, 46, 51, 55, 51, 48, 54, 50, 68, 66, 50, 49, 50, 67, 49, 51,
                        69, 57, 55, 67, 5, 0, 90, 140, 160, 160, 150, 140, 120, 100, 120, 0, 30,
                        255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 4, 4, 7, 3, 1, 80, 1, 1,
                        0, 5, 49, 1, 1, 0, 1, 1, 1, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::FirmwareVersion, "03.37".into()),
            (SettingId::SerialNumber, "3062DB212C13E97C".into()),
            (SettingId::PresetEqualizerProfile, Some("Podcast").into()),
            (SettingId::BatteryLevel, "5/10".into()),
            (SettingId::AmbientSoundMode, "Transparency".into()),
            (SettingId::ManualTransparency, 5.into()),
            (SettingId::NoiseCancelingMode, "Adaptive".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::WindNoiseSuppression, false.into()),
            (SettingId::NoiseCancelingModeInCycle, true.into()),
            (SettingId::TransparencyModeInCycle, true.into()),
            (SettingId::NormalModeInCycle, false.into()),
            (SettingId::DoublePress, Some("BassUp").into()),
            (SettingId::AutoPowerOff, "60m".into()),
            (SettingId::LimitHighVolume, false.into()),
            (SettingId::LimitHighVolumeRefreshRate, "RealTime".into()),
            (SettingId::LimitHighVolumeDbLimit, 90.into()),
            (SettingId::SideTone, false.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::VoicePrompt, true.into()),
            (SettingId::DolbyAudio, true.into()),
        ]);
    }
}
