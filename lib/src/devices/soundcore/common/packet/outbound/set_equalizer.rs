use crate::devices::soundcore::common::{packet, structures::EqualizerConfiguration};

use super::outbound_packet::IntoPacket;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetEqualizer<'a, const C: usize, const B: usize> {
    pub equalizer_configuration: &'a EqualizerConfiguration<C, B>,
}

pub const SET_EQUALIZER_COMMAND: packet::Command = packet::Command([0x02, 0x81]);

impl<const C: usize, const B: usize> IntoPacket for SetEqualizer<'_, C, B> {
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
        packet::outbound::IntoPacket,
        structures::{EqualizerConfiguration, PresetEqualizerProfile, VolumeAdjustments},
    };

    use super::SetEqualizer;

    #[test]
    fn it_matches_an_example_custom_eq_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0xfe, 0xfe, 0x3c, 0xb4, 0x8f,
            0xa0, 0x8e, 0xb4, 0x74, 0x88, 0xe6,
        ];
        let actual = SetEqualizer {
            equalizer_configuration: &EqualizerConfiguration::new_custom_profile([
                VolumeAdjustments::new([-60, 60, 23, 40, 22, 60, -4, 16]),
            ]),
        }
        .into_packet()
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
            equalizer_configuration: &EqualizerConfiguration::<1, 8>::new_from_preset_profile(
                PresetEqualizerProfile::SoundcoreSignature,
                [Vec::new()],
            ),
        }
        .into_packet()
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
            equalizer_configuration: &EqualizerConfiguration::<1, 8>::new_from_preset_profile(
                PresetEqualizerProfile::TrebleReducer,
                [Vec::new()],
            ),
        }
        .into_packet()
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
            equalizer_configuration: &EqualizerConfiguration::<2, 8>::new_from_preset_profile(
                PresetEqualizerProfile::TrebleReducer,
                [Vec::new(), Vec::new()],
            ),
        };
        assert_eq!(EXPECTED, packet.into_packet().bytes());
    }
}
