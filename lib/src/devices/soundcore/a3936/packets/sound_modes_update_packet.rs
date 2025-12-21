use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a3936::structures::A3936SoundModes,
    common::packet::{self, Command, inbound::FromPacketBody},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct A3936SoundModesUpdatePacket {
    pub sound_modes: A3936SoundModes,
}

impl A3936SoundModesUpdatePacket {
    pub const COMMAND: Command = Command([0x06, 0x01]);
}

impl FromPacketBody for A3936SoundModesUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        // offset 9
        context(
            "A3936SoundModesUpdatePacket",
            all_consuming(map(A3936SoundModes::take, |sound_modes| Self {
                sound_modes,
            })),
        )
        .parse_complete(input)
        // offset 15
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::{
        a3936::{
            packets::A3936SoundModesUpdatePacket,
            structures::{A3936NoiseCancelingMode, AdaptiveNoiseCanceling, ManualNoiseCanceling},
        },
        common::{
            packet::{self, inbound::FromPacketBody},
            structures::{AmbientSoundMode, TransparencyMode},
        },
    };

    #[test]
    fn it_parses_a_known_good_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x10, 0x00, 0x01, 0x30, 0x00, 0x01, 0x00,
            0x00, 0x52,
        ];
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(input).unwrap();
        A3936SoundModesUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .expect("parsing should succeed");
    }

    #[test]
    fn it_parses_valid_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x10, 0x00, 0x02, 0x22, 0x01, 0x01, 0x03,
            0x05, 0x4e,
        ];
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(input).unwrap();
        let sound_modes = A3936SoundModesUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .unwrap()
            .1
            .sound_modes;
        assert_eq!(AmbientSoundMode::Normal, sound_modes.ambient_sound_mode);
        assert_eq!(
            A3936NoiseCancelingMode::Adaptive,
            sound_modes.noise_canceling_mode
        );
        assert_eq!(
            ManualNoiseCanceling::Moderate,
            sound_modes.manual_noise_canceling
        );
        assert_eq!(
            AdaptiveNoiseCanceling::HighNoise,
            sound_modes.adaptive_noise_canceling
        );
        assert_eq!(TransparencyMode::VocalMode, sound_modes.transparency_mode);
        assert!(sound_modes.wind_noise.is_suppression_enabled);
        assert_eq!(5, sound_modes.noise_canceling_adaptive_sensitivity_level);
    }
}
