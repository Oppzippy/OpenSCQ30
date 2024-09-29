use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
};

use crate::devices::standard::{
    packets::parsing::ParseResult,
    structures::{
        AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, SoundModes, TransparencyMode,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundModeUpdatePacket {
    pub ambient_sound_mode: AmbientSoundMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub transparency_mode: TransparencyMode,
    pub custom_noise_canceling: CustomNoiseCanceling,
}

impl SoundModeUpdatePacket {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<SoundModeUpdatePacket, E> {
        // offset 9
        context(
            "SoundModeUpdatePacket",
            all_consuming(map(SoundModes::take, |sound_modes| SoundModeUpdatePacket {
                ambient_sound_mode: sound_modes.ambient_sound_mode,
                noise_canceling_mode: sound_modes.noise_canceling_mode,
                transparency_mode: sound_modes.transparency_mode,
                custom_noise_canceling: sound_modes.custom_noise_canceling,
            })),
        )(input)
        // offset 13
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::{
        packets::inbound::{take_inbound_packet_body, SoundModeUpdatePacket},
        structures::{AmbientSoundMode, NoiseCancelingMode},
    };

    #[test]
    fn it_parses_valid_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x02, 0x02, 0x01, 0x00, 0x23,
        ];
        let (_, body) = take_inbound_packet_body(input).unwrap();
        let packet = SoundModeUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;
        assert_eq!(AmbientSoundMode::Normal, packet.ambient_sound_mode);
        assert_eq!(NoiseCancelingMode::Indoor, packet.noise_canceling_mode);
    }

    #[test]
    fn it_falls_back_to_default_with_invalid_ambient_sound_mode() {
        let input: &[u8] = &[
            //                                                    max value of 0x02
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x03, 0x02, 0x01, 0x00, 0x24,
        ];
        let (_, body) = take_inbound_packet_body(input).unwrap();
        let (_, packet) = SoundModeUpdatePacket::take::<VerboseError<_>>(body).unwrap();
        assert_eq!(AmbientSoundMode::default(), packet.ambient_sound_mode);
    }

    #[test]
    fn it_falls_back_to_default_with_invalid_noise_canceling_mode() {
        let input: &[u8] = &[
            //                                                          max value of 0x03
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x02, 0x04, 0x01, 0x00, 0x25,
        ];
        let (_, body) = take_inbound_packet_body(input).unwrap();
        let (_, packet) = SoundModeUpdatePacket::take::<VerboseError<_>>(body).unwrap();
        assert_eq!(NoiseCancelingMode::default(), packet.noise_canceling_mode);
    }
}
