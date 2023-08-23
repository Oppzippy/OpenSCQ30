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
        // TODO maybe a bug? DRC is clamped to -120-120 meaning the range will be 0-240 rather than 0-255.
        // The clamping is likely unnecessary anyway.
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
            0xf0, 0x8e, 0x00, 0x74, 0x88, 0xdb, 0xf0, 0xe0, 0xf0, 0xec, 0xd7, 0xef, 0xe4, 0xbd,
        ];

        let packet = SetEqualizerWithDrcPacket::new(
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                -60, 60, 23, 120, 22, -120, -4, 16,
            ])),
            None,
        );
        assert_eq!(EXPECTED, packet.bytes());
    }
}
