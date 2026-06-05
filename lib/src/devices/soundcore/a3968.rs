use std::collections::HashMap;

use crate::devices::soundcore::{
    a3968::{packets::inbound::A3968StateUpdatePacket, state::A3968State},
    common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        packet::outbound::{RequestState, ToPacket},
    },
};

mod packets;
mod state;

soundcore_device!(
    A3968State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<A3968State, A3968StateUpdatePacket>(packet_io).await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3959_sound_modes();
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3968StateUpdatePacket::default().to_packet(),
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
            packet,
        },
        settings::SettingId,
    };

    /// State update packet posted by "Hate9" in GitHub issue #170 (a different X20 unit).
    #[tokio::test(start_paused = true)]
    async fn parses_state_update_packet_from_issue_170() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3968,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        0, 1, 5, 5, 0, 0, 48, 49, 46, 54, 53, 48, 49, 46, 54, 53, 51, 57, 54, 56,
                        98, 48, 51, 56, 101, 50, 54, 98, 98, 101, 57, 97, 0, 0, 0, 0, 0, 2, 254,
                        254, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 48, 1, 1, 135, 114, 140, 142, 150, 145, 166, 134,
                        60, 60, 135, 114, 140, 142, 150, 145, 166, 134, 60, 60, 255, 255, 255, 255,
                        1, 151, 151, 140, 143, 151, 145, 165, 134, 60, 0, 151, 151, 140, 143, 151,
                        145, 165, 134, 60, 0, 0, 0, 8, 97, 102, 48, 51, 68, 68, 55, 0, 50, 1, 0, 0,
                        255, 54, 1, 0, 1, 0, 1, 2, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::TwsStatus, "Connected".into()),
            (SettingId::HostDevice, "Left".into()),
            (SettingId::BatteryLevelLeft, "5/5".into()),
            (SettingId::BatteryLevelRight, "5/5".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::FirmwareVersionLeft, "01.65".into()),
            (SettingId::FirmwareVersionRight, "01.65".into()),
            (SettingId::SerialNumber, "3968b038e26bbe9a".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::NoiseCancelingMode, "Manual".into()),
        ]);
    }
}
