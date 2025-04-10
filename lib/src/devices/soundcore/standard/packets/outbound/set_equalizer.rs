use crate::devices::soundcore::standard::structures::{Command, EqualizerConfiguration};

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, PartialEq)]
pub struct SetEqualizerPacket<'a, const C: usize, const B: usize> {
    pub equalizer_configuration: &'a EqualizerConfiguration<C, B>,
}

pub const COMMAND: Command = Command::new([0x08, 0xEE, 0x00, 0x00, 0x00, 0x02, 0x81]);

impl<const C: usize, const B: usize> OutboundPacket for SetEqualizerPacket<'_, C, B> {
    fn command(&self) -> Command {
        COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.equalizer_configuration.bytes().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::standard::{
        packets::outbound::OutboundPacketBytesExt,
        structures::{EqualizerConfiguration, PresetEqualizerProfile, VolumeAdjustments},
    };

    use super::SetEqualizerPacket;

    #[test]
    fn it_matches_an_example_custom_eq_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0xfe, 0xfe, 0x3c, 0xb4, 0x8f,
            0xa0, 0x8e, 0xb4, 0x74, 0x88, 0xe6,
        ];
        let actual = SetEqualizerPacket {
            equalizer_configuration: &EqualizerConfiguration::new_custom_profile([
                VolumeAdjustments::new([-60, 60, 23, 40, 22, 60, -4, 16]),
            ]),
        }
        .bytes();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_matches_an_example_soundcore_signature_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x00, 0x00, 0x78, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x4d,
        ];
        let actual = SetEqualizerPacket {
            equalizer_configuration: &EqualizerConfiguration::<1, 8>::new_from_preset_profile(
                PresetEqualizerProfile::SoundcoreSignature,
                [Vec::new()],
            ),
        }
        .bytes();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_matches_an_example_treble_reducer_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x15, 0x00, 0x78, 0x78, 0x78,
            0x64, 0x5a, 0x50, 0x50, 0x3c, 0xa4,
        ];
        let actual = SetEqualizerPacket {
            equalizer_configuration: &EqualizerConfiguration::<1, 8>::new_from_preset_profile(
                PresetEqualizerProfile::TrebleReducer,
                [Vec::new()],
            ),
        }
        .bytes();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_sends_second_channel_if_present() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x1C, 0x00, 0x15, 0x00, 0x78, 0x78, 0x78,
            0x64, 0x5a, 0x50, 0x50, 0x3c, 0x78, 0x78, 0x78, 0x64, 0x5a, 0x50, 0x50, 0x3c, 0xae,
        ];
        let packet = SetEqualizerPacket {
            equalizer_configuration: &EqualizerConfiguration::<2, 8>::new_from_preset_profile(
                PresetEqualizerProfile::TrebleReducer,
                [Vec::new(), Vec::new()],
            ),
        };
        assert_eq!(EXPECTED, packet.bytes());
    }
}
