use crate::packets::structures::EqualizerConfiguration;

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetEqualizerWithDrcPacket {
    left_configuration: EqualizerConfiguration,
    right_configuration: Option<EqualizerConfiguration>,
}

impl SetEqualizerWithDrcPacket {
    pub fn new(
        left_configuration: EqualizerConfiguration,
        right_configuration: Option<EqualizerConfiguration>,
    ) -> Self {
        Self {
            left_configuration,
            right_configuration,
        }
    }
}

impl OutboundPacket for SetEqualizerWithDrcPacket {
    fn command(&self) -> [u8; 7] {
        [0x08, 0xEE, 0x00, 0x00, 0x00, 0x02, 0x83]
    }

    fn body(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(34);

        // profile
        bytes.extend(self.left_configuration.profile_id().to_le_bytes());
        // eq without drc
        bytes.extend(self.left_configuration.volume_adjustments().bytes());
        if let Some(right_eq) = self.right_configuration {
            bytes.extend(right_eq.volume_adjustments().bytes());
        }
        // eq with drc
        bytes.extend(
            self.left_configuration
                .volume_adjustments()
                .apply_drc()
                .bytes(),
        );
        if let Some(right_eq) = self.right_configuration {
            bytes.extend(right_eq.volume_adjustments().apply_drc().bytes());
        }

        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{
        outbound::{OutboundPacketBytes, SetEqualizerWithDrcPacket},
        structures::{EqualizerConfiguration, VolumeAdjustments},
    };

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x83, 0x1c, 0x00, 0xfe, 0xfe, 0x3c, 0xb4, 0x8f,
            0xf0, 0x8e, 0x00, 0x74, 0x88, 0x6d, 0x86, 0x70, 0x88, 0x7b, 0x66, 0x7e, 0x79, 0x4f,
        ];

        let packet = SetEqualizerWithDrcPacket::new(
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                -6.0, 6.0, 2.3, 12.0, 2.2, -12.0, -0.4, 1.6,
            ])),
            None,
        );
        assert_eq!(EXPECTED, packet.bytes());
    }
}
