use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use openscq30_lib_macros::MigrationSteps;
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::devices::soundcore::common::{
    modules::sound_modes_v2,
    packet::parsing::take_bool,
    structures::{AmbientSoundMode, flag},
};

flag!(VoicePrompt);
flag!(LowBatteryPrompt);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, MigrationSteps)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    #[migration_requirement(field = ambient_sound_mode, value = AmbientSoundMode::NoiseCanceling)]
    pub noise_canceling_mode: NoiseCancelingMode,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Manual)]
    pub manual_noise_canceling: ManualNoiseCanceling,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Adaptive)]
    pub adaptive_noise_canceling: AdaptiveNoiseCanceling,
    #[migration_requirement(field = ambient_sound_mode, value = AmbientSoundMode::Transparency)]
    pub transparency_mode: TransparencyMode,
    #[migration_requirement(field = transparency_mode, value = TransparencyMode::Manual)]
    pub manual_transparency: ManualTransparency,
    #[migration_requirement(
        field = ambient_sound_mode,
        value = AmbientSoundMode::NoiseCanceling,
        value2 = AmbientSoundMode::Transparency,
    )]
    pub wind_noise_reduction: bool,
}

impl SoundModes {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3040 sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    take_manual_and_adaptive_noise_canceling,
                    TransparencyMode::take,
                    NoiseCancelingMode::take,
                    take_bool,
                    ManualTransparency::take,
                ),
                |(
                    ambient_sound_mode,
                    (manual_noise_canceling, adaptive_noise_canceling),
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise_reduction,
                    manual_transparency,
                )| Self {
                    ambient_sound_mode,
                    manual_noise_canceling,
                    adaptive_noise_canceling,
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise_reduction,
                    manual_transparency,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 6] {
        [
            self.ambient_sound_mode.id(),
            (self.manual_noise_canceling.inner() << 4) | self.adaptive_noise_canceling.inner(),
            self.transparency_mode as u8,
            self.noise_canceling_mode as u8,
            self.wind_noise_reduction.into(),
            self.manual_transparency.0,
        ]
    }
}

impl sound_modes_v2::ToPacketBody for SoundModes {
    fn bytes(&self) -> Vec<u8> {
        self.bytes().to_vec()
    }
}

#[repr(u8)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    FromRepr,
    EnumIter,
    Translate,
    EnumString,
    IntoStaticStr,
)]
pub enum TransparencyMode {
    #[default]
    TalkMode = 0,
    Manual = 1,
}

impl TransparencyMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |id| Self::from_repr(id).unwrap_or_default()).parse_complete(input)
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Default)]
pub struct ManualTransparency(pub u8);

impl ManualTransparency {
    pub fn new(byte: u8) -> Self {
        Self(byte.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, Self).parse_complete(input)
    }
}

#[repr(u8)]
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Default,
    FromRepr,
    EnumIter,
    Translate,
    EnumString,
    IntoStaticStr,
)]
pub enum NoiseCancelingMode {
    #[default]
    Manual = 0,
    Adaptive = 1,
}

impl NoiseCancelingMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |id| Self::from_repr(id).unwrap_or_default()).parse_complete(input)
    }
}

pub fn take_manual_and_adaptive_noise_canceling<
    'a,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
>(
    input: &'a [u8],
) -> IResult<&'a [u8], (ManualNoiseCanceling, AdaptiveNoiseCanceling), E> {
    map(le_u8, |b| {
        (
            ManualNoiseCanceling::new(b >> 4),
            AdaptiveNoiseCanceling::new(b & 0xF),
        )
    })
    .parse_complete(input)
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Default)]
pub struct ManualNoiseCanceling(u8);

impl ManualNoiseCanceling {
    pub fn new(byte: u8) -> Self {
        Self(byte.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Default)]
pub struct AdaptiveNoiseCanceling(u8);

impl AdaptiveNoiseCanceling {
    pub fn new(byte: u8) -> Self {
        Self(byte.clamp(0, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug, Default)]
pub struct ButtonConfiguration {
    pub double_press_action: Option<ButtonAction>,
}

impl ButtonConfiguration {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |single_press_action| Self {
            double_press_action: ButtonAction::from_repr(single_press_action),
        })
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 1] {
        [self.double_press_action.map_or(0xF, |action| action as u8)]
    }
}

#[repr(u8)]
#[derive(
    Clone,
    Copy,
    Hash,
    Eq,
    PartialEq,
    Debug,
    Default,
    FromRepr,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Translate,
)]
pub enum ButtonAction {
    #[default]
    BassUp = 7,
}
