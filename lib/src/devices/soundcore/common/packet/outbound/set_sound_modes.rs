use crate::devices::soundcore::common::{packet::Command, structures::SoundModes};

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetSoundModes(pub SoundModes);

impl SetSoundModes {
    pub const COMMAND: Command = Command([0x06, 0x81]);
}

impl OutboundPacket for SetSoundModes {
    fn command(&self) -> Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        vec![
            self.0.ambient_sound_mode.id(),
            self.0.noise_canceling_mode.id(),
            self.0.transparency_mode.id(),
            self.0.custom_noise_canceling.value(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::{
        packet::outbound::{OutboundPacketBytesExt, SetSoundModes},
        structures::{
            AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, SoundModes,
            TransparencyMode,
        },
    };

    #[test]
    fn it_matches_an_example_normal_mode_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81, 0x0e, 0x00, 0x02, 0x00, 0x01, 0x00, 0x8e,
        ];
        let packet = SetSoundModes(SoundModes {
            ambient_sound_mode: AmbientSoundMode::Normal,
            noise_canceling_mode: NoiseCancelingMode::Transport,
            transparency_mode: TransparencyMode::VocalMode,
            custom_noise_canceling: CustomNoiseCanceling::new(0),
        });
        assert_eq!(EXPECTED, packet.bytes());
    }

    #[test]
    fn it_matches_an_example_noise_canceling_mode_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81, 0x0e, 0x00, 0x00, 0x01, 0x01, 0x00, 0x8d,
        ];
        let packet = SetSoundModes(SoundModes {
            ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
            noise_canceling_mode: NoiseCancelingMode::Outdoor,
            transparency_mode: TransparencyMode::VocalMode,
            custom_noise_canceling: CustomNoiseCanceling::new(0),
        });
        assert_eq!(EXPECTED, packet.bytes());
    }
}
