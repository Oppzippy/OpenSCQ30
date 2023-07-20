use crate::packets::{
    checksum::calculate_checksum,
    structures::{AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, TransparencyMode},
};

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetSoundModePacket {
    pub ambient_sound_mode: AmbientSoundMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub transparency_mode: TransparencyMode,
    pub custom_noise_canceling: CustomNoiseCanceling,
}

impl OutboundPacket for SetSoundModePacket {
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
            self.transparency_mode.id(),
            self.custom_noise_canceling.value(),
        ];
        bytes.push(calculate_checksum(&bytes));

        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{
        outbound::{OutboundPacket, SetSoundModePacket},
        structures::{
            AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, TransparencyMode,
        },
    };

    #[test]
    fn it_matches_an_example_normal_mode_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81, 0x0e, 0x00, 0x02, 0x00, 0x01, 0x00, 0x8e,
        ];
        let packet = SetSoundModePacket {
            ambient_sound_mode: AmbientSoundMode::Normal,
            noise_canceling_mode: NoiseCancelingMode::Transport,
            transparency_mode: TransparencyMode::VocalMode,
            custom_noise_canceling: CustomNoiseCanceling::new(0),
        };
        assert_eq!(EXPECTED, packet.bytes());
    }

    #[test]
    fn it_matches_an_example_noise_canceling_mode_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81, 0x0e, 0x00, 0x00, 0x01, 0x01, 0x00, 0x8d,
        ];
        let packet = SetSoundModePacket {
            ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
            noise_canceling_mode: NoiseCancelingMode::Outdoor,
            transparency_mode: TransparencyMode::VocalMode,
            custom_noise_canceling: CustomNoiseCanceling::new(0),
        };
        assert_eq!(EXPECTED, packet.bytes());
    }
}
