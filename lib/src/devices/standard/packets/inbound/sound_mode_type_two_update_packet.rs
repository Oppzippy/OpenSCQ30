use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
};

use crate::devices::standard::{
    packets::parsing::ParseResult,
    structures::{take_sound_modes_type_two, SoundModesTypeTwo},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundModeTypeTwoUpdatePacket {
    pub sound_modes: SoundModesTypeTwo,
}

pub fn take_sound_mode_type_two_update_packet<
    'a,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
>(
    input: &'a [u8],
) -> ParseResult<SoundModeTypeTwoUpdatePacket, E> {
    // offset 9
    context(
        "SoundModeTypeTwoUpdatePacket",
        all_consuming(map(take_sound_modes_type_two, |sound_modes| {
            SoundModeTypeTwoUpdatePacket { sound_modes }
        })),
    )(input)
    // offset 15
}

#[cfg(test)]

mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::{
        packets::inbound::{take_inbound_packet_body, take_sound_mode_type_two_update_packet},
        structures::{
            AdaptiveNoiseCanceling, AmbientSoundMode, ManualNoiseCanceling,
            NoiseCancelingModeTypeTwo, TransparencyMode,
        },
    };

    #[test]
    fn it_parses_valid_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x02, 0x22, 0x01, 0x01, 0x03,
            0x05, 0x4C,
        ];
        let (_, body) = take_inbound_packet_body(input).unwrap();
        let sound_modes = take_sound_mode_type_two_update_packet::<VerboseError<_>>(body)
            .unwrap()
            .1
            .sound_modes;
        assert_eq!(AmbientSoundMode::Normal, sound_modes.ambient_sound_mode);
        assert_eq!(
            NoiseCancelingModeTypeTwo::Manual,
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
        assert_eq!(true, sound_modes.wind_noise_suppression);
        assert_eq!(5, sound_modes.noise_canceling_adaptive_sensitivity_level);
    }
}
