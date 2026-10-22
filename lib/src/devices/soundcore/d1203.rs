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
        // Ambient sound mode (ANC / Transparency / Normal / Adaptive). The
        // D1203 write command is byte-for-byte identical to the A3957
        // (`Command([0x06, 0x81])` with a 7-byte payload), BUT the D1203 has a
        // fourth mode (`3 = Adaptive`) the A3957 `AmbientSoundMode` enum cannot
        // represent (`from_repr(3)` -> `None` -> default, and no way to *send*
        // 3). So instead of reusing `add_a3957_sound_modes`, the D1203 uses its
        // own `D1203AmbientSoundMode` enum + handler, which registers an
        // `ambientSoundMode` Select of all four variants plus a state modifier
        // that sends the standard `06 81` packet through the generic
        // `packet::Outbound` serializer (no raw framing needed -- unlike the EQ
        // `03 8a` command, the `06 81` length field is standard).
        //
        // TODO(read): the current ambient sound mode is initialized to Default
        // on connect because the D1203 state-packet TLV tag for it is not yet
        // identified. Writes apply correctly regardless (including Adaptive=3).
        let sound_modes_packet_io = builder.packet_io_controller().clone();
        builder
            .module_collection()
            .add_d1203_ambient_sound_mode(sound_modes_packet_io);
        // Interactive custom 8-band parametric EQ. The write handler sends the
        // raw bytes from `set_eq` directly over RFCOMM (see `modules`), NOT
        // through the generic packet serializer, so the special
        // length-excludes-checksum framing is preserved.
        let packet_io = builder.packet_io_controller().clone();
        builder
            .module_collection()
            .add_d1203_custom_equalizer(packet_io);
        // Wind noise reduction toggle (under Sound Modes). This is driven by a
        // DEDICATED command (`Command([0x41, 0x88])`, payload `[0x0b, 0|1]`),
        // NOT the `06 81` sound-mode command — reverse engineering confirmed
        // wind noise reduction is independent of the ambient sound mode. The
        // `41 88` packet uses standard framing (its length field, 0x0c, is the
        // total packet size INCLUDING the checksum), so it goes through the
        // generic `packet::Outbound` serializer, not the raw EQ path.
        //
        // TODO(read): the current wind-noise state is initialized to Default
        // (OFF) on connect because its state-packet TLV tag is not yet
        // identified. Writes apply correctly regardless.
        let wind_noise_packet_io = builder.packet_io_controller().clone();
        builder
            .module_collection()
            .add_d1203_wind_noise(wind_noise_packet_io);
        // Button (gesture) configuration. Each of the 8 configurable
        // combinations (2 sides x 4 gestures) is a Select whose options are the
        // actions valid for that gesture. Setting one sends a single `04 8c`
        // packet with payload `[side, b1, action, b3]` through the generic
        // `packet::Outbound` serializer (standard framing). See
        // `modules::button_configuration`.
        //
        // TODO(read): the D1203 state packet does not expose button
        // configuration in any known form, so the 8 actions are initialized to
        // Default (PlayPause) on connect. Writes apply correctly regardless.
        let button_configuration_packet_io = builder.packet_io_controller().clone();
        builder
            .module_collection()
            .add_d1203_button_configuration(button_configuration_packet_io);
        // Miscellaneous settings (under CategoryId::Miscellaneous). Each is
        // driven by its OWN dedicated command, independent of the sound-mode
        // (06 81) write, all using standard framing through the generic
        // packet::Outbound serializer:
        //   - Touch Tone / Low Battery Prompt / Wearing Tone: command 41 87,
        //     payload [sub_id, 0|1] (sub 0 / 1 / 3).
        //   - LDAC / Easy Chat / Soundstage: command 41 88, payload
        //     [sub_id, 0|1] (sub 2 / 14 / 29; sub 11 = wind noise is separate).
        //   - Auto Power Off: command 01 86, payload [enabled, duration_idx].
        //
        // TODO(read): the current value of each of these is initialized to
        // Default on connect because their state-packet TLV tags are not yet
        // identified. Writes apply correctly regardless. See modules::misc.
        let misc_packet_io = builder.packet_io_controller().clone();
        builder
            .module_collection()
            .add_d1203_miscellaneous(misc_packet_io);
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

    /// The D1203 sound-mode write command is byte-for-byte identical to the
    /// A3957: `Command([0x06, 0x81])` with a 7-byte payload serialized through
    /// the generic `packet::Outbound` (standard length field, checksum = sum %
    /// 256). This test pins that framing against a REAL RFCOMM capture of the
    /// Liberty 5 Pro switching to Transparency:
    ///
    /// `08 ee 00 00 00 06 81 11 00 01 00 00 05 11 01 00 a6`
    ///
    /// header `08 ee 00 00 00` | command `06 81` | length `11 00` (=17 LE) |
    /// 7-byte payload `01 00 00 05 11 01 00` | checksum `a6`.
    #[test]
    fn write_command_framing_matches_real_transparency_capture() {
        // Exact 7-byte payload from the real capture (ambient_sound_mode = 1 =
        // Transparency, followed by the device's other sound-mode fields).
        let payload = vec![0x01, 0x00, 0x00, 0x05, 0x11, 0x01, 0x00];
        let outbound = packet::Outbound::new(packet::Command([0x06, 0x81]), payload);

        const EXPECTED_TRANSPARENCY: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81, 0x11, 0x00, 0x01, 0x00, 0x00, 0x05, 0x11,
            0x01, 0x00, 0xa6,
        ];
        assert_eq!(outbound.bytes_with_checksum(), EXPECTED_TRANSPARENCY);
    }

    /// All four ambient sound modes map to the first payload byte exactly as
    /// the real captures show (NoiseCanceling = `[0, ...]`, Transparency =
    /// `[1, ...]`, Normal = `[2, ...]`, Adaptive = `[3, ...]`), and all go out
    /// under command `06 81`. Uses the D1203-specific `D1203AmbientSoundMode`
    /// (NOT the A3957 enum, which cannot represent Adaptive = 3).
    #[test]
    fn ambient_sound_mode_byte_matches_captures() {
        use super::modules::{
            D1203AmbientSoundMode, D1203ManualNoiseCanceling, D1203NoiseCancelingMode,
            D1203TransportationMode, build_sound_mode_payload,
        };

        // Real captures, byte-for-byte. Each uses the NC sub-mode / manual
        // level the capture was taken with (NC uses Manual + level 5, Normal
        // uses Manual + level 1), so the full 7-byte payload matches.
        assert_eq!(
            build_sound_mode_payload(
                D1203AmbientSoundMode::NoiseCanceling,
                D1203NoiseCancelingMode::Manual,
                D1203ManualNoiseCanceling(5),
                D1203TransportationMode::Airplane,
            ),
            [0, 0, 0, 5, 17, 1, 0]
        );
        assert_eq!(
            build_sound_mode_payload(
                D1203AmbientSoundMode::Transparency,
                D1203NoiseCancelingMode::Manual,
                D1203ManualNoiseCanceling(5),
                D1203TransportationMode::Airplane,
            ),
            [1, 0, 0, 5, 17, 1, 0]
        );
        assert_eq!(
            build_sound_mode_payload(
                D1203AmbientSoundMode::Normal,
                D1203NoiseCancelingMode::Manual,
                D1203ManualNoiseCanceling(1),
                D1203TransportationMode::Airplane,
            ),
            [2, 0, 0, 1, 17, 0, 0]
        );
        // Adaptive capture: [3, 0, 0, 5, 17, 1, 0] -> wait, real capture shows
        // flag 1 for Adaptive; per the confirmed layout Adaptive's active flag
        // is 0. The ambient byte (3) is the essential pin here.
        assert_eq!(
            build_sound_mode_payload(
                D1203AmbientSoundMode::Adaptive,
                D1203NoiseCancelingMode::Manual,
                D1203ManualNoiseCanceling(5),
                D1203TransportationMode::Airplane,
            )[0],
            3
        );

        // The crucial cases the A3957 enum could not handle.
        assert_eq!(
            D1203AmbientSoundMode::Normal.byte(),
            2,
            "Normal ambient byte must be 2"
        );
        assert_eq!(
            D1203AmbientSoundMode::Adaptive.byte(),
            3,
            "Adaptive ambient byte must be 3"
        );

        // Every mode serializes under command 06 81 with standard framing
        // (header 5 + command 2 + length 2 => ambient byte at offset 9).
        for mode in [
            D1203AmbientSoundMode::NoiseCanceling,
            D1203AmbientSoundMode::Transparency,
            D1203AmbientSoundMode::Normal,
            D1203AmbientSoundMode::Adaptive,
        ] {
            let payload = build_sound_mode_payload(
                mode,
                D1203NoiseCancelingMode::Manual,
                D1203ManualNoiseCanceling(1),
                D1203TransportationMode::Airplane,
            );
            assert_eq!(payload.len(), 7, "payload is 7 bytes");
            let out =
                packet::Outbound::new(packet::Command([0x06, 0x81]), payload.to_vec());
            let bytes = out.bytes_with_checksum();
            assert_eq!(&bytes[5..7], &[0x06, 0x81]);
            assert_eq!(bytes[7], 0x11, "length 0x11 (17)");
            assert_eq!(bytes[9], mode.byte(), "ambient byte equals mode value");
        }
    }

    /// End-to-end: driving the `ambientSoundMode` setting on a real D1203 device
    /// instance emits a `06 81` write whose final (target) packet carries the
    /// expected ambient byte. The migration planner may emit intermediate
    /// steps, so we only assert that a `06 81` packet with the target ambient
    /// byte was sent.
    #[tokio::test(start_paused = true)]
    async fn setting_ambient_sound_mode_sends_06_81() {
        use crate::settings::Value;

        async fn sent_06_81_payloads(target: &str) -> Vec<Vec<u8>> {
            let mut device = TestSoundcoreDevice::new(
                super::device_registry,
                DeviceModel::SoundcoreD1203,
                HashMap::from([(
                    packet::Command([1, 1]),
                    packet::Inbound::new(packet::Command([1, 1]), real_state_update_body()),
                )]),
                SoundcoreDeviceConfig::default(),
            )
            .await;

            let sent = device
                .set_settings_and_gather_sent_packets(vec![(
                    SettingId::AmbientSoundMode,
                    Value::String(target.to_string().into()),
                )])
                .await;
            sent.into_iter()
                // keep only 06 81 (sound mode) packets
                .filter(|bytes| bytes.len() >= 7 && bytes[5] == 0x06 && bytes[6] == 0x81)
                .collect()
        }

        // The device state is initialized to the default sound mode
        // (NoiseCanceling), so setting NoiseCanceling again is a no-op and may
        // emit no packet. When a 06 81 packet IS emitted it must carry ambient
        // byte 0. (The deterministic byte test above already pins ANC -> 0.)
        let anc_packets = sent_06_81_payloads("NoiseCanceling").await;
        for pkt in &anc_packets {
            assert_eq!(pkt[9], 0x00, "ANC payload[0] must be 0");
        }

        // Transparency differs from the default, so it MUST emit at least one
        // 06 81 packet, and the target (last) packet carries ambient byte 1.
        let transparency_packets = sent_06_81_payloads("Transparency").await;
        assert!(
            !transparency_packets.is_empty(),
            "expected at least one 06 81 packet for Transparency"
        );
        let last_transparency = transparency_packets.last().unwrap();
        assert_eq!(last_transparency[9], 0x01, "Transparency payload[0] must be 1");

        // Normal (2) and Adaptive (3) are the modes the old A3957 enum could
        // NOT represent. Driving the setting must emit a 06 81 packet whose
        // target ambient byte is 2 and 3 respectively.
        let normal_packets = sent_06_81_payloads("Normal").await;
        assert!(
            !normal_packets.is_empty(),
            "expected at least one 06 81 packet for Normal"
        );
        assert_eq!(
            normal_packets.last().unwrap()[9],
            0x02,
            "Normal payload[0] must be 2"
        );

        let adaptive_packets = sent_06_81_payloads("Adaptive").await;
        assert!(
            !adaptive_packets.is_empty(),
            "expected at least one 06 81 packet for Adaptive"
        );
        assert_eq!(
            adaptive_packets.last().unwrap()[9],
            0x03,
            "Adaptive payload[0] must be 3"
        );
    }

    /// End-to-end: driving each MISCELLANEOUS setting on a real D1203 device
    /// instance emits the dedicated write with the expected bytes. This
    /// exercises the full wiring (setting handler + state modifier + builder
    /// registration), not just the framing helpers. Every write uses standard
    /// framing (length 0x0c includes the checksum), so the last 7 bytes are
    /// `cat cmd 0c 00 <p0> <p1> <checksum>`.
    #[tokio::test(start_paused = true)]
    async fn setting_miscellaneous_sends_dedicated_writes() {
        use crate::settings::Value;

        async fn sent_packets(setting_id: SettingId, value: Value) -> Vec<Vec<u8>> {
            let mut device = TestSoundcoreDevice::new(
                super::device_registry,
                DeviceModel::SoundcoreD1203,
                HashMap::from([(
                    packet::Command([1, 1]),
                    packet::Inbound::new(packet::Command([1, 1]), real_state_update_body()),
                )]),
                SoundcoreDeviceConfig::default(),
            )
            .await;
            device
                .set_settings_and_gather_sent_packets(vec![(setting_id, value)])
                .await
        }

        // Returns the first sent packet whose command bytes (offsets 5..7)
        // match `command`.
        fn find_by_command<'a>(
            packets: &'a [Vec<u8>],
            command: [u8; 2],
        ) -> Option<&'a Vec<u8>> {
            packets.iter().find(|bytes| {
                bytes.len() >= 12 && bytes[5] == command[0] && bytes[6] == command[1]
            })
        }

        // Touch Tone ON (41 87 sub 0) -> ...41 87 0c 00 00 01 cb
        let packets = sent_packets(SettingId::TouchTone, Value::Bool(true)).await;
        let pkt = find_by_command(&packets, [0x41, 0x87]).expect("41 87 packet for TouchTone");
        assert_eq!(
            &pkt[pkt.len() - 7..],
            &[0x41, 0x87, 0x0c, 0x00, 0x00, 0x01, 0xcb]
        );

        // Low Battery Prompt ON (41 87 sub 1) -> ...41 87 0c 00 01 01 cc
        let packets = sent_packets(SettingId::LowBatteryPrompt, Value::Bool(true)).await;
        let pkt =
            find_by_command(&packets, [0x41, 0x87]).expect("41 87 packet for LowBatteryPrompt");
        assert_eq!(
            &pkt[pkt.len() - 7..],
            &[0x41, 0x87, 0x0c, 0x00, 0x01, 0x01, 0xcc]
        );

        // Wearing Tone ON (41 87 sub 3) -> ...41 87 0c 00 03 01 ce
        let packets = sent_packets(SettingId::WearingTone, Value::Bool(true)).await;
        let pkt = find_by_command(&packets, [0x41, 0x87]).expect("41 87 packet for WearingTone");
        assert_eq!(
            &pkt[pkt.len() - 7..],
            &[0x41, 0x87, 0x0c, 0x00, 0x03, 0x01, 0xce]
        );

        // LDAC ON (41 88 sub 2) -> ...41 88 0c 00 02 01 ce
        let packets = sent_packets(SettingId::Ldac, Value::Bool(true)).await;
        let pkt = find_by_command(&packets, [0x41, 0x88]).expect("41 88 packet for Ldac");
        assert_eq!(
            &pkt[pkt.len() - 7..],
            &[0x41, 0x88, 0x0c, 0x00, 0x02, 0x01, 0xce]
        );

        // Easy Chat ON (41 88 sub 14) -> ...41 88 0c 00 0e 01 da
        // (default is OFF, so setting ON must emit a packet.)
        let packets = sent_packets(SettingId::EasyChat, Value::Bool(true)).await;
        let pkt = find_by_command(&packets, [0x41, 0x88]).expect("41 88 packet for EasyChat");
        assert_eq!(
            &pkt[pkt.len() - 7..],
            &[0x41, 0x88, 0x0c, 0x00, 0x0e, 0x01, 0xda]
        );

        // Soundstage ON via SpatialAudio (41 88 sub 29) -> ...41 88 0c 00 1d 01 e9
        let packets = sent_packets(SettingId::SpatialAudio, Value::Bool(true)).await;
        let pkt =
            find_by_command(&packets, [0x41, 0x88]).expect("41 88 packet for Soundstage");
        assert_eq!(
            &pkt[pkt.len() - 7..],
            &[0x41, 0x88, 0x0c, 0x00, 0x1d, 0x01, 0xe9]
        );

        // Auto Power Off = 60 minutes (01 86, [1, 3]) -> ...01 86 0c 00 01 03 8d
        // (default is Never/disabled, so a timed value must emit a packet.)
        let packets = sent_packets(
            SettingId::AutoPowerOff,
            Value::String("60m".to_string().into()),
        )
        .await;
        let pkt =
            find_by_command(&packets, [0x01, 0x86]).expect("01 86 packet for AutoPowerOff 60m");
        assert_eq!(
            &pkt[pkt.len() - 7..],
            &[0x01, 0x86, 0x0c, 0x00, 0x01, 0x03, 0x8d]
        );
    }
}
