use std::collections::HashMap;

use crate::devices::soundcore::{
    a3035::{packets::inbound::A3035StateUpdatePacket, state::A3035State},
    common::{
        self,
        macros::soundcore_device,
        packet::{
            inbound::TryToPacket,
            outbound::{RequestState, ToPacket},
        },
    },
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3035State,
    async |packet_io| {
        let state_update_packet: A3035StateUpdatePacket = packet_io
            .send_with_response(&RequestState::default().to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(A3035State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.a3035_sound_modes();

        builder.a3035_equalizer().await;

        builder.a3035_button_configuration();
        builder.ambient_sound_mode_cycle();

        builder.limit_high_volume();

        builder.dual_connections();

        builder.ldac();
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::half_hour_increments(),
        );
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        devices::soundcore::common::{
            device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
            packet::{self},
        },
        settings::SettingId,
    };

    #[tokio::test(start_paused = true)]
    async fn test_with_known_good_packet() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3035,
            HashMap::from([
                (
                    packet::Command([1, 1]),
                    packet::Inbound::new(
                        packet::Command([1, 1]),
                        vec![
                            5, 255, 48, 54, 46, 56, 55, 51, 48, 51, 53, 55, 48, 53, 48, 50, 56, 56,
                            65, 57, 68, 70, 52, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120,
                            0, 30, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0,
                            0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 4, 4, 7, 3,
                            0, 0x50, 0, 0, 1, 5, 0, 1, 0, 0, 0, 49, 1, 0, 1, 0, 1, 2, 0, 90, 0, 1,
                            1, 0, 0, 0,
                        ],
                    ),
                ),
                TestSoundcoreDevice::basic_dual_connections_response(),
            ]),
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
            (SettingId::DualConnections, false.into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_custom_transparency() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3035,
            HashMap::from([
                (
                    packet::Command([1, 1]),
                    packet::Inbound::new(
                        packet::Command([1, 1]),
                        vec![
                            5, 255, 48, 54, 46, 56, 55, 51, 48, 51, 53, 55, 48, 53, 48, 50, 56, 56,
                            65, 57, 68, 70, 52, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120,
                            0, 30, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0,
                            0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 4, 4, 7, 3,
                            0, 0x50, 0, 0, 1, 5, 0, 1, 0, 0, 0, 49, 1, 0, 1, 0, 1, 2, 0, 90, 0, 1,
                            1, 0, 0, 0,
                        ],
                    ),
                ),
                TestSoundcoreDevice::basic_dual_connections_response(),
            ]),
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
