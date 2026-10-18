use std::collections::HashMap;

use crate::devices::soundcore::{
    common::{
        self,
        macros::soundcore_device,
        packet::{
            inbound::TryToPacket,
            outbound::{RequestState, ToPacket},
        },
    },
    d1402::{packets::inbound::D1402StateUpdatePacket, state::D1402State},
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    D1402State,
    async |packet_io| {
        let state_update_packet: D1402StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(D1402State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.d1402_sound_modes();

        builder.d1402_equalizer().await;

        builder.d1402_button_configuration();
        builder.ambient_sound_mode_cycle();

        builder.limit_high_volume();

        builder.dual_connections();

        builder.d1402_ldac_and_three_d_sound();
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::thirty_sixty_one_eighty_three_hundred(),
        );
        builder.auto_play_pause();
        builder.low_battery_prompt();
        builder.d1402_ambient_sound_mode_voice_prompt();

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
            D1402StateUpdatePacket::default().to_packet(),
        )])
    },
);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        devices::soundcore::{
            common::{
                device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
                packet::{self, inbound::TryToPacket, outbound::ToPacket},
            },
            d1402::packets::inbound::D1402StateUpdatePacket,
        },
        settings::SettingId,
    };

    /// GitHub issue 313, firmware 02.59
    const ISSUE_PACKET_1: [u8; 103] = [
        2, 0, 48, 50, 46, 53, 57, 49, 52, 48, 50, 56, 52, 57, 68, 52, 66, 56, 55, 48, 54, 65, 56,
        1, 0, 160, 130, 140, 140, 160, 160, 160, 140, 120, 0, 30, 255, 0, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0,
        6, 4, 7, 255, 3, 5, 0, 95, 255, 0, 0, 1, 49, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 90, 0, 0, 1, 0,
        1, 0, 0, 0, 0, 255, 255, 255, 255, 255,
    ];

    /// GitHub issue 313, firmware 01.59, with a real hear id
    const ISSUE_PACKET_2: [u8; 103] = [
        4, 0, 48, 49, 46, 53, 57, 49, 52, 48, 50, 56, 52, 57, 68, 52, 66, 66, 48, 66, 69, 53, 65,
        0, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 30, 3, 0, 125, 135, 150, 150, 137,
        119, 109, 103, 60, 60, 255, 255, 255, 255, 1, 125, 125, 124, 125, 127, 126, 126, 121, 60,
        60, 0, 0, 6, 4, 15, 255, 3, 5, 0, 95, 255, 0, 0, 1, 49, 0, 0, 0, 1, 1, 0, 1, 1, 3, 1, 90,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255,
    ];

    async fn new_device(state_update_body: &[u8]) -> TestSoundcoreDevice {
        TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1402,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(packet::Command([1, 1]), state_update_body.to_vec()),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await
    }

    #[tokio::test(start_paused = true)]
    async fn parses_packet_from_github_issue_313_firmware_02_59() {
        let device = new_device(&ISSUE_PACKET_1).await;

        device.assert_setting_values([
            (SettingId::FirmwareVersion, "02.59".into()),
            (SettingId::SerialNumber, "1402849D4B8706A8".into()),
            (SettingId::PresetEqualizerProfile, Some("Acoustic").into()),
            (SettingId::BatteryLevel, "3/10".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::NoiseCancelingModeInCycle, true.into()),
            (SettingId::TransparencyModeInCycle, true.into()),
            (SettingId::NormalModeInCycle, false.into()),
            (SettingId::DoublePress, Some("BassUp").into()),
            (SettingId::AutoPowerOff, "30m".into()),
            (SettingId::LimitHighVolume, false.into()),
            (SettingId::LimitHighVolumeRefreshRate, "RealTime".into()),
            (SettingId::LimitHighVolumeDbLimit, 90.into()),
            (SettingId::Ldac, true.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::VoicePrompt, true.into()),
            (SettingId::SurroundSound, false.into()),
            (SettingId::DualConnections, true.into()),
            (SettingId::AutoPlayPause, true.into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn parses_packet_from_github_issue_313_firmware_01_59() {
        let device = new_device(&ISSUE_PACKET_2).await;

        device.assert_setting_values([
            (SettingId::FirmwareVersion, "01.59".into()),
            (SettingId::SerialNumber, "1402849D4BB0BE5A".into()),
            (
                SettingId::PresetEqualizerProfile,
                Some("SoundcoreSignature").into(),
            ),
            (SettingId::BatteryLevel, "5/10".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::NoiseCancelingModeInCycle, true.into()),
            (SettingId::TransparencyModeInCycle, true.into()),
            (SettingId::NormalModeInCycle, false.into()),
            (SettingId::DoublePress, None::<&str>.into()),
            (SettingId::AutoPowerOff, "300m".into()),
            (SettingId::LimitHighVolume, true.into()),
            (SettingId::LimitHighVolumeRefreshRate, "RealTime".into()),
            (SettingId::LimitHighVolumeDbLimit, 90.into()),
            (SettingId::Ldac, false.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::VoicePrompt, true.into()),
            (SettingId::SurroundSound, false.into()),
            (SettingId::DualConnections, true.into()),
            (SettingId::AutoPlayPause, false.into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_ambient_sound_mode_preserves_unknown_bytes() {
        let mut device = new_device(&ISSUE_PACKET_1).await;

        device
            .assert_set_settings_response(
                vec![(SettingId::AmbientSoundMode, "Transparency".into())],
                vec![packet::Outbound::new(
                    packet::Command([6, 129]),
                    vec![1, 0x5F, 255, 0, 0, 1],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_noise_canceling_level_while_in_noise_canceling() {
        let mut device = new_device(&ISSUE_PACKET_1).await;

        device
            .assert_set_settings_response(
                vec![(SettingId::ManualNoiseCanceling, 3.into())],
                vec![packet::Outbound::new(
                    packet::Command([6, 129]),
                    vec![0, 0x3F, 255, 0, 0, 1],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn switching_to_noise_canceling_and_changing_level_takes_two_packets() {
        let mut state_update_body = ISSUE_PACKET_1;
        state_update_body[71] = 1; // transparency
        let mut device = new_device(&state_update_body).await;

        device
            .assert_set_settings_response(
                vec![
                    (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
                    (SettingId::ManualNoiseCanceling, 3.into()),
                ],
                vec![
                    packet::Outbound::new(packet::Command([6, 129]), vec![0, 0x5F, 255, 0, 0, 1]),
                    packet::Outbound::new(packet::Command([6, 129]), vec![0, 0x3F, 255, 0, 0, 1]),
                ],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn changing_noise_canceling_level_while_in_transparency_goes_through_noise_canceling() {
        let mut state_update_body = ISSUE_PACKET_1;
        state_update_body[71] = 1; // transparency
        let mut device = new_device(&state_update_body).await;

        device
            .assert_set_settings_response(
                vec![(SettingId::ManualNoiseCanceling, 3.into())],
                vec![
                    packet::Outbound::new(packet::Command([6, 129]), vec![0, 0x5F, 255, 0, 0, 1]),
                    packet::Outbound::new(packet::Command([6, 129]), vec![0, 0x3F, 255, 0, 0, 1]),
                    packet::Outbound::new(packet::Command([6, 129]), vec![1, 0x3F, 255, 0, 0, 1]),
                ],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_equalizer_preserves_unknown_bytes_and_hear_id() {
        let mut device = new_device(&ISSUE_PACKET_2).await;

        device
            .assert_set_settings_response(
                vec![(SettingId::PresetEqualizerProfile, "Flat".into())],
                vec![packet::Outbound::new(
                    packet::Command([3, 135]),
                    vec![
                        9, 0, // preset
                        0, 0, // hear id favorite music genre
                        100, 100, 110, 120, 120, 120, 100, 100, 120, 0, // volume adjustments
                        30, 3, // unknown bytes from the state update packet
                        0, // hear id enabled
                        125, 135, 150, 150, 137, 119, 109, 103, 60,
                        60, // hear id volume adjustments
                        255, 255, 255, 255, // hear id time
                        1,   // hear id type
                        125, 125, 124, 125, 127, 126, 126, 121, 60,
                        60, // hear id custom volume adjustments
                        118, 119, 119, 120, 120, 121, 118, 118, 120,
                        0, // drc volume adjustments
                        0, // unknown
                    ],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_equalizer_writes_absent_hear_id_as_the_device_stores_it() {
        let mut device = new_device(&ISSUE_PACKET_1).await;

        // The last band of each absent hear id array must be 0, not 255. 255 results in extremely
        // loud and distorted audio.
        device
            .assert_set_settings_response(
                vec![(SettingId::PresetEqualizerProfile, "Flat".into())],
                vec![packet::Outbound::new(
                    packet::Command([3, 135]),
                    vec![
                        9, 0, // preset
                        0, 0, // hear id favorite music genre
                        100, 100, 110, 120, 120, 120, 100, 100, 120, 0, // volume adjustments
                        30, 255, // unknown bytes from the state update packet
                        0,   // hear id enabled
                        255, 255, 255, 255, 255, 255, 255, 255, 255,
                        0, // hear id volume adjustments
                        0, 0, 0, 0, // hear id time
                        0, // hear id type
                        255, 255, 255, 255, 255, 255, 255, 255, 255,
                        0, // hear id custom volume adjustments
                        118, 119, 119, 120, 120, 121, 118, 118, 120,
                        0, // drc volume adjustments
                        0, // unknown
                    ],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_ldac_turns_off_three_d_sound() {
        let mut state_update_body = ISSUE_PACKET_1;
        state_update_body[78] = 1; // 3d sound on
        state_update_body[83] = 0; // ldac off
        let mut device = new_device(&state_update_body).await;

        device
            .assert_set_settings_response(
                vec![(SettingId::Ldac, true.into())],
                vec![packet::Outbound::new(packet::Command([1, 0xff]), vec![1])],
            )
            .await;

        device.assert_setting_values([
            (SettingId::SurroundSound, false.into()),
            (SettingId::Ldac, true.into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_three_d_sound_turns_off_ldac() {
        let mut device = new_device(&ISSUE_PACKET_1).await;

        device
            .assert_set_settings_response(
                vec![(SettingId::SurroundSound, true.into())],
                vec![packet::Outbound::new(packet::Command([2, 134]), vec![1])],
            )
            .await;

        device.assert_setting_values([
            (SettingId::SurroundSound, true.into()),
            (SettingId::Ldac, false.into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_flags_and_buttons() {
        let mut state_update_body = ISSUE_PACKET_1;
        state_update_body[67] = 15; // double press: none
        state_update_body[93] = 0; // auto play/pause off
        let mut device = new_device(&state_update_body).await;

        device
            .assert_set_settings_response_unordered(
                vec![
                    (SettingId::Ldac, false.into()),
                    (SettingId::LowBatteryPrompt, false.into()),
                    (SettingId::VoicePrompt, false.into()),
                    (SettingId::AutoPlayPause, true.into()),
                    (SettingId::DoublePress, Some("BassUp").into()),
                    (SettingId::AutoPowerOff, "180m".into()),
                ],
                vec![
                    packet::Outbound::new(packet::Command([1, 0xff]), vec![0]),
                    packet::Outbound::new(packet::Command([0x10, 0x82]), vec![0]),
                    packet::Outbound::new(packet::Command([1, 0xae]), vec![0]),
                    packet::Outbound::new(packet::Command([1, 0x81]), vec![1]),
                    packet::Outbound::new(packet::Command([4, 0x81]), vec![0, 0, 7]),
                    packet::Outbound::new(packet::Command([1, 0x86]), vec![1, 2]),
                ],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_auto_power_off_to_sixty_minutes() {
        let mut device = new_device(&ISSUE_PACKET_1).await;

        device
            .assert_set_settings_response(
                vec![(SettingId::AutoPowerOff, "60m".into())],
                vec![packet::Outbound::new(
                    packet::Command([1, 0x86]),
                    vec![1, 1],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_auto_power_off_to_never() {
        let mut device = new_device(&ISSUE_PACKET_1).await;

        device
            .assert_set_settings_response(
                vec![(SettingId::AutoPowerOff, "disabled".into())],
                vec![packet::Outbound::new(
                    packet::Command([1, 0x86]),
                    vec![0, 0],
                )],
            )
            .await;
    }

    #[test]
    fn state_update_packet_round_trips_through_to_packet() {
        for body in [ISSUE_PACKET_1, ISSUE_PACKET_2] {
            let parsed: D1402StateUpdatePacket =
                packet::Inbound::new(packet::Command([1, 1]), body.to_vec())
                    .try_to_packet()
                    .unwrap();
            let to_packet_body = parsed.body();
            assert_eq!(to_packet_body.len(), body.len());
            // the unknown bytes aren't preserved, but everything through the hear id is
            assert_eq!(to_packet_body[..65], body[..65]);
            let reparsed: D1402StateUpdatePacket =
                packet::Inbound::new(packet::Command([1, 1]), to_packet_body)
                    .try_to_packet()
                    .unwrap();
            assert_eq!(parsed, reparsed);
        }
    }
}
