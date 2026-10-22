use std::collections::HashMap;

use crate::devices::soundcore::{
    d1203::{packets::inbound::D1203StateUpdatePacket, state::D1203State},
    common::{
        self,
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        packet::outbound::{RequestState, ToPacket},
    },
};

mod modules;
mod packets;
mod state;

// Soundcore Liberty 5 Pro (D1203).
//
// Unlike most Soundcore devices, the D1203 state packet is not a fixed-layout
// struct. Its body is a 6 byte prefix followed by a sequence of TLV
// (tag-length-value) fields. Phase 1 only decodes the fields we are certain
// about -- firmware versions (tags 5 and 6) and the serial number (tag 7) --
// and tolerantly skips every other tag. See
// `packets/inbound/state_update.rs` for the parser.
soundcore_device!(
    D1203State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<D1203State, D1203StateUpdatePacket>(packet_io).await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        // Earbud battery uses a 0-100 percentage scale (NOT the 0-10 scale
        // the A3957 uses), so max_level is 100 and there is no offset.
        builder.dual_battery_custom(common::modules::dual_battery::DualBatteryConfiguration {
            max_level: 100,
            level_offset: 0,
        });
        builder.serial_number_and_dual_firmware_version();
        // Interactive custom 8-band parametric EQ. The write handler sends the
        // raw bytes from `set_eq` directly over RFCOMM (see `modules`), NOT
        // through the generic packet serializer, so the special
        // length-excludes-checksum framing is preserved.
        let packet_io = builder.packet_io_controller().clone();
        builder
            .module_collection()
            .add_d1203_custom_equalizer(packet_io);
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            D1203StateUpdatePacket::default().to_packet(),
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

    /// Body captured from a real Liberty 5 Pro (D1203), after the command [1, 1].
    /// 6 byte prefix [1,1,1,2,1,1] followed by TLV fields. This capture was
    /// cross-referenced against the hardware: the app showed L=55% / R=45%,
    /// matching tag 3 value 55 and tag 4 value 45.
    fn real_state_update_body() -> Vec<u8> {
        let mut body: Vec<u8> = vec![
            1, 1, 1, 2, 1, 1, // prefix
            3, 2, 0, 55, // tag 3: battery left, [not charging, 55%]
            4, 2, 0, 45, // tag 4: battery right, [not charging, 45%]
            5, 5, 48, 52, 46, 53, 50, // tag 5: firmware left "04.52"
            6, 5, 48, 52, 46, 53, 50, // tag 6: firmware right "04.52"
            7, 17, 49, 50, 48, 51, 55, 67, 69, 57, 49, 51, 55, 65, 65, 57, 66, 54,
            0, // tag 7: serial "12037CE9137AA9B6" + null
            8, 2, 0, 1, // tag 8 (case battery - ambiguous, not decoded)
            9, 5, 48, 49, 46, 53, 52, // tag 9
            10, 1, 50, // tag 10
            11, 2, 1, 0, // tag 11
            12, 32, // tag 12 header (value = 32 zeros)
        ];
        body.extend(std::iter::repeat(0).take(32)); // tag 12 value
        body.extend_from_slice(&[
            13, 2, 15, 15, 15, 2, 3, 6, 17, 2, 15, 15, 19, 2, 4, 4, 21, 2, 0, 0, 23, 2, 1, 1, 14,
            2, 15, 15, 16, 2, 6, 6, 18, 2, 15, 15, 20, 2, 4, 4, 22, 2, 0, 0, 24, 2, 1, 1, 25, 2, 1,
            2, 42, 1, 1, 26, 1, 255, 27, 4, 8, 73, 16, 32, 28, 1, 1, 35, 92, // tag 35 header
        ]);
        body.extend(std::iter::repeat(0).take(92)); // tag 35 value
        body.extend_from_slice(&[
            36, 3, 2, 0, 0, 37, 2, 5, 1, 38, 2, 1, 0, 39, 3, 0, 90, 0, 41, 6, 124, 233, 19, 118,
            207, 174, 44, 3, 0, 0, 1, 46, 1, 99, 48, 1, 0, 49, 1, 0, 50, 2, 0, 1, 51, 6, 0, 1, 181,
            166, 208, 106, 52, 40, 3, 1, 82, 184, 254, 63, 51, 43, 136, 69, 170, 241, 18, 63, 1, 0,
            0, 64, 191, 102, 94, 23, 70, 115, 104, 65, 64, 1, 0, 0, 128, 64, 0, 192, 106, 70, 242,
            210, 77, 62, 53, 1, 255, 54, 2, 1, 1, 68, 1, 0, 58, 1, 2, 92, 2, 0, 0,
        ]);
        body
    }

    #[tokio::test(start_paused = true)]
    async fn test_parses_firmware_and_serial_from_real_packet() {
        let body = real_state_update_body();

        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1203,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(packet::Command([1, 1]), body),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "55/100".into()),
            (SettingId::BatteryLevelRight, "45/100".into()),
            (SettingId::FirmwareVersionLeft, "04.52".into()),
            (SettingId::FirmwareVersionRight, "04.52".into()),
            (SettingId::SerialNumber, "12037CE9137AA9B6".into()),
        ]);
    }
}
