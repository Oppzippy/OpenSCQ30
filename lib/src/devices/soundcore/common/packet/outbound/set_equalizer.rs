use crate::devices::soundcore::common::{packet, structures::CommonEqualizerConfiguration};

use super::outbound_packet::ToPacket;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetEqualizer<'a, const C: usize, const B: usize> {
    pub equalizer_configuration: &'a CommonEqualizerConfiguration<C, B>,
}

pub const SET_EQUALIZER_COMMAND: packet::Command = packet::Command([0x02, 0x81]);

impl<const C: usize, const B: usize> ToPacket for SetEqualizer<'_, C, B> {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        SET_EQUALIZER_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.equalizer_configuration.bytes().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::{
        packet::outbound::ToPacket,
        structures::{CommonEqualizerConfiguration, CommonVolumeAdjustments},
    };

    use super::SetEqualizer;

    #[test]
    fn it_matches_an_example_custom_eq_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0xfe, 0xfe, 0x3c, 0xb4, 0x8f,
            0xa0, 0x8e, 0xb4, 0x74, 0x88, 0xe6,
        ];
        let actual = SetEqualizer {
            equalizer_configuration: &CommonEqualizerConfiguration::new(
                0xfefe,
                [CommonVolumeAdjustments::new([
                    -60, 60, 23, 40, 22, 60, -4, 16,
                ])],
            ),
        }
        .to_packet()
        .bytes();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_matches_an_example_soundcore_signature_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x00, 0x00, 0x78, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x4d,
        ];
        let actual = SetEqualizer {
            equalizer_configuration: &CommonEqualizerConfiguration::<1, 8>::new(
                0x0000,
                [CommonVolumeAdjustments::default()],
            ),
        }
        .to_packet()
        .bytes();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_matches_an_example_treble_reducer_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x15, 0x00, 0x78, 0x78, 0x78,
            0x64, 0x5a, 0x50, 0x50, 0x3c, 0xa4,
        ];
        let actual = SetEqualizer {
            equalizer_configuration: &CommonEqualizerConfiguration::<1, 8>::new(
                0x15,
                [CommonVolumeAdjustments::new([
                    0, 0, 0, -20, -30, -40, -40, -60,
                ])],
            ),
        }
        .to_packet()
        .bytes();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_sends_second_channel_if_present() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x1C, 0x00, 0x15, 0x00, 0x78, 0x78, 0x78,
            0x64, 0x5a, 0x50, 0x50, 0x3c, 0x78, 0x78, 0x78, 0x64, 0x5a, 0x50, 0x50, 0x3c, 0xae,
        ];
        let packet = SetEqualizer {
            equalizer_configuration: &CommonEqualizerConfiguration::<2, 8>::new(
                0x15,
                [
                    CommonVolumeAdjustments::new([0, 0, 0, -20, -30, -40, -40, -60]),
                    CommonVolumeAdjustments::new([0, 0, 0, -20, -30, -40, -40, -60]),
                ],
            ),
        };
        assert_eq!(EXPECTED, packet.to_packet().bytes());
    }
}
