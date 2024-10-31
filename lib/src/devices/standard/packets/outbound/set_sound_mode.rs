use crate::devices::standard::structures::{
    AmbientSoundMode, Command, CustomNoiseCanceling, NoiseCancelingMode, TransparencyMode,
};

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetSoundModePacket {
    pub ambient_sound_mode: AmbientSoundMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub transparency_mode: TransparencyMode,
    pub custom_noise_canceling: CustomNoiseCanceling,
}

impl OutboundPacket for SetSoundModePacket {
    fn command(&self) -> Command {
        Command::new([0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81])
    }

    fn body(&self) -> Vec<u8> {
        vec![
            self.ambient_sound_mode.id(),
            self.noise_canceling_mode.id(),
            self.transparency_mode.id(),
            self.custom_noise_canceling.value(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::standard::{
        packets::outbound::{OutboundPacketBytesExt, SetSoundModePacket},
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
