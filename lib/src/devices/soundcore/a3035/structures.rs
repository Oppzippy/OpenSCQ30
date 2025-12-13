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
    self, modules::sound_modes_v2, packet::parsing::take_bool, structures::AmbientSoundMode,
};

common::structures::flag!(AmbientSoundModeVoicePrompt);
common::structures::flag!(BatteryAlert);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, MigrationSteps)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    #[migration_requirement(field = ambient_sound_mode, value = AmbientSoundMode::NoiseCanceling)]
    pub noise_canceling_mode: NoiseCancelingMode,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Adaptive)]
    pub adaptive_noise_canceling: AdaptiveNoiseCanceling,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Custom)]
    pub custom_noise_canceling: CustomNoiseCanceling,
    #[migration_requirement(field = ambient_sound_mode, value = AmbientSoundMode::Transparency)]
    pub custom_transparency: CustomTransparency,
    #[migration_requirement(field = ambient_sound_mode, value = AmbientSoundMode::NoiseCanceling)]
    pub wind_noise_reduction: WindNoiseReduction,
}

impl SoundModes {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3035 sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    take_manual_and_adaptive_noise_canceling,
                    le_u8, // unused
                    NoiseCancelingMode::take,
                    WindNoiseReduction::take,
                    CustomTransparency::take,
                ),
                |(
                    ambient_sound_mode,
                    (custom_noise_canceling, adaptive_noise_canceling),
                    _unused,
                    noise_canceling_mode,
                    wind_noise_reduction,
                    custom_transparency,
                )| Self {
                    ambient_sound_mode,
                    custom_noise_canceling,
                    adaptive_noise_canceling,
                    noise_canceling_mode,
                    wind_noise_reduction,
                    custom_transparency,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 6] {
        [
            self.ambient_sound_mode.id(),
            self.custom_noise_canceling.inner() << 4 | self.adaptive_noise_canceling.inner(),
            self.ambient_sound_mode.id(), // unused, repeats ambient sound mode
            self.noise_canceling_mode as u8,
            self.wind_noise_reduction.0.into(),
            self.custom_transparency.inner(),
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
    Default,
    Hash,
    FromRepr,
    EnumIter,
    Translate,
    EnumString,
    IntoStaticStr,
)]
pub enum NoiseCancelingMode {
    #[default]
    Custom = 0,
    Adaptive = 1,
}

impl NoiseCancelingMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |id| Self::from_repr(id).unwrap_or_default()).parse_complete(input)
    }
}

fn take_manual_and_adaptive_noise_canceling<
    'a,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
>(
    input: &'a [u8],
) -> IResult<&'a [u8], (CustomNoiseCanceling, AdaptiveNoiseCanceling), E> {
    map(le_u8, |b| {
        (
            CustomNoiseCanceling::new(b >> 4),
            AdaptiveNoiseCanceling::new(b & 0xF),
        )
    })
    .parse_complete(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct AdaptiveNoiseCanceling(u8);

impl AdaptiveNoiseCanceling {
    pub fn new(byte: u8) -> Self {
        Self(byte.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for AdaptiveNoiseCanceling {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy;

        (1u8..=5u8).prop_map(Self::new)
    }

    type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct CustomNoiseCanceling(u8);

impl CustomNoiseCanceling {
    pub fn new(byte: u8) -> Self {
        Self(byte.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for CustomNoiseCanceling {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy;

        (1u8..=5u8).prop_map(Self::new)
    }

    type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct CustomTransparency(u8);

impl CustomTransparency {
    pub fn new(byte: u8) -> Self {
        Self(byte.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, Self::new).parse_complete(input)
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for CustomTransparency {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy;

        (1u8..=5u8).prop_map(Self::new)
    }

    type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct WindNoiseReduction(pub bool);

impl WindNoiseReduction {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(take_bool, Self).parse_complete(input)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
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
