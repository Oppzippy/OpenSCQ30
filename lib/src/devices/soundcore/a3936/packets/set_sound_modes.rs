use crate::devices::soundcore::{
    a3936::structures::A3936SoundModes,
    common::packet::{self, Command, outbound::IntoPacket},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct A3936SetSoundModesPacket {
    pub sound_modes: A3936SoundModes,
}

impl IntoPacket for A3936SetSoundModesPacket {
    type DirectionMarker = packet::OutboundMarker;

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
        a3936::{
            packets::set_sound_modes::A3936SetSoundModesPacket,
            structures::{
                A3936NoiseCancelingMode, A3936SoundModes, AdaptiveNoiseCanceling,
                ManualNoiseCanceling, WindNoise,
            },
        },
        common::{
            packet::outbound::IntoPacket,
            structures::{AmbientSoundMode, TransparencyMode},
        },
    };

    #[test]
    fn it_matches_an_example_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x06, 0x81, 0x10, 0x00, 0x02, 0x12, 0x00, 0x01, 0x01,
            0x02, 0xa5,
        ];
        let packet = A3936SetSoundModesPacket {
            sound_modes: A3936SoundModes {
                ambient_sound_mode: AmbientSoundMode::Normal,
                manual_noise_canceling: ManualNoiseCanceling::Weak,
                adaptive_noise_canceling: AdaptiveNoiseCanceling::HighNoise,
                transparency_mode: TransparencyMode::FullyTransparent,
                noise_canceling_mode: A3936NoiseCancelingMode::Manual,
                wind_noise: WindNoise {
                    is_detected: false,
                    is_suppression_enabled: true,
                },
                noise_canceling_adaptive_sensitivity_level: 2,
            },
        };
        assert_eq!(EXPECTED, packet.into_packet().bytes());
    }
}
