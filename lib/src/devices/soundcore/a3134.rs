use std::collections::HashMap;

use uuid::uuid;

use crate::{
    connection::RfcommServiceSelectionStrategy,
    devices::soundcore::{
        a3134::{packets::inbound::A3134StateUpdatePacket, state::A3134State},
        common::{
            device::{SoundcoreDeviceConfig, fetch_state_from_state_update_packet},
            macros::soundcore_device,
            packet::{
                self,
                outbound::{RequestState, ToPacket},
            },
        },
    },
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3134State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<A3134State, A3134StateUpdatePacket>(packet_io).await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3134_equalizer().await;
        builder.a3134_bass_up();
        builder.a3134_lights();
        builder.single_battery(5);
        builder.serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3134StateUpdatePacket::default().to_packet(),
        )])
    },
    CONFIG,
);

// The speaker also offers ...7c3134, which leaves state requests unanswered (checked on firmware 03.12).
const CONFIG: SoundcoreDeviceConfig = SoundcoreDeviceConfig {
    checksum_kind: packet::ChecksumKind::Suffix,
    rfcomm_service_selection_strategy: RfcommServiceSelectionStrategy::Constant(uuid!(
        "0cf12d31-fac3-4553-bd80-d6832e7b3134"
    )),
};

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use nom_language::error::VerboseError;

    use crate::{
        DeviceModel,
        devices::soundcore::common::{
            device::test_utils::TestSoundcoreDevice, packet::inbound::FromPacketBody,
        },
        settings::{SettingId, Value},
    };

    use super::*;

    fn state_update_body() -> Vec<u8> {
        vec![
            0, 4, 0, 0, 1, 1, 2, 48, 51, 46, 49, 50, 65, 66, 67, 68, 69, 70, 65, 66, 67, 68, 69,
            70, 65, 66, 67, 68, 0, 49, 255, 255, 255, 0, 120, 7, 10, 120, 7, 10, 120, 7, 10, 120,
            7, 10, 120, 7, 10, 120, 7, 10, 120, 7, 10, 120, 6, 10, 120, 1, 10, 0, 0, 0, 86, 3, 100,
            0, 0, 6, 255, 0, 216, 138, 0, 255, 0, 20, 255, 0, 255, 204, 255, 252, 0, 255, 58, 0, 1,
            1, 0, 178, 255, 2, 4, 255, 30, 0, 255, 122, 0, 255, 168, 0, 255, 226, 0, 3, 1, 255,
            255, 255, 4, 4, 0, 20, 255, 94, 0, 255, 138, 0, 255, 204, 0, 255, 5, 2, 255, 252, 0, 0,
            255, 41, 6, 2, 0, 20, 255, 204, 0, 255, 7, 2, 0, 173, 255, 0, 255, 204, 0, 0, 0, 0,
        ]
    }

    async fn test_device() -> TestSoundcoreDevice {
        TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3134,
            HashMap::from([(
                packet::inbound::STATE_COMMAND,
                packet::Inbound::new(packet::inbound::STATE_COMMAND, state_update_body()),
            )]),
            CONFIG,
        )
        .await
    }

    #[test]
    fn state_update_packet_round_trips() {
        let (_, packet) =
            A3134StateUpdatePacket::take::<VerboseError<_>>(&state_update_body()).unwrap();
        assert_eq!(packet.body(), state_update_body());
    }

    #[test]
    fn truncated_state_update_packet_is_rejected() {
        let mut body = state_update_body();
        body.truncate(body.len() - 8);
        A3134StateUpdatePacket::take::<VerboseError<_>>(&body).unwrap_err();
    }

    #[tokio::test(start_paused = true)]
    async fn packet_from_device_is_parsed() {
        let device = test_device().await;
        device.assert_setting_values([
            (SettingId::BatteryLevel, Cow::from("4/5").into()),
            (SettingId::IsCharging, Cow::from("No").into()),
            (SettingId::FirmwareVersion, Cow::from("03.12").into()),
            (
                SettingId::SerialNumber,
                Cow::from("ABCDEFABCDEFABCD").into(),
            ),
            (SettingId::BassUp, false.into()),
            (
                SettingId::PresetEqualizerProfile,
                Value::OptionalString(Some("SoundcoreSignature".into())),
            ),
            (
                SettingId::VolumeAdjustments,
                Value::I16Vec(vec![10, 0, 10, 0, 10, -10, 0, 15, 15]),
            ),
            (SettingId::LightsEnabled, false.into()),
            (SettingId::LightsBrightness, 100.into()),
            (SettingId::LightsMode, Cow::from("Lightning").into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_bass_up_sends_flag() {
        let mut device = test_device().await;
        device
            .assert_set_settings_response(
                vec![(SettingId::BassUp, true.into())],
                vec![packet::Outbound::new(
                    packet::Command([0x02, 0x84]),
                    vec![1],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_preset_sends_official_app_curve() {
        let mut device = test_device().await;
        let cases = [
            (
                "Voice",
                [
                    1, 1, 255, 100, 7, 10, 120, 7, 10, 140, 7, 10, 130, 7, 10, 140, 7, 10, 140, 7,
                    10, 140, 7, 10, 120, 6, 10, 120, 1, 10,
                ],
            ),
            (
                "TrebleBooster",
                [
                    2, 1, 255, 110, 7, 10, 110, 7, 10, 130, 7, 10, 120, 7, 10, 140, 7, 10, 120, 7,
                    10, 130, 7, 10, 140, 6, 10, 160, 1, 10,
                ],
            ),
            (
                "Balanced",
                [
                    3, 1, 255, 110, 7, 10, 110, 7, 10, 120, 7, 10, 110, 7, 10, 120, 7, 10, 110, 7,
                    10, 120, 7, 10, 130, 6, 10, 110, 1, 10,
                ],
            ),
            (
                "SoundcoreSignature",
                [
                    0, 1, 255, 130, 7, 10, 120, 7, 10, 130, 7, 10, 120, 7, 10, 130, 7, 10, 110, 7,
                    10, 120, 7, 10, 135, 6, 10, 135, 1, 10,
                ],
            ),
        ];
        for (preset, body) in cases {
            device
                .assert_set_settings_response(
                    vec![(
                        SettingId::PresetEqualizerProfile,
                        Value::OptionalString(Some(preset.into())),
                    )],
                    vec![packet::Outbound::new(
                        packet::Command([0x02, 0x88]),
                        body.to_vec(),
                    )],
                )
                .await;
        }
    }

    #[tokio::test(start_paused = true)]
    async fn set_volume_adjustments_sends_custom_profile() {
        let mut device = test_device().await;
        let cases = [
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [
                    254, 1, 255, 120, 7, 10, 120, 7, 10, 120, 7, 10, 120, 7, 10, 120, 7, 10, 120,
                    7, 10, 120, 7, 10, 120, 6, 10, 120, 1, 10,
                ],
            ),
            (
                [60, -60, 10, 0, 0, 0, 0, 0, -10],
                [
                    254, 1, 255, 180, 7, 10, 60, 7, 10, 130, 7, 10, 120, 7, 10, 120, 7, 10, 120, 7,
                    10, 120, 7, 10, 120, 6, 10, 110, 1, 10,
                ],
            ),
        ];
        for (volume_adjustments, body) in cases {
            device
                .assert_set_settings_response(
                    vec![(
                        SettingId::VolumeAdjustments,
                        Value::I16Vec(volume_adjustments.to_vec()),
                    )],
                    vec![packet::Outbound::new(
                        packet::Command([0x02, 0x88]),
                        body.to_vec(),
                    )],
                )
                .await;
        }
    }

    fn set_flame_effect() -> packet::Outbound {
        packet::Outbound::new(
            packet::Command([0x0e, 0x07]),
            vec![2, 4, 255, 30, 0, 255, 122, 0, 255, 168, 0, 255, 226, 0],
        )
    }

    #[tokio::test(start_paused = true)]
    async fn set_lights_enabled_sends_toggle() {
        let mut device = test_device().await;
        device
            .assert_set_settings_response(
                vec![(SettingId::LightsEnabled, true.into())],
                vec![packet::Outbound::new(
                    packet::Command([0x0e, 0x08]),
                    vec![1],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn selecting_effect_turns_lights_on() {
        let mut device = test_device().await;
        device
            .assert_set_settings_response(
                vec![(SettingId::LightsMode, Cow::from("Flame").into())],
                vec![set_flame_effect()],
            )
            .await;
        device.assert_setting_values([(SettingId::LightsEnabled, true.into())]);
    }
}
