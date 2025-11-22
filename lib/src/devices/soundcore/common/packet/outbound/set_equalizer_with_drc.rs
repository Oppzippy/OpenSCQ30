use crate::devices::soundcore::common::{packet, structures::CommonEqualizerConfiguration};

use super::outbound_packet::ToPacket;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetEqualizerWithDrc<'a, const C: usize, const B: usize> {
    pub equalizer_configuration: &'a CommonEqualizerConfiguration<C, B>,
}

impl<const C: usize, const B: usize> ToPacket for SetEqualizerWithDrc<'_, C, B> {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        packet::Command([0x02, 0x83])
    }

    fn body(&self) -> Vec<u8> {
        self.equalizer_configuration
            .bytes()
            .chain(
                self.equalizer_configuration
                    .volume_adjustments()
                    .iter()
                    .flat_map(|v| v.apply_drc().bytes()),
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::{
        packet::outbound::{SetEqualizerWithDrc, ToPacket},
        structures::{CommonEqualizerConfiguration, CommonVolumeAdjustments},
    };

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x83, 0x1c, 0x00, 0xfe, 0xfe, 0x3c, 0xb4, 0x8f,
            0xf0, 0x8e, 0x00, 0x74, 0x88, 0x6d, 0x86, 0x70, 0x88, 0x7b, 0x66, 0x7e, 0x79, 0x4f,
        ];

        let actual = SetEqualizerWithDrc {
            equalizer_configuration: &CommonEqualizerConfiguration::new(
                0xfefe,
                [CommonVolumeAdjustments::new([
                    -60, 60, 23, 120, 22, -120, -4, 16,
                ])],
            ),
        }
        .to_packet()
        .bytes();
        assert_eq!(EXPECTED, actual);
    }
}
