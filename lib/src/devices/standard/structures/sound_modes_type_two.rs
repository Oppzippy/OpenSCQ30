use super::{AmbientSoundMode, TransparencyMode};

use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
    sequence::tuple,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, FromRepr, IntoStaticStr};

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
    pub noise_canceling_adaptive_sensitivity_level: u8,
}

impl SoundModesTypeTwo {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], SoundModesTypeTwo, E> {
        context(
            "sound modes type two",
            map(
                tuple((
                    AmbientSoundMode::take,
                    NoiseCancelingSettings::take,
                    TransparencyMode::take,
                    NoiseCancelingModeTypeTwo::take,
                    WindNoise::take,
                    le_u8,
                )),
                |(
                    ambient_sound_mode,
                    noise_canceling_settings,
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise,
                    noise_canceling_adaptive_sensitivity_level,
                )| {
                    SoundModesTypeTwo {
                        ambient_sound_mode,
                        transparency_mode,
                        adaptive_noise_canceling: noise_canceling_settings.adaptive,
                        manual_noise_canceling: noise_canceling_settings.manual,
                        noise_canceling_mode,
                        wind_noise_suppression: wind_noise.is_suppression_enabled,
                        noise_canceling_adaptive_sensitivity_level,
                    }
                },
            ),
        )(input)
    }
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

impl AdaptiveNoiseCanceling {
    pub fn id(&self) -> u8 {
        *self as u8
    }
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

impl ManualNoiseCanceling {
    pub fn id(&self) -> u8 {
        *self as u8
    }
}

struct NoiseCancelingSettings {
    manual: ManualNoiseCanceling,
    adaptive: AdaptiveNoiseCanceling,
}

impl NoiseCancelingSettings {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], NoiseCancelingSettings, E> {
        map(le_u8, |b| NoiseCancelingSettings {
            manual: ManualNoiseCanceling::from_repr((b & 0xF0) >> 4).unwrap_or_default(),
            adaptive: AdaptiveNoiseCanceling::from_repr(b & 0x0F).unwrap_or_default(),
        })(input)
    }
}

#[repr(u8)]
#[derive(
    FromRepr, Clone, Copy, Debug, PartialEq, Eq, Hash, Display, Default, AsRefStr, IntoStaticStr,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum NoiseCancelingModeTypeTwo {
    #[default]
    Adaptive = 0,
    Manual = 1,
}

impl NoiseCancelingModeTypeTwo {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], NoiseCancelingModeTypeTwo, E> {
        context(
            "noise canceling mode type two",
            map(le_u8, |noise_canceling_mode| {
                NoiseCancelingModeTypeTwo::from_repr(noise_canceling_mode).unwrap_or_default()
            }),
        )(input)
    }
}

impl NoiseCancelingModeTypeTwo {
    pub fn id(&self) -> u8 {
        *self as u8
    }
}

pub struct WindNoise {
    pub is_suppression_enabled: bool,
    pub is_detected: bool,
}

impl WindNoise {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], WindNoise, E> {
        context(
            "wind noise",
            map(le_u8, |wind_noise| WindNoise {
                is_suppression_enabled: wind_noise & 1 != 0,
                is_detected: wind_noise & 2 != 0,
            }),
        )(input)
    }
}
