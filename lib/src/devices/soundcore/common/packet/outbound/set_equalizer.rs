use crate::devices::soundcore::common::{packet, structures::EqualizerConfiguration};

pub const SET_EQUALIZER_COMMAND: packet::Command = packet::Command([0x02, 0x81]);

pub fn set_equalizer<
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>(
    equalizer_configuration: &EqualizerConfiguration<
        CHANNELS,
        BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >,
) -> packet::Outbound {
    packet::Outbound::new(
        SET_EQUALIZER_COMMAND,
        equalizer_configuration.bytes().collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::structures::{
        CommonEqualizerConfiguration, CommonVolumeAdjustments,
    };

    use super::*;

    #[test]
    fn it_matches_an_example_custom_eq_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0xfe, 0xfe, 0x3c, 0xb4, 0x8f,
            0xa0, 0x8e, 0xb4, 0x74, 0x88, 0xe6,
        ];
        let actual = set_equalizer(&CommonEqualizerConfiguration::new(
            0xfefe,
            [CommonVolumeAdjustments::new([
                -60, 60, 23, 40, 22, 60, -4, 16,
            ])],
        ))
        .bytes_with_checksum();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_matches_an_example_soundcore_signature_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x00, 0x00, 0x78, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x4d,
        ];
        let actual = set_equalizer(&CommonEqualizerConfiguration::<1, 8>::new(
            0x0000,
            [CommonVolumeAdjustments::default()],
        ))
        .bytes_with_checksum();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_matches_an_example_treble_reducer_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x15, 0x00, 0x78, 0x78, 0x78,
            0x64, 0x5a, 0x50, 0x50, 0x3c, 0xa4,
        ];
        let actual = set_equalizer(&CommonEqualizerConfiguration::<1, 8>::new(
            0x15,
            [CommonVolumeAdjustments::new([
                0, 0, 0, -20, -30, -40, -40, -60,
            ])],
        ))
        .bytes_with_checksum();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_sends_second_channel_if_present() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x1C, 0x00, 0x15, 0x00, 0x78, 0x78, 0x78,
            0x64, 0x5a, 0x50, 0x50, 0x3c, 0x78, 0x78, 0x78, 0x64, 0x5a, 0x50, 0x50, 0x3c, 0xae,
        ];
        let packet = set_equalizer(&CommonEqualizerConfiguration::<2, 8>::new(
            0x15,
            [
                CommonVolumeAdjustments::new([0, 0, 0, -20, -30, -40, -40, -60]),
                CommonVolumeAdjustments::new([0, 0, 0, -20, -30, -40, -40, -60]),
            ],
        ));
        assert_eq!(EXPECTED, packet.bytes_with_checksum());
    }
}
