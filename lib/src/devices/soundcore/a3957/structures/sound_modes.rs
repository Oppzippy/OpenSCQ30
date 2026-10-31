use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_lib_macros::MigrationSteps;

use crate::devices::soundcore::common::{
    self,
    macros::sound_mode_enum,
    modules::sound_modes_v2,
    packet::{self, inbound::FromPacketBody, parsing::take_bool},
    structures::{
        AdaptiveNoiseCancelingNamedStrength, ManualNoiseCanceling, WindNoise,
        manual_adaptive_noise_canceling_byte, take_manual_adaptive_noise_canceling,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, MigrationSteps)]
pub struct SoundModes {
    pub ambient_sound_mode: common::structures::AmbientSoundMode,
    // Liberty 5 has no transparency sub-mode; this byte is In-Call Sound Awareness.
    pub in_call_sound_awareness: bool,
    #[migration_requirement(field = ambient_sound_mode, value = common::structures::AmbientSoundMode::NoiseCanceling)]
    pub noise_canceling_mode: NoiseCancelingMode,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Adaptive)]
    pub adaptive_noise_canceling: AdaptiveNoiseCancelingNamedStrength,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Manual)]
    pub manual_noise_canceling: ManualNoiseCanceling,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Transportation)]
    pub transportation_mode: TransportationMode,
    #[migration_requirement(
        field = ambient_sound_mode,
        value = common::structures::AmbientSoundMode::NoiseCanceling,
        value2 = common::structures::AmbientSoundMode::Transparency,
    )]
    pub wind_noise: WindNoise,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Adaptive)]
    pub noise_canceling_adaptive_sensitivity_level: u8,
}

impl SoundModes {
    pub fn bytes(&self) -> [u8; 7] {
        [
            self.ambient_sound_mode.byte(),
            manual_adaptive_noise_canceling_byte(
                self.manual_noise_canceling,
                self.adaptive_noise_canceling,
            ),
            u8::from(self.in_call_sound_awareness),
            self.noise_canceling_mode.byte(),
            self.wind_noise.byte(),
            self.noise_canceling_adaptive_sensitivity_level,
            self.transportation_mode.byte(),
        ]
    }
}

impl FromPacketBody for SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3957 sound modes",
            map(
                (
                    common::structures::AmbientSoundMode::take,
                    take_manual_adaptive_noise_canceling,
                    take_bool,
                    NoiseCancelingMode::take,
                    WindNoise::take,
                    le_u8,
                    TransportationMode::take,
                ),
                |(
                    ambient_sound_mode,
                    (manual_noise_canceling, adaptive_noise_canceling),
                    in_call_sound_awareness,
                    noise_canceling_mode,
                    wind_noise,
                    noise_canceling_adaptive_sensitivity_level,
                    transportation_mode,
                )| {
                    Self {
                        ambient_sound_mode,
                        in_call_sound_awareness,
                        adaptive_noise_canceling,
                        manual_noise_canceling,
                        noise_canceling_mode,
                        wind_noise,
                        noise_canceling_adaptive_sensitivity_level,
                        transportation_mode,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl sound_modes_v2::ToPacketBody for SoundModes {
    fn bytes(&self) -> Vec<u8> {
        self.bytes().to_vec()
    }
}

sound_mode_enum!(
    pub enum NoiseCancelingMode {
        Manual = 0,
        Adaptive = 1,
        Transportation = 2,
    }
);

sound_mode_enum!(
    pub enum TransportationMode {
        Plane = 0,
        Car = 3,
    }
);

#[cfg(test)]
mod tests {
    use super::SoundModes;
    use crate::devices::soundcore::common::packet::inbound::FromPacketBody;

    #[test]
    fn in_call_sound_awareness_round_trip() {
        let cases: [([u8; 7], bool); 2] = [
            ([2, 0x50, 0, 0, 1, 0, 3], false),
            ([2, 0x50, 1, 0, 1, 0, 3], true),
        ];
        for (wire, expected) in cases {
            let (_, parsed) = SoundModes::take::<nom::error::Error<&[u8]>>(&wire).unwrap();
            assert_eq!(parsed.in_call_sound_awareness, expected);
            assert_eq!(parsed.bytes(), wire);
        }
    }
}
