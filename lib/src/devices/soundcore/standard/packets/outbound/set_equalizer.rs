use crate::devices::soundcore::standard::structures::{Command, EqualizerConfiguration};

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, PartialEq)]
pub struct SetEqualizerPacket<'a> {
    left_configuration: &'a EqualizerConfiguration,
    right_configuration: Option<&'a EqualizerConfiguration>,
}

impl<'a> SetEqualizerPacket<'a> {
    pub const COMMAND: Command = Command::new([0x08, 0xEE, 0x00, 0x00, 0x00, 0x02, 0x81]);
    pub fn new(
        left_configuration: &'a EqualizerConfiguration,
        right_configuration: Option<&'a EqualizerConfiguration>,
    ) -> Self {
        Self {
            left_configuration,
            right_configuration,
        }
    }
}

impl OutboundPacket for SetEqualizerPacket<'_> {
    fn command(&self) -> Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(22);
        bytes.extend(self.left_configuration.profile_id().to_le_bytes());
        bytes.extend(self.left_configuration.volume_adjustments().bytes());
        if let Some(right_eq) = self.right_configuration {
            bytes.extend(right_eq.volume_adjustments().bytes());
        }

        bytes
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
        let actual = SetEqualizerPacket::new(
            &EqualizerConfiguration::new_custom_profile(
                VolumeAdjustments::new([-6.0, 6.0, 2.3, 4.0, 2.2, 6.0, -0.4, 1.6]).unwrap(),
            ),
            None,
        )
        .bytes();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_matches_an_example_soundcore_signature_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x00, 0x00, 0x78, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x4d,
        ];
        let actual = SetEqualizerPacket::new(
            &EqualizerConfiguration::new_from_preset_profile(
                PresetEqualizerProfile::SoundcoreSignature,
                [],
            ),
            None,
        )
        .bytes();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_matches_an_example_treble_reducer_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x15, 0x00, 0x78, 0x78, 0x78,
            0x64, 0x5a, 0x50, 0x50, 0x3c, 0xa4,
        ];
        let actual = SetEqualizerPacket::new(
            &EqualizerConfiguration::new_from_preset_profile(
                PresetEqualizerProfile::TrebleReducer,
                [],
            ),
            None,
        )
        .bytes();
        assert_eq!(EXPECTED, actual);
    }

    #[test]
    fn it_sends_second_channel_if_present() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x1C, 0x00, 0x15, 0x00, 0x78, 0x78, 0x78,
            0x64, 0x5a, 0x50, 0x50, 0x3c, 0x78, 0x78, 0x78, 0x64, 0x5a, 0x50, 0x50, 0x3c, 0xae,
        ];
        let configuration = EqualizerConfiguration::new_from_preset_profile(
            PresetEqualizerProfile::TrebleReducer,
            [],
        );
        let packet = SetEqualizerPacket::new(&configuration, Some(&configuration));
        assert_eq!(EXPECTED, packet.bytes());
    }
}
