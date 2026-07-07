use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::common::{
    macros::sound_mode_enum,
    modules::sound_modes_v2::ToPacketBody,
    packet::{self, inbound::FromPacketBody, parsing::take_bool},
    structures::{
        AdaptiveNoiseCancelingNamedStrength, AmbientSoundMode, ManualNoiseCanceling, WindNoise,
        manual_adaptive_noise_canceling_byte, take_manual_adaptive_noise_canceling,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub manual_noise_canceling: ManualNoiseCanceling,
    pub adaptive_noise_canceling: AdaptiveNoiseCancelingNamedStrength,
    pub transparency_mode: TransparencyMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub wind_noise: WindNoise,
    pub multi_scene_anc: MultiSceneAnc,
    pub real_time_adaptive_anc: bool,
}

impl SoundModes {
    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [
            self.ambient_sound_mode.byte(),
            manual_adaptive_noise_canceling_byte(
                self.manual_noise_canceling,
                self.adaptive_noise_canceling,
            ),
            self.transparency_mode.byte(),
            self.noise_canceling_mode.byte(),
            self.wind_noise.byte(),
            0,
            self.multi_scene_anc.byte(),
            u8::from(self.real_time_adaptive_anc),
        ]
        .into_iter()
    }
}

impl ToPacketBody for SoundModes {
    fn bytes(&self) -> Vec<u8> {
        self.bytes().collect()
    }
}

impl FromPacketBody for SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    take_manual_adaptive_noise_canceling,
                    TransparencyMode::take,
                    NoiseCancelingMode::take,
                    WindNoise::take,
                    le_u8,
                    MultiSceneAnc::take,
                    take_bool,
                ),
                |(
                    ambient_sound_mode,
                    (manual_noise_canceling, adaptive_noise_canceling),
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise,
                    _unknown,
                    multi_scene_anc,
                    real_time_adaptive_anc,
                )| Self {
                    ambient_sound_mode,
                    transparency_mode,
                    noise_canceling_mode,
                    multi_scene_anc,
                    real_time_adaptive_anc,
                    manual_noise_canceling,
                    adaptive_noise_canceling,
                    wind_noise,
                },
            ),
        )
        .parse_complete(input)
    }
}

sound_mode_enum!(
    pub enum TransparencyMode {
        FullyTransparent = 0,
        VocalMode = 1,
    }
);

sound_mode_enum!(
    pub enum NoiseCancelingMode {
        Manual = 0,
        Adaptive = 1,
        MultiScene = 2,
    }
);

sound_mode_enum!(
    pub enum MultiSceneAnc {
        Transport = 0,
        Outdoor = 1,
        Indoor = 2,
    }
);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpatialAudio {
    pub is_enabled: bool,
    pub mode: SpatialAudioMode,
}

impl SpatialAudio {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "spatial audio",
            map((take_bool, SpatialAudioMode::take), |(is_enabled, mode)| {
                Self { is_enabled, mode }
            }),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [u8::from(self.is_enabled), self.mode as u8].into_iter()
    }
}

sound_mode_enum!(
    pub enum SpatialAudioMode {
        Music = 0,
        Movie = 2,
        Gaming = 3,
    }
);
