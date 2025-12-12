use crate::devices::soundcore::{
    a3955,
    common::packet::{self, outbound::ToPacket},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct A3955SetSoundModes {
    pub sound_modes: a3955::structures::SoundModes,
}

impl ToPacket for A3955SetSoundModes {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        packet::Command([0x06, 0x81])
    }

    fn body(&self) -> Vec<u8> {
        self.sound_modes.bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::{a3955, common};

    use super::*;

    #[test]
    fn it_matches_an_example_packet() {
        const EXPECTED: &[u8] = &[8, 238, 0, 0, 0, 6, 129, 17, 0, 0, 85, 0, 2, 1, 0, 1, 231];
        let packet = A3955SetSoundModes {
            sound_modes: a3955::structures::SoundModes {
                ambient_sound_mode: common::structures::AmbientSoundMode::NoiseCanceling,
                manual_noise_canceling: a3955::structures::ManualNoiseCanceling::new(5),
                adaptive_noise_canceling: a3955::structures::AdaptiveNoiseCanceling::new(5),
                transparency_mode: common::structures::TransparencyMode::FullyTransparent,
                noise_canceling_mode: a3955::structures::NoiseCancelingMode::MultiScene,
                wind_noise: a3955::structures::WindNoise {
                    is_suppression_enabled: true,
                    is_detected: false,
                },
                noise_canceling_adaptive_sensitivity_level: 0,
                multi_scene_anc: common::structures::NoiseCancelingMode::Outdoor,
            },
        };
        assert_eq!(EXPECTED, packet.to_packet().bytes());
    }
}
