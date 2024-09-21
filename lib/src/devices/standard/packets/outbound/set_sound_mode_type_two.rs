use crate::devices::standard::structures::SoundModesTypeTwo;

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetSoundModeTypeTwoPacket {
    pub sound_modes: SoundModesTypeTwo,
}

impl OutboundPacket for SetSoundModeTypeTwoPacket {
    fn command(&self) -> [u8; 7] {
        [0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81]
    }

    fn body(&self) -> Vec<u8> {
        vec![
            self.sound_modes.ambient_sound_mode.id(),
            (self.sound_modes.manual_noise_canceling.id() << 4)
                | self.sound_modes.adaptive_noise_canceling.id(),
            self.sound_modes.transparency_mode.id(),
            self.sound_modes.noise_canceling_mode.id(), // ANC automation mode?
            self.sound_modes.wind_noise_suppression.into(),
            self.sound_modes.noise_canceling_adaptive_sensitivity_level,
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::standard::{
        packets::outbound::{OutboundPacketBytes, SetSoundModeTypeTwoPacket},
        structures::{
            AdaptiveNoiseCanceling, AmbientSoundMode, ManualNoiseCanceling,
            NoiseCancelingModeTypeTwo, SoundModesTypeTwo, TransparencyMode,
        },
    };

    #[test]
    fn it_matches_an_example_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81, 0x10, 0x00, 0x02, 0x12, 0x00, 0x01, 0x01,
            0x02, 0xa5,
        ];
        let packet = SetSoundModeTypeTwoPacket {
            sound_modes: SoundModesTypeTwo {
                ambient_sound_mode: AmbientSoundMode::Normal,
                manual_noise_canceling: ManualNoiseCanceling::Weak,
                adaptive_noise_canceling: AdaptiveNoiseCanceling::HighNoise,
                transparency_mode: TransparencyMode::FullyTransparent,
                noise_canceling_mode: NoiseCancelingModeTypeTwo::Manual,
                wind_noise_suppression: true,
                noise_canceling_adaptive_sensitivity_level: 2,
            },
        };
        assert_eq!(EXPECTED, packet.bytes());
    }
}
