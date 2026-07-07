use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::common::{
    self,
    macros::sound_mode_enum,
    modules::sound_modes_v2,
    packet::{self, inbound::FromPacketBody},
    structures::{
        AdaptiveNoiseCancelingNamedNoiseLevel, ManualNoiseCancelingNamed, TransparencyMode,
        take_manual_adaptive_noise_canceling,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SoundModes {
    pub ambient_sound_mode: common::structures::AmbientSoundMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub adaptive_noise_canceling: AdaptiveNoiseCancelingNamedNoiseLevel,
    pub manual_noise_canceling: ManualNoiseCancelingNamed,
    pub transparency_mode: TransparencyMode,
    pub wind_noise: WindNoise,
}

impl SoundModes {
    pub fn bytes(&self) -> [u8; 6] {
        [
            self.ambient_sound_mode.id(),
            ((self.manual_noise_canceling as u8) << 4) | self.adaptive_noise_canceling as u8,
            self.transparency_mode.id(),
            self.noise_canceling_mode.byte(),
            self.wind_noise.byte(),
            0xFF, // unknown
        ]
    }
}

impl FromPacketBody for SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3968 sound modes",
            map(
                (
                    common::structures::AmbientSoundMode::take,
                    take_manual_adaptive_noise_canceling,
                    common::structures::TransparencyMode::take,
                    NoiseCancelingMode::take,
                    WindNoise::take,
                    le_u8,
                ),
                |(
                    ambient_sound_mode,
                    (manual_noise_canceling, adaptive_noise_canceling),
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise,
                    _unknown,
                )| {
                    Self {
                        ambient_sound_mode,
                        adaptive_noise_canceling,
                        manual_noise_canceling,
                        noise_canceling_mode,
                        transparency_mode,
                        wind_noise,
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
