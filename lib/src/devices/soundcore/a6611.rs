use std::collections::HashMap;

use crate::devices::soundcore::{
    a6611::{packets::inbound::A6611StateUpdatePacket, state::A6611State},
    common::{
        macros::soundcore_device,
        modules::dual_battery_level::DualBatteryLevelConfiguration,
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
    A6611State,
    async |packet_io| {
        let state_update_packet: A6611StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        Ok(A6611State::new(state_update_packet))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.a6611_sleep_mode();
        builder.a6611_find_device();

        builder.tws_status();
        builder.dual_battery_level_custom(DualBatteryLevelConfiguration {
            max_level: 10,
            level_offset: 1,
        });
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A6611StateUpdatePacket::default().to_packet(),
        )])
    },
);

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc, time::Duration};

    use macaddr::MacAddr6;
    use tokio::sync::mpsc;

    use crate::{
        DeviceModel,
        api::{device::OpenSCQ30DeviceRegistry, settings::Value},
        connection_backend::mock::rfcomm::MockRfcommBackend,
        devices::soundcore::{
            a6611::packets::inbound::BLUETOOTH_MODE_UPDATE_COMMAND,
            common::{
                device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
                packet,
            },
        },
        settings::SettingId,
        storage::OpenSCQ30Database,
    };

    // Captured from a real A6611 on firmware 01.57 in sleep mode
    const REAL_STATE_UPDATE_BODY: &[u8] = &[
        0x01, 0x01, 0x09, 0x09, 0x30, 0x31, 0x2e, 0x35, 0x37, 0x30, 0x31, 0x2e, 0x35, 0x37, 0x36,
        0x36, 0x31, 0x31, 0x46, 0x34, 0x39, 0x44, 0x38, 0x41, 0x31, 0x45, 0x44, 0x42, 0x38, 0x38,
        0x00, 0x00, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x00, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x06, 0xff, 0xff, 0x66, 0x66, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x01, 0x3c, 0x00, 0x00, 0xff, 0xff, 0xff, 0x01,
        0x01, 0x00, 0x31, 0x37, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0x01, 0x00, 0x00, 0x01, 0x07, 0x00, 0x00, 0x00, 0x00,
    ];

    const BLUETOOTH_MODE_INDEX: usize = 65;

    async fn device_with_state(body: Vec<u8>) -> TestSoundcoreDevice {
        TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA6611,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(packet::Command([1, 1]), body),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await
    }

    async fn real_device() -> TestSoundcoreDevice {
        device_with_state(REAL_STATE_UPDATE_BODY.to_vec()).await
    }

    #[tokio::test(start_paused = true)]
    async fn test_parses_real_state_update() {
        let device = real_device().await;
        device.assert_setting_values([
            (SettingId::FirmwareVersionLeft, "01.57".into()),
            (SettingId::FirmwareVersionRight, "01.57".into()),
            (SettingId::SerialNumber, "6611F49D8A1EDB88".into()),
            (SettingId::BatteryLevelLeft, "10/10".into()),
            (SettingId::BatteryLevelRight, "10/10".into()),
            (SettingId::SleepMode, true.into()),
            (SettingId::FindDeviceLeft, false.into()),
            (SettingId::FindDeviceRight, false.into()),
        ]);
    }

    // Captured from a real A6611 on firmware 03.22 in Bluetooth mode
    const REAL_BLUETOOTH_MODE_STATE_UPDATE_BODY: &[u8] = &[
        0x01, 0x01, 0x09, 0x09, 0x30, 0x33, 0x2e, 0x32, 0x32, 0x30, 0x33, 0x2e, 0x32, 0x32, 0x36,
        0x36, 0x31, 0x31, 0x39, 0x30, 0x42, 0x46, 0x44, 0x39, 0x37, 0x30, 0x31, 0x38, 0x35, 0x38,
        0xfe, 0xfe, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x06, 0xff, 0xff, 0xff, 0xdd, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x01, 0x01, 0x3c, 0x00, 0x00, 0xff, 0xff, 0xff, 0x01,
        0x01, 0x00, 0x31, 0x0d, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0x01, 0x00, 0x00, 0x01, 0x05, 0x00, 0x00, 0x00, 0x00,
    ];

    #[tokio::test(start_paused = true)]
    async fn test_parses_bluetooth_mode() {
        assert_eq!(REAL_STATE_UPDATE_BODY[BLUETOOTH_MODE_INDEX], 0);
        assert_eq!(REAL_BLUETOOTH_MODE_STATE_UPDATE_BODY[BLUETOOTH_MODE_INDEX], 1);
        let device = device_with_state(REAL_BLUETOOTH_MODE_STATE_UPDATE_BODY.to_vec()).await;
        device.assert_setting_values([
            (SettingId::SleepMode, false.into()),
            (SettingId::FirmwareVersionLeft, "03.22".into()),
            (SettingId::SerialNumber, "661190BFD9701858".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn test_sleep_mode_follows_device_notifications() {
        let device = real_device().await;
        device
            .receive_packet(packet::Inbound::new(BLUETOOTH_MODE_UPDATE_COMMAND, vec![1]))
            .await;
        device.assert_setting_values([(SettingId::SleepMode, false.into())]);
        device
            .receive_packet(packet::Inbound::new(BLUETOOTH_MODE_UPDATE_COMMAND, vec![0]))
            .await;
        device.assert_setting_values([(SettingId::SleepMode, true.into())]);
    }

    // Expected packets are the official app's, captured from real earbuds. Every toggle must update the setting and
    // notify the UI, which never happened on the A20 because its change notifications closed right after connecting.
    #[tokio::test(start_paused = true)]
    async fn test_toggles_match_official_app_and_notify() {
        let (inbound_sender, inbound_receiver) = mpsc::channel(100);
        let (outbound_sender, mut outbound_receiver) = mpsc::channel(100);
        let registry = super::device_registry(
            Arc::new(MockRfcommBackend::new(inbound_receiver, outbound_sender)),
            Arc::new(OpenSCQ30Database::new_in_memory().await.unwrap()),
            DeviceModel::SoundcoreA6611,
        );
        let connect = tokio::spawn(async move { registry.connect(MacAddr6::nil()).await.unwrap() });
        outbound_receiver.recv().await.expect("state request");
        inbound_sender
            .send(
                packet::Inbound::new(packet::Command([1, 1]), REAL_STATE_UPDATE_BODY.to_vec())
                    .bytes_with_checksum(),
            )
            .await
            .unwrap();
        let device = connect.await.unwrap();
        let mut changes = device.watch_for_changes();
        changes.borrow_and_update();

        for (setting_id, value, command, body) in [
            (SettingId::FindDeviceLeft, true, [0x10, 0x89], vec![1, 0, 0]),
            (SettingId::FindDeviceLeft, false, [0x10, 0x89], vec![0, 0, 0]),
            (SettingId::FindDeviceRight, true, [0x10, 0x89], vec![0, 1, 0]),
            (SettingId::FindDeviceRight, false, [0x10, 0x89], vec![0, 0, 0]),
            (SettingId::SleepMode, false, [0x01, 0xa9], vec![1]),
            (SettingId::SleepMode, true, [0x01, 0xa9], vec![0]),
            (SettingId::SleepMode, false, [0x01, 0xa9], vec![1]),
        ] {
            let set = tokio::spawn({
                let device = device.clone();
                async move { device.set_setting_values(vec![(setting_id, value.into())]).await }
            });
            assert_eq!(
                outbound_receiver.recv().await.unwrap(),
                packet::Outbound::new(packet::Command(command), body).bytes_with_checksum(),
            );
            // Acked like the real earbuds do, with an empty body
            inbound_sender
                .send(packet::Inbound::new(packet::Command(command), Vec::new()).bytes_with_checksum())
                .await
                .unwrap();
            set.await.unwrap().expect("acked write should succeed");
            assert_eq!(
                Value::from(device.setting(&setting_id).unwrap()),
                value.into(),
                "{setting_id} should reflect the toggle",
            );
            tokio::time::timeout(Duration::from_secs(1), changes.changed())
                .await
                .expect("the UI must be notified of the change")
                .expect("change notifications must stay open");

        }
    }

    #[test]
    fn test_find_device_requires_confirmation() {
        assert!(SettingId::FindDeviceLeft.requires_confirmation());
        assert!(SettingId::FindDeviceRight.requires_confirmation());
        assert!(!SettingId::SleepMode.requires_confirmation());
    }

}
