use crate::packets::structures::EqualizerConfiguration;

use super::{outbound_packet::OutboundPacket, utils};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetEqualizerPacket {
    configuration: EqualizerConfiguration,
}

impl SetEqualizerPacket {
    pub fn new(configuration: EqualizerConfiguration) -> Self {
        Self { configuration }
    }
}

impl OutboundPacket for SetEqualizerPacket {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0x08, 0xEE, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00];

        bytes.extend(self.configuration.profile_id().to_le_bytes());
        bytes.extend(self.configuration.band_offsets().bytes());

        bytes.push(utils::calculate_checksum(&bytes));

        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{
        outbound::OutboundPacket,
        structures::{EqualizerBandOffsets, EqualizerConfiguration, PresetEqualizerProfile},
    };

    use super::SetEqualizerPacket;

    #[test]
    fn it_matches_an_example_custom_eq_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0xfe, 0xfe, 0x3c, 0xb4, 0x8f,
            0xa0, 0x8e, 0xb4, 0x74, 0x88, 0xe6,
        ];
        let packet = SetEqualizerPacket::new(EqualizerConfiguration::new_custom_profile(
            EqualizerBandOffsets::new([-60, 60, 23, 40, 22, 60, -4, 16]),
        ));
        assert_eq!(EXPECTED, packet.bytes());
    }

    #[test]
    fn it_matches_an_example_soundcore_signature_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x00, 0x00, 0x78, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x4d,
        ];
        let packet = SetEqualizerPacket::new(EqualizerConfiguration::new_from_preset_profile(
            PresetEqualizerProfile::SoundcoreSignature,
        ));
        assert_eq!(EXPECTED, packet.bytes());
    }

    #[test]
    fn it_matches_an_example_treble_reducer_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x15, 0x00, 0x78, 0x78, 0x78,
            0x64, 0x5a, 0x50, 0x50, 0x3c, 0xa4,
        ];
        let packet = SetEqualizerPacket::new(EqualizerConfiguration::new_from_preset_profile(
            PresetEqualizerProfile::TrebleReducer,
        ));
        assert_eq!(EXPECTED, packet.bytes());
    }
}
