use crate::devices::standard::packets::parsing::{
    take_ambient_sound_mode, take_transparency_mode, ParseResult,
};

use super::{AmbientSoundMode, TransparencyMode};

use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, FromRepr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SoundModesTypeTwo {
    pub ambient_sound_mode: AmbientSoundMode,
    pub transparency_mode: TransparencyMode,
    pub adaptive_noise_canceling: AdaptiveNoiseCanceling,
    pub manual_noise_canceling: ManualNoiseCanceling,
    pub noise_canceling_mode: NoiseCancelingModeTypeTwo,
    pub wind_noise_suppression: bool,
    pub detected_wind_noise: bool,
    pub noise_canceling_adaptive_sensitivity_level: u8,
}

#[repr(u8)]
#[derive(FromRepr, Clone, Copy, Debug, PartialEq, Eq, Hash, Display, Default, AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum AdaptiveNoiseCanceling {
    #[default]
    LowNoise = 0,
    MediumNoise = 1,
    HighNoise = 2,
}

#[repr(u8)]
#[derive(FromRepr, Clone, Copy, Debug, PartialEq, Eq, Hash, Display, Default, AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ManualNoiseCanceling {
    #[default]
    Weak = 1,
    Moderate = 2,
    Strong = 3,
}

#[repr(u8)]
#[derive(FromRepr, Clone, Copy, Debug, PartialEq, Eq, Hash, Display, Default, AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum NoiseCancelingModeTypeTwo {
    #[default]
    Adaptive = 0,
    Manual = 1,
}

pub(crate) fn take_sound_modes_type_two<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<SoundModesTypeTwo, E> {
    context(
        "sound modes type two",
        map(
            tuple((
                take_ambient_sound_mode,
                take_manual_and_adaptive_noise_canceling,
                take_transparency_mode,
                take_noise_canceling_mode_type_two,
                take_wind_noise,
                le_u8,
            )),
            |(
                ambient_sound_mode,
                (manual_noise_canceling, adaptive_noise_canceling),
                transparency_mode,
                noise_canceling_mode,
                wind_noise,
                noise_canceling_adaptive_sensitivity_level,
            )| {
                SoundModesTypeTwo {
                    ambient_sound_mode,
                    transparency_mode,
                    adaptive_noise_canceling,
                    manual_noise_canceling,
                    noise_canceling_mode,
                    wind_noise_suppression: wind_noise.is_suppression_enabled,
                    detected_wind_noise: wind_noise.is_detected,
                    noise_canceling_adaptive_sensitivity_level,
                }
            },
        ),
    )(input)
}

pub(crate) fn take_manual_and_adaptive_noise_canceling<
    'a,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
>(
    input: &'a [u8],
) -> ParseResult<(ManualNoiseCanceling, AdaptiveNoiseCanceling), E> {
    map(le_u8, |b| {
        (
            ManualNoiseCanceling::from_repr((b & 0xF0) >> 4).unwrap_or_default(),
            AdaptiveNoiseCanceling::from_repr(b & 0x0F).unwrap_or_default(),
        )
    })(input)
}

pub(crate) fn take_noise_canceling_mode_type_two<
    'a,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
>(
    input: &'a [u8],
) -> ParseResult<NoiseCancelingModeTypeTwo, E> {
    context(
        "noise canceling mode type two",
        map(le_u8, |noise_canceling_mode| {
            NoiseCancelingModeTypeTwo::from_repr(noise_canceling_mode).unwrap_or_default()
        }),
    )(input)
}

pub struct WindNoise {
    pub is_suppression_enabled: bool,
    pub is_detected: bool,
}

pub(crate) fn take_wind_noise<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<WindNoise, E> {
    context(
        "wind noise",
        map(le_u8, |wind_noise| WindNoise {
            is_suppression_enabled: wind_noise & 1 != 0,
            is_detected: wind_noise & 2 != 0,
        }),
    )(input)
}
