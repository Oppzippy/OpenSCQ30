use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::common::{
    packet::{self, Command},
    structures,
};

use super::FromPacketBody;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundModes(pub structures::SoundModes);

impl SoundModes {
    pub const COMMAND: Command = Command([0x06, 0x01]);
}

impl FromPacketBody for SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        // offset 9
        context(
            "SoundModeUpdatePacket",
            all_consuming(map(structures::SoundModes::take, |sound_modes| {
                Self(structures::SoundModes {
                    ambient_sound_mode: sound_modes.ambient_sound_mode,
                    noise_canceling_mode: sound_modes.noise_canceling_mode,
                    transparency_mode: sound_modes.transparency_mode,
                    custom_noise_canceling: sound_modes.custom_noise_canceling,
                })
            })),
        )
        .parse_complete(input)
        // offset 13
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::common::{
        packet::{
            self,
            inbound::{FromPacketBody, SoundModes},
        },
        structures::{AmbientSoundMode, NoiseCancelingMode},
    };

    #[test]
    fn it_parses_valid_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x02, 0x02, 0x01, 0x00, 0x23,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let packet = SoundModes::take::<VerboseError<_>>(&packet.body).unwrap().1;
        assert_eq!(AmbientSoundMode::Normal, packet.0.ambient_sound_mode);
        assert_eq!(NoiseCancelingMode::Indoor, packet.0.noise_canceling_mode);
    }

    #[test]
    fn it_falls_back_to_default_with_invalid_ambient_sound_mode() {
        let input: &[u8] = &[
            //                                                    max value of 0x02
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x03, 0x02, 0x01, 0x00, 0x24,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let (_, packet) = SoundModes::take::<VerboseError<_>>(&packet.body).unwrap();
        assert_eq!(AmbientSoundMode::default(), packet.0.ambient_sound_mode);
    }

    #[test]
    fn it_falls_back_to_default_with_invalid_noise_canceling_mode() {
        let input: &[u8] = &[
            //                                                          max value of 0x03
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x02, 0x04, 0x01, 0x00, 0x25,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let (_, packet) = SoundModes::take::<VerboseError<_>>(&packet.body).unwrap();
        assert_eq!(NoiseCancelingMode::default(), packet.0.noise_canceling_mode);
    }
}
