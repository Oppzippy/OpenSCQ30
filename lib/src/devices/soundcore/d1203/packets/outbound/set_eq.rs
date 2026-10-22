//! Custom equalizer write for the Soundcore Liberty 5 Pro (D1203).
//!
//! The D1203 uses a *parametric* 8-band EQ that is encoded completely
//! differently from the project's common `set_equalizer` (which uses command
//! `02 81` with integer-byte gains). This EQ was decoded by reverse
//! engineering (see `D1203_REVERSE_ENGINEERING.md`): category `0x03`, command
//! `0x8a`, with each band carrying a float gain (dB), float frequency (Hz),
//! float Q and an `enabled` byte.
//!
//! Because the on-wire length field and checksum semantics differ slightly
//! from the generic `packet::Outbound::bytes()` (the device's length field
//! EXCLUDES the trailing checksum byte, whereas the generic serializer
//! includes it), this module builds the full packet bytes itself rather than
//! reusing the common packet serializer. The checksum rule is identical to the
//! project's (`sum(all preceding bytes) mod 256`).
//!
//! The public API here is not yet wired into an interactive setting, so it is
//! allowed to be unused for now.
#![allow(dead_code)]

/// Number of EQ bands on the D1203.
pub const BAND_COUNT: usize = 8;

/// 5 byte outbound direction header: `08 ee 00 00 00`.
const DIRECTION_HEADER: [u8; 5] = [0x08, 0xee, 0x00, 0x00, 0x00];

/// Category `0x03`, command `0x8a` = SET EQ (custom).
const CATEGORY: u8 = 0x03;
const COMMAND: u8 = 0x8a;

/// `fe fe` marks the "Custom" preset, matching tag 11 in the state packet.
const CUSTOM_PRESET: [u8; 2] = [0xfe, 0xfe];

/// 8 byte meta block that follows the `fe fe` preset marker. Captured verbatim
/// from the real device; its exact meaning is not needed to drive the EQ.
const META: [u8; 8] = [0x00, 0x01, 0xb5, 0xa6, 0xd0, 0x6a, 0x08, 0x01];

/// Fixed center frequencies in Hz for the 8 bands (octave progression).
pub const BAND_FREQUENCIES: [f32; BAND_COUNT] =
    [100.0, 200.0, 400.0, 800.0, 1600.0, 3200.0, 6400.0, 12800.0];

/// Fixed Q factor used for every band.
pub const BAND_Q: f32 = 0.70;

/// Each band is 13 bytes: `[gain f32 LE][freq f32 LE][Q f32 LE][enabled u8]`.
const BAND_LEN: usize = 13;

/// Build the raw bytes of the D1203 "set custom EQ" packet from 8 gains (dB).
///
/// The frequencies, Q and enabled flag are fixed (100..12800 Hz, Q=0.70,
/// enabled=1). The returned `Vec<u8>` is the complete soundcore packet from the
/// `08` header through the trailing checksum byte, ready to send.
pub fn set_eq(gains_db: [f32; BAND_COUNT]) -> Vec<u8> {
    // Body that follows the length field: preset marker + meta + bands.
    let mut body: Vec<u8> = Vec::with_capacity(CUSTOM_PRESET.len() + META.len() + BAND_COUNT * BAND_LEN);
    body.extend_from_slice(&CUSTOM_PRESET);
    body.extend_from_slice(&META);
    for (gain, freq) in gains_db.iter().zip(BAND_FREQUENCIES.iter()) {
        body.extend_from_slice(&gain.to_le_bytes());
        body.extend_from_slice(&freq.to_le_bytes());
        body.extend_from_slice(&BAND_Q.to_le_bytes());
        body.push(0x01); // enabled
    }

    // length field = total packet size INCLUDING the checksum byte... except
    // the D1203 reports the size of everything BEFORE the checksum. Assemble
    // the pre-checksum bytes first so we can use their length directly.
    let mut packet: Vec<u8> =
        Vec::with_capacity(DIRECTION_HEADER.len() + 2 + 2 + body.len() + 1);
    packet.extend_from_slice(&DIRECTION_HEADER);
    packet.push(CATEGORY);
    packet.push(COMMAND);
    // Placeholder for the 2 byte LE length; filled in once we know the size.
    packet.push(0x00);
    packet.push(0x00);
    packet.extend_from_slice(&body);

    // Length field (LE) = number of bytes from the `08` header up to and
    // including the last band byte, i.e. everything except the checksum.
    let length = packet.len() as u16;
    let length_bytes = length.to_le_bytes();
    packet[7] = length_bytes[0];
    packet[8] = length_bytes[1];

    // Checksum = sum of every preceding byte, mod 256.
    let checksum = packet
        .iter()
        .fold(0_u8, |acc, &byte| acc.wrapping_add(byte));
    packet.push(checksum);

    packet
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The real captured packet (bytes `08` .. last band byte) for 8 bands all
    /// at -6.0 dB. The checksum is computed in the test and appended.
    const EXPECTED_WITHOUT_CHECKSUM: &[u8] = &[
        0x08, 0xee, 0x00, 0x00, 0x00, // direction header
        0x03, 0x8a, // category + command
        0x7b, 0x00, // length = 123 (LE), excludes checksum
        0xfe, 0xfe, // custom preset marker
        0x00, 0x01, 0xb5, 0xa6, 0xd0, 0x6a, 0x08, 0x01, // meta
        // band 1: gain -6.0, freq 100.0, Q 0.70, enabled
        0x00, 0x00, 0xc0, 0xc0, 0x00, 0x00, 0xc8, 0x42, 0x33, 0x33, 0x33, 0x3f, 0x01,
        // band 2: freq 200.0
        0x00, 0x00, 0xc0, 0xc0, 0x00, 0x00, 0x48, 0x43, 0x33, 0x33, 0x33, 0x3f, 0x01,
        // band 3: freq 400.0
        0x00, 0x00, 0xc0, 0xc0, 0x00, 0x00, 0xc8, 0x43, 0x33, 0x33, 0x33, 0x3f, 0x01,
        // band 4: freq 800.0
        0x00, 0x00, 0xc0, 0xc0, 0x00, 0x00, 0x48, 0x44, 0x33, 0x33, 0x33, 0x3f, 0x01,
        // band 5: freq 1600.0
        0x00, 0x00, 0xc0, 0xc0, 0x00, 0x00, 0xc8, 0x44, 0x33, 0x33, 0x33, 0x3f, 0x01,
        // band 6: freq 3200.0
        0x00, 0x00, 0xc0, 0xc0, 0x00, 0x00, 0x48, 0x45, 0x33, 0x33, 0x33, 0x3f, 0x01,
        // band 7: freq 6400.0
        0x00, 0x00, 0xc0, 0xc0, 0x00, 0x00, 0xc8, 0x45, 0x33, 0x33, 0x33, 0x3f, 0x01,
        // band 8: freq 12800.0
        0x00, 0x00, 0xc0, 0xc0, 0x00, 0x00, 0x48, 0x46, 0x33, 0x33, 0x33, 0x3f, 0x01,
    ];

    #[test]
    fn it_matches_the_real_minus_six_db_packet() {
        // Checksum of the real packet = sum of all 123 preceding bytes mod 256.
        let expected_checksum = EXPECTED_WITHOUT_CHECKSUM
            .iter()
            .fold(0_u8, |acc, &byte| acc.wrapping_add(byte));
        // Computed independently: 0xc1 (193).
        assert_eq!(expected_checksum, 0xc1);

        let mut expected: Vec<u8> = EXPECTED_WITHOUT_CHECKSUM.to_vec();
        expected.push(expected_checksum);

        let actual = set_eq([-6.0; BAND_COUNT]);
        assert_eq!(expected, actual);

        // Sanity: length field is 123 (0x7b), total packet is 124 bytes.
        assert_eq!(actual.len(), 124);
        assert_eq!(u16::from_le_bytes([actual[7], actual[8]]), 123);
    }

    #[test]
    fn length_field_excludes_checksum() {
        let packet = set_eq([0.0; BAND_COUNT]);
        let length = u16::from_le_bytes([packet[7], packet[8]]) as usize;
        assert_eq!(length, packet.len() - 1);
    }

    #[test]
    fn gain_encoding_matches_known_values() {
        // +4.1 dB = 33 33 83 40, +6.0 = 00 00 c0 40 per the reverse-engineering doc.
        assert_eq!(4.1_f32.to_le_bytes(), [0x33, 0x33, 0x83, 0x40]);
        assert_eq!(6.0_f32.to_le_bytes(), [0x00, 0x00, 0xc0, 0x40]);
        assert_eq!((-6.0_f32).to_le_bytes(), [0x00, 0x00, 0xc0, 0xc0]);
    }
}
