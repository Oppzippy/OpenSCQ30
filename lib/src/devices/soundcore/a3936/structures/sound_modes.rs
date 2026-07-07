use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::common::{
    macros::sound_mode_enum,
    modules::sound_modes_v2,
    packet::{self, inbound::FromPacketBody},
    structures::{
        AdaptiveNoiseCancelingNamedNoiseLevel, AmbientSoundMode, ManualNoiseCancelingNamed,
        TransparencyMode, manual_adaptive_noise_canceling_byte,
        take_manual_adaptive_noise_canceling,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct A3936SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub transparency_mode: TransparencyMode,
    pub noise_canceling_mode: A3936NoiseCancelingMode,
    pub adaptive_noise_canceling: AdaptiveNoiseCancelingNamedNoiseLevel,
    pub manual_noise_canceling: ManualNoiseCancelingNamed,
    pub wind_noise: WindNoise,
    pub noise_canceling_adaptive_sensitivity_level: u8,
}

impl A3936SoundModes {
    pub fn bytes(&self) -> [u8; 6] {
        [
            self.ambient_sound_mode.id(),
            manual_adaptive_noise_canceling_byte(
                self.manual_noise_canceling,
                self.adaptive_noise_canceling,
            ),
            self.transparency_mode.id(),
            self.noise_canceling_mode.byte(),
            self.wind_noise.byte(),
            self.noise_canceling_adaptive_sensitivity_level,
        ]
    }
}

impl FromPacketBody for A3936SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3936 sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    take_manual_adaptive_noise_canceling,
                    TransparencyMode::take,
                    A3936NoiseCancelingMode::take,
                    WindNoise::take,
                    le_u8,
                ),
                |(
                    ambient_sound_mode,
                    (manual_noise_canceling, adaptive_noise_canceling),
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise,
                    noise_canceling_adaptive_sensitivity_level,
                )| {
                    Self {
                        ambient_sound_mode,
                        transparency_mode,
                        adaptive_noise_canceling,
                        manual_noise_canceling,
                        noise_canceling_mode,
                        wind_noise,
                        noise_canceling_adaptive_sensitivity_level,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl sound_modes_v2::ToPacketBody for A3936SoundModes {
    fn bytes(&self) -> Vec<u8> {
        self.bytes().to_vec()
    }
}

sound_mode_enum!(
    pub enum A3936NoiseCancelingMode {
        Manual = 0,
        Adaptive = 1,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WindNoise {
    pub is_suppression_enabled: bool,
    pub is_detected: bool,
}

impl WindNoise {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "wind noise",
            map(le_u8, |wind_noise| Self {
                is_suppression_enabled: wind_noise & 1 != 0,
                is_detected: wind_noise & 2 != 0,
            }),
        )
        .parse_complete(input)
    }

    pub fn byte(&self) -> u8 {
        u8::from(self.is_suppression_enabled) | (u8::from(self.is_detected) << 1)
    }
}
