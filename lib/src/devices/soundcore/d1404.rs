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
    d1404::{packets::inbound::D1404StateUpdatePacket, state::D1404State},
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    D1404State,
    async |packet_io| {
        let state_update_packet: D1404StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(D1404State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.d1404_sound_modes();

        builder.d1404_equalizer().await;

        builder.d1404_button_configuration();
        builder.ambient_sound_mode_cycle();

        builder.d1404_ldac_and_three_d_sound();
        builder.dual_connections();
        builder.low_battery_prompt();
        builder.d1404_ambient_sound_mode_voice_prompt();
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::thirty_sixty_one_eighty_three_hundred(),
        );

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
            D1404StateUpdatePacket::default().to_packet(),
        )])
    },
);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        api::settings::Value,
        devices::soundcore::{
            common::{
                device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
                packet::{self, inbound::TryToPacket, outbound::ToPacket},
            },
        },
        settings::SettingId,
    };

    /// Firmware 02.40, Soundcore signature preset, LDAC on, noise canceling level 2,
    /// normal mode, 1x press cycle noise canceling + normal, auto power off 1 hour.
    const SIGNATURE_PACKET: [u8; 133] = [
        5, 0, 48, 50, 46, 52, 48, 49, 52, 48, 52, 48,
        48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 3, 1, 0, 0, 64, 64, 0, 0, 240,
        65, 154, 153, 153, 62, 1, 0, 0, 128, 63, 0, 0,
        72, 66, 0, 0, 128, 63, 2, 0, 0, 192, 63, 0,
        96, 234, 69, 51, 51, 51, 63, 6, 5, 7, 255, 5,
        15, 2, 32, 0, 0, 0, 0, 49, 0, 1, 1, 1,
        0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0,
        1,
    ];

    /// The same device with a custom equalizer profile set to +6, -6, +6, -6, +6, -6, +6, -6 dB.
    /// Custom profiles extend the packet with the 8 filter definitions of the profile.
    const CUSTOM_EQ_PACKET: [u8; 198] = [
        3, 0, 48, 50, 46, 52, 48, 49, 52, 48, 52, 48,
        48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 254,
        254, 0, 0, 192, 64, 0, 0, 192, 192, 0, 0, 192,
        64, 0, 0, 192, 192, 0, 0, 192, 64, 0, 0, 192,
        192, 0, 0, 192, 64, 0, 0, 192, 192, 0, 0, 0,
        0, 0, 0, 8, 1, 0, 0, 192, 64, 0, 0, 200,
        66, 51, 51, 51, 63, 1, 0, 0, 192, 192, 0, 0,
        72, 67, 51, 51, 51, 63, 1, 0, 0, 192, 64, 0,
        0, 200, 67, 51, 51, 51, 63, 1, 0, 0, 192, 192,
        0, 0, 72, 68, 51, 51, 51, 63, 1, 0, 0, 192,
        64, 0, 0, 200, 68, 51, 51, 51, 63, 1, 0, 0,
        192, 192, 0, 0, 72, 69, 51, 51, 51, 63, 1, 0,
        0, 192, 64, 0, 0, 200, 69, 51, 51, 51, 63, 1,
        0, 0, 192, 192, 0, 0, 72, 70, 51, 51, 51, 63,
        6, 5, 7, 255, 5, 15, 2, 32, 0, 0, 0, 0,
        49, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0,
        0, 0, 1, 1, 0, 1,
    ];

    /// 3D sound on. The app turns LDAC off first, so this capture has LDAC and multipoint
    /// off, noise canceling at level 3, and byte 116 set, which is the 3D sound flag.
    const THREE_D_SOUND_PACKET: [u8; 133] = [
        3, 0, 48, 50, 46, 52, 48, 49, 52, 48, 52, 48,
        48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 3, 1, 0, 0, 64, 64, 0, 0, 240,
        65, 154, 153, 153, 62, 1, 0, 0, 128, 63, 0, 0,
        72, 66, 0, 0, 128, 63, 2, 0, 0, 192, 63, 0,
        96, 234, 69, 51, 51, 51, 63, 6, 5, 7, 255, 5,
        15, 0, 48, 0, 0, 0, 0, 49, 1, 1, 1, 0,
        0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0,
        1,
    ];

    /// After LDAC was switched off the device restarts into multipoint mode, which reports
    /// LDAC off and multipoint on.
    const SBC_MODE_PACKET: [u8; 133] = [
        3, 0, 48, 50, 46, 52, 48, 49, 52, 48, 52, 48,
        48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 3, 1, 0, 0, 64, 64, 0, 0, 240,
        65, 154, 153, 153, 62, 1, 0, 0, 128, 63, 0, 0,
        72, 66, 0, 0, 128, 63, 2, 0, 0, 192, 63, 0,
        96, 234, 69, 51, 51, 51, 63, 6, 5, 7, 255, 5,
        15, 0, 32, 0, 0, 0, 0, 49, 0, 1, 1, 0,
        1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0,
        1,
    ];

    async fn new_device(state_update_body: &[u8]) -> TestSoundcoreDevice {
        TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1404,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(packet::Command([1, 1]), state_update_body.to_vec()),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await
    }

    #[tokio::test(start_paused = true)]
    async fn parses_signature_preset_packet() {
        let device = new_device(&SIGNATURE_PACKET).await;

        device.assert_setting_values([
            (SettingId::FirmwareVersion, "02.40".into()),
            (SettingId::SerialNumber, "1404000000000000".into()),
            (SettingId::BatteryLevel, "6/10".into()),
            (
                SettingId::PresetEqualizerProfile,
                Some("SoundcoreSignature").into(),
            ),
            (
                SettingId::VolumeAdjustments,
                Value::I16Vec(vec![0, 0, 0, 0, 0, 0, 0, 0]),
            ),
            (SettingId::AmbientSoundMode, "Normal".into()),
            (SettingId::ManualNoiseCanceling, 2.into()),
            (SettingId::WindNoiseSuppression, false.into()),
            (SettingId::NoiseCancelingModeInCycle, true.into()),
            (SettingId::TransparencyModeInCycle, false.into()),
            (SettingId::NormalModeInCycle, true.into()),
            (SettingId::DoublePress, Some("BassUp").into()),
            (SettingId::MfbLongPress, None::<&str>.into()),
            (SettingId::Ldac, true.into()),
            (SettingId::SurroundSound, false.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::VoicePrompt, true.into()),
            (SettingId::AutoPowerOff, "60m".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn parses_custom_equalizer_packet() {
        let device = new_device(&CUSTOM_EQ_PACKET).await;

        device.assert_setting_values([
            (SettingId::BatteryLevel, "4/10".into()),
            (SettingId::PresetEqualizerProfile, None::<&str>.into()),
            (
                SettingId::VolumeAdjustments,
                Value::I16Vec(vec![60, -60, 60, -60, 60, -60, 60, -60]),
            ),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn parses_packet_after_ldac_was_switched_off() {
        let device = new_device(&SBC_MODE_PACKET).await;

        device.assert_setting_values([
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::Ldac, false.into()),
            (SettingId::DualConnections, true.into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn parses_three_d_sound_packet() {
        let device = new_device(&THREE_D_SOUND_PACKET).await;

        device.assert_setting_values([
            (SettingId::SurroundSound, true.into()),
            (SettingId::Ldac, false.into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_three_d_sound_turns_off_ldac() {
        let mut device = new_device(&SIGNATURE_PACKET).await;

        // With LDAC on, the official app first sends the LDAC off command, then the
        // 3D sound command.
        device
            .assert_set_settings_response(
                vec![(SettingId::SurroundSound, true.into())],
                vec![
                    packet::Outbound::new(packet::Command([1, 0xff]), vec![0]),
                    packet::Outbound::new(packet::Command([0x10, 0x81]), vec![1, 0, 0]),
                ],
            )
            .await;

        device.assert_setting_values([
            (SettingId::SurroundSound, true.into()),
            (SettingId::Ldac, false.into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn turning_three_d_sound_off_sends_only_three_d_sound() {
        let mut device = new_device(&THREE_D_SOUND_PACKET).await;

        device
            .assert_set_settings_response(
                vec![(SettingId::SurroundSound, false.into())],
                vec![packet::Outbound::new(packet::Command([0x10, 0x81]), vec![0, 0, 0])],
            )
            .await;

        device.assert_setting_values([(SettingId::SurroundSound, false.into())]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_ldac_turns_off_three_d_sound() {
        let mut device = new_device(&THREE_D_SOUND_PACKET).await;

        device
            .assert_set_settings_response(
                vec![(SettingId::Ldac, true.into())],
                vec![packet::Outbound::new(packet::Command([1, 0xff]), vec![1])],
            )
            .await;

        device.assert_setting_values([
            (SettingId::Ldac, true.into()),
            (SettingId::SurroundSound, false.into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn setting_ldac_and_three_d_sound_on_together_only_sends_three_d_sound() {
        // Same as the 3D sound packet, but with the 3D sound flag off, so both are off.
        let mut state_update_body = THREE_D_SOUND_PACKET;
        state_update_body[116] = 0;
        let mut device = new_device(&state_update_body).await;

        device
            .assert_set_settings_response(
                vec![
                    (SettingId::Ldac, true.into()),
                    (SettingId::SurroundSound, true.into()),
                ],
                vec![packet::Outbound::new(packet::Command([0x10, 0x81]), vec![1, 0, 0])],
            )
            .await;

        device.assert_setting_values([
            (SettingId::SurroundSound, true.into()),
            (SettingId::Ldac, false.into()),
        ]);
    }

    #[test]
    fn three_d_sound_packet_round_trips() {
        let packet: super::packets::inbound::D1404StateUpdatePacket = packet::Inbound::new(
            packet::inbound::STATE_COMMAND,
            THREE_D_SOUND_PACKET.to_vec(),
        )
        .try_to_packet()
        .expect("3d sound packet should parse");

        assert_eq!(packet.body(), THREE_D_SOUND_PACKET.to_vec());
    }

    #[tokio::test(start_paused = true)]
    async fn set_dual_connections() {
        let mut device = new_device(&THREE_D_SOUND_PACKET).await;

        device
            .assert_set_settings_response(
                vec![(SettingId::DualConnections, true.into())],
                vec![packet::Outbound::new(packet::Command([0x0b, 0x84]), vec![1])],
            )
            .await;

        device.assert_setting_values([(SettingId::DualConnections, true.into())]);
    }

    #[test]
    fn default_packet_round_trips() {
        let default_packet = super::packets::inbound::D1404StateUpdatePacket::default();

        let parsed_packet: super::packets::inbound::D1404StateUpdatePacket =
            packet::Inbound::new(packet::inbound::STATE_COMMAND, default_packet.body())
                .try_to_packet()
                .expect("default packet body should parse");

        assert_eq!(default_packet, parsed_packet);
    }
}
