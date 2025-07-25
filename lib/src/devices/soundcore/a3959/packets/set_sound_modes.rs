use crate::devices::soundcore::{
    a3959::structures::A3959SoundModes,
    standard::packet::{Command, outbound::OutboundPacket},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct A3959SetSoundModesPacket {
    pub sound_modes: A3959SoundModes,
}

impl OutboundPacket for A3959SetSoundModesPacket {
    fn command(&self) -> Command {
        Command([0x06, 0x81])
    }

    fn body(&self) -> Vec<u8> {
        self.sound_modes.bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::{
        a3959::{
            packets::set_sound_modes::A3959SetSoundModesPacket,
            structures::{
                A3959NoiseCancelingMode, A3959SoundModes, AdaptiveNoiseCanceling,
                ManualNoiseCanceling, WindNoise,
            },
        },
        standard::{
            packet::outbound::OutboundPacketBytesExt,
            structures::{AmbientSoundMode, NoiseCancelingMode, TransparencyMode},
        },
    };

    #[test]
    fn it_matches_an_example_packet() {
        const EXPECTED: &[u8] = &[8, 238, 0, 0, 0, 6, 129, 17, 0, 0, 85, 0, 2, 1, 0, 1, 231];
        let packet = A3959SetSoundModesPacket {
            sound_modes: A3959SoundModes {
                ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
                manual_noise_canceling: ManualNoiseCanceling::new(5),
                adaptive_noise_canceling: AdaptiveNoiseCanceling::new(5),
                transparency_mode: TransparencyMode::FullyTransparent,
                noise_canceling_mode: A3959NoiseCancelingMode::MultiScene,
                wind_noise: WindNoise {
                    is_suppression_enabled: true,
                    is_detected: false,
                },
                noise_canceling_adaptive_sensitivity_level: 0,
                multi_scene_anc: NoiseCancelingMode::Outdoor,
            },
        };
        assert_eq!(EXPECTED, packet.bytes());
    }
}
