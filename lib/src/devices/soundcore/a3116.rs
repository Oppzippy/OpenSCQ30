use std::collections::HashMap;

use crate::devices::soundcore::a3116::packets::inbound::A3116StateUpdatePacket;
use crate::devices::soundcore::a3116::state::A3116State;
use crate::devices::soundcore::common::device::SoundcoreDeviceConfig;
use crate::devices::soundcore::common::packet;
use crate::devices::soundcore::common::packet::outbound::ToPacket;
use crate::devices::soundcore::common::{
    device::fetch_state_from_state_update_packet, macros::soundcore_device,
    packet::outbound::RequestState,
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3116State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3116State, A3116StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3116_equalizer().await;
        builder.a3116_volume(16);
        builder.a3116_auto_power_off();
        builder.single_battery(5);
        builder.serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3116StateUpdatePacket::default().to_packet(),
        )])
    },
    CONFIG,
);

const CONFIG: SoundcoreDeviceConfig = SoundcoreDeviceConfig {
    checksum_kind: packet::ChecksumKind::None,
};

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{
        DeviceModel,
        devices::soundcore::common::device::test_utils::TestSoundcoreDevice,
        settings::{self, SettingId},
    };

    use super::*;

    #[tokio::test(start_paused = true)]
    async fn manually_crafted_packet_matches_soundcore_app() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3116,
            HashMap::from([
                (
                    packet::Command([1, 1]),
                    packet::Inbound::new(
                        packet::Command([1, 1]),
                        vec![
                            0, 5, 7, 0, 2, 57, 57, 46, 57, 57, 65, 66, 67, 68, 69, 70, 65, 66, 67,
                            68, 69, 70, 65, 66, 67, 68, 255, 6, 7, 8, 9, 10, 6, 5, 4, 3,
                        ],
                    ),
                ),
                (
                    packet::Command([1, 16]), // voice prompt (TODO)
                    packet::Inbound::new(packet::Command([1, 16]), vec![1]),
                ),
            ]),
            CONFIG,
        )
        .await;

        device.assert_setting_values([
            (SettingId::IsCharging, Cow::from("No").into()),
            (SettingId::BatteryLevel, Cow::from("5/5").into()),
            (SettingId::Volume, 7.into()),
            (SettingId::FirmwareVersion, Cow::from("99.99").into()),
            (
                SettingId::SerialNumber,
                Cow::from("ABCDEFABCDEFABCD").into(),
            ),
            (
                SettingId::PresetEqualizerProfile,
                settings::Value::OptionalString(None),
            ),
            (
                SettingId::VolumeAdjustments,
                settings::Value::I16Vec(vec![0, 1, 2, 3, 4, 0, -1, -2, -3]),
            ),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn equalizer_export_has_no_fraction_digits() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3116,
            HashMap::from([(
                packet::Command([1, 1]),
                packets::inbound::A3116StateUpdatePacket::default().to_packet(),
            )]),
            CONFIG,
        )
        .await;

        device
            .assert_set_settings_response_unordered(
                vec![
                    (
                        SettingId::VolumeAdjustments,
                        settings::Value::I16Vec(vec![4, 3, 2, 1, 0, -1, -2, -3, -4]),
                    ),
                    (
                        SettingId::CustomEqualizerProfile,
                        settings::Value::ModifiableSelectCommand(
                            settings::ModifiableSelectCommand::Add("Test".into()),
                        ),
                    ),
                    (
                        SettingId::ExportCustomEqualizerProfiles,
                        settings::Value::StringVec(vec!["Test".into()]),
                    ),
                ],
                vec![
                    packet::Outbound::new(packet::Command([0x02, 0x81]), vec![0xF]),
                    packet::Outbound::new(
                        packet::Command([0x02, 0x83]),
                        vec![10, 9, 8, 7, 6, 5, 4, 3, 2],
                    ),
                ],
            )
            .await;

        device.assert_setting_values([(
            SettingId::ExportCustomEqualizerProfilesOutput,
            settings::Value::String(r#"[{"name":"Test","volumeAdjustments":[4.0,3.0,2.0,1.0,0.0,-1.0,-2.0,-3.0,-4.0]}]"#.into()),
        )]);
    }

    #[tokio::test(start_paused = true)]
    async fn equalizer_import_has_no_fraction_digits() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3116,
            HashMap::from([(
                packet::Command([1, 1]),
                packets::inbound::A3116StateUpdatePacket::default().to_packet(),
            )]),
            CONFIG,
        )
        .await;

        device
            .assert_set_settings_response_unordered(
                vec![
                    (
                        SettingId::ImportCustomEqualizerProfiles,
                        settings::Value::String(r#"[{"name":"Test","volumeAdjustments":[4.0,3.0,2.0,1.0,0.0,-1.0,-2.0,-3.0,-4.0]}]"#.into()),
                    ),
                    (
                        SettingId::CustomEqualizerProfile,
                        settings::Value::String("Test".into()),
                    )
                ],
                vec![
                    packet::Outbound::new(packet::Command([0x02, 0x81]), vec![0xF]),
                    packet::Outbound::new(
                        packet::Command([0x02, 0x83]),
                        vec![10, 9, 8, 7, 6, 5, 4, 3, 2],
                    ),
                ],
            )
            .await;

        device.assert_setting_values([(
            SettingId::VolumeAdjustments,
            settings::Value::I16Vec(vec![4, 3, 2, 1, 0, -1, -2, -3, -4]),
        )]);
    }
}
