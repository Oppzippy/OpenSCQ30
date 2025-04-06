use crate::devices::soundcore::standard::structures::{Command, EqualizerConfiguration};

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, PartialEq)]
pub struct SetEqualizerWithDrcPacket<'a> {
    pub equalizer_configuration: &'a EqualizerConfiguration,
}

impl OutboundPacket for SetEqualizerWithDrcPacket<'_> {
    fn command(&self) -> Command {
        Command::new([0x08, 0xEE, 0x00, 0x00, 0x00, 0x02, 0x83])
    }

    fn body(&self) -> Vec<u8> {
        self.equalizer_configuration
            .bytes()
            .chain(
                self.equalizer_configuration
                    .volume_adjustments()
                    .iter()
                    .map(|v| v.apply_drc().into_bytes())
                    .flatten(),
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::standard::{
        packets::outbound::{OutboundPacketBytesExt, SetEqualizerWithDrcPacket},
        structures::{EqualizerConfiguration, VolumeAdjustments},
    };

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x83, 0x1c, 0x00, 0xfe, 0xfe, 0x3c, 0xb4, 0x8f,
            0xf0, 0x8e, 0x00, 0x74, 0x88, 0x6d, 0x86, 0x70, 0x88, 0x7b, 0x66, 0x7e, 0x79, 0x4f,
        ];

        let actual = SetEqualizerWithDrcPacket {
            equalizer_configuration: &EqualizerConfiguration::new_custom_profile(vec![
                VolumeAdjustments::new(vec![-60, 60, 23, 120, 22, -120, -4, 16]).unwrap(),
            ]),
        }
        .bytes();
        assert_eq!(EXPECTED, actual);
    }
}
