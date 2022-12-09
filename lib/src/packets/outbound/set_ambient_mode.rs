use crate::packets::structures::{AmbientSoundMode, NoiseCancelingMode};

use super::{outbound_packet::OutboundPacket, utils::calculate_checksum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetAmbientSoundModePacket {
    ambient_sound_mode: AmbientSoundMode,
    noise_canceling_mode: NoiseCancelingMode,
}

impl SetAmbientSoundModePacket {
    pub fn new(
        ambient_sound_mode: AmbientSoundMode,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> Self {
        SetAmbientSoundModePacket {
            ambient_sound_mode,
            noise_canceling_mode,
        }
    }
}

impl OutboundPacket for SetAmbientSoundModePacket {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![
            0x08,
            0xee,
            0x00,
            0x00,
            0x00,
            0x06,
            0x81,
            0x0e,
            0x00,
            self.ambient_sound_mode.id(),
            self.noise_canceling_mode.id(),
            0x01,
            0x00,
        ];
        bytes.push(calculate_checksum(&bytes));

        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{
        outbound::{OutboundPacket, SetAmbientSoundModePacket},
        structures::{AmbientSoundMode, NoiseCancelingMode},
    };

    #[test]
    fn it_matches_an_example_normal_mode_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81, 0x0e, 0x00, 0x02, 0x00, 0x01, 0x00, 0x8e,
        ];
        let packet =
            SetAmbientSoundModePacket::new(AmbientSoundMode::Normal, NoiseCancelingMode::Transport);
        assert_eq!(EXPECTED, packet.bytes());
    }

    #[test]
    fn it_matches_an_example_noise_canceling_mode_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81, 0x0e, 0x00, 0x00, 0x01, 0x01, 0x00, 0x8d,
        ];
        let packet = SetAmbientSoundModePacket::new(
            AmbientSoundMode::NoiseCanceling,
            NoiseCancelingMode::Outdoor,
        );
        assert_eq!(EXPECTED, packet.bytes());
    }
}
