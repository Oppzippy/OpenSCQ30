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
    packet::{self, inbound::FromPacketBody},
    structures::{
        AdaptiveNoiseCanceling, ManualNoiseCanceling, WindNoise,
        manual_adaptive_noise_canceling_byte, take_manual_adaptive_noise_canceling,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, MigrationSteps)]
pub struct SoundModes {
    pub ambient_sound_mode: common::structures::AmbientSoundMode,
    #[migration_requirement(field = ambient_sound_mode, value = common::structures::AmbientSoundMode::NoiseCanceling)]
    pub noise_canceling_mode: NoiseCancelingMode,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Adaptive)]
    pub adaptive_noise_canceling: AdaptiveNoiseCanceling,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Manual)]
    pub manual_noise_canceling: ManualNoiseCanceling,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::MultiScene)]
    pub multi_scene_anc: common::structures::NoiseCancelingMode,
    #[migration_requirement(
        field = ambient_sound_mode,
        value = common::structures::AmbientSoundMode::NoiseCanceling,
        value2 = common::structures::AmbientSoundMode::Transparency,
    )]
    pub wind_noise: WindNoise,
    pub noise_canceling_adaptive_sensitivity_level: u8,
}

impl SoundModes {
    pub fn bytes(&self) -> [u8; 7] {
        [
            self.ambient_sound_mode.id(),
            manual_adaptive_noise_canceling_byte(
                self.manual_noise_canceling,
                self.adaptive_noise_canceling,
            ),
            self.ambient_sound_mode.id(),
            self.noise_canceling_mode.byte(), // ANC automation mode?
            self.wind_noise.byte(),
            self.noise_canceling_adaptive_sensitivity_level,
            self.multi_scene_anc.id(),
        ]
    }
}

impl FromPacketBody for SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3959 sound modes",
            map(
                (
                    common::structures::AmbientSoundMode::take,
                    take_manual_adaptive_noise_canceling,
                    common::structures::AmbientSoundMode::take,
                    NoiseCancelingMode::take,
                    WindNoise::take,
                    le_u8,
                    common::structures::NoiseCancelingMode::take,
                ),
                |(
                    ambient_sound_mode,
                    (manual_noise_canceling, adaptive_noise_canceling),
                    _ambient_sound_mode,
                    noise_canceling_mode,
                    wind_noise,
                    noise_canceling_adaptive_sensitivity_level,
                    multi_scene_anc,
                )| {
                    Self {
                        ambient_sound_mode,
                        adaptive_noise_canceling,
                        manual_noise_canceling,
                        noise_canceling_mode,
                        wind_noise,
                        noise_canceling_adaptive_sensitivity_level,
                        multi_scene_anc,
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
        MultiScene = 2,
    }
);
