use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use strum::{Display, EnumIter, EnumString, FromRepr, IntoStaticStr, VariantArray};

pub fn take_manual_adaptive_noise_canceling<'a, ManualT, AdaptiveT, E>(
    input: &'a [u8],
) -> IResult<&'a [u8], (ManualT, AdaptiveT), E>
where
    ManualT: ManualAdaptiveNoiseCancelingFromByte + ManualNoiseCancelingMarker,
    AdaptiveT: ManualAdaptiveNoiseCancelingFromByte + AdaptiveNoiseCancelingMarker,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
{
    map(le_u8, |b| {
        (
            ManualT::from_byte((b & 0xF0) >> 4),
            AdaptiveT::from_byte(b & 0x0F),
        )
    })
    .parse_complete(input)
}

pub fn manual_adaptive_noise_canceling_byte<ManualT, AdaptiveT>(
    manual: ManualT,
    adaptive: AdaptiveT,
) -> u8
where
    ManualT: ManualAdaptiveNoiseCancelingAsByte + ManualNoiseCancelingMarker,
    AdaptiveT: ManualAdaptiveNoiseCancelingAsByte + AdaptiveNoiseCancelingMarker,
{
    (manual.byte() << 4) | (adaptive.byte() as u8)
}

pub trait ManualAdaptiveNoiseCancelingAsByte {
    fn byte(self) -> u8;
}

pub trait ManualAdaptiveNoiseCancelingFromByte {
    fn from_byte(byte: u8) -> Self;
}

pub trait ManualNoiseCancelingMarker {}
pub trait AdaptiveNoiseCancelingMarker {}

#[repr(u8)]
#[derive(
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Display,
    Default,
    IntoStaticStr,
    EnumString,
    EnumIter,
    Translate,
)]
pub enum ManualNoiseCancelingNamed {
    #[default]
    Weak = 1,
    Moderate = 2,
    Strong = 3,
}

impl ManualNoiseCancelingMarker for ManualNoiseCancelingNamed {}
impl ManualAdaptiveNoiseCancelingAsByte for ManualNoiseCancelingNamed {
    fn byte(self) -> u8 {
        self as u8
    }
}
impl ManualAdaptiveNoiseCancelingFromByte for ManualNoiseCancelingNamed {
    fn from_byte(byte: u8) -> Self {
        Self::from_repr(byte).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ManualNoiseCanceling(u8);

impl Default for ManualNoiseCanceling {
    fn default() -> Self {
        Self::new(1)
    }
}

impl ManualNoiseCanceling {
    pub fn new(value: u8) -> Self {
        Self(value.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

impl ManualNoiseCancelingMarker for ManualNoiseCanceling {}
impl ManualAdaptiveNoiseCancelingAsByte for ManualNoiseCanceling {
    fn byte(self) -> u8 {
        self.0
    }
}
impl ManualAdaptiveNoiseCancelingFromByte for ManualNoiseCanceling {
    fn from_byte(byte: u8) -> Self {
        Self::new(byte)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    FromRepr,
    Display,
    IntoStaticStr,
    EnumString,
    EnumIter,
    VariantArray,
    Translate,
)]
#[repr(u8)]
pub enum AdaptiveNoiseCancelingNamedStrength {
    #[default]
    Weak = 0,
    Moderate = 1,
    Strong = 2,
}

impl AdaptiveNoiseCancelingMarker for AdaptiveNoiseCancelingNamedStrength {}
impl ManualAdaptiveNoiseCancelingAsByte for AdaptiveNoiseCancelingNamedStrength {
    fn byte(self) -> u8 {
        self as u8
    }
}
impl ManualAdaptiveNoiseCancelingFromByte for AdaptiveNoiseCancelingNamedStrength {
    fn from_byte(byte: u8) -> Self {
        Self::from_repr(byte).unwrap_or_default()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    FromRepr,
    Display,
    IntoStaticStr,
    EnumString,
    EnumIter,
    VariantArray,
    Translate,
)]
#[repr(u8)]
pub enum AdaptiveNoiseCancelingNamedNoiseLevel {
    #[default]
    LowNoise = 0,
    MediumNoise = 1,
    HighNoise = 2,
}

impl AdaptiveNoiseCancelingMarker for AdaptiveNoiseCancelingNamedNoiseLevel {}
impl ManualAdaptiveNoiseCancelingAsByte for AdaptiveNoiseCancelingNamedNoiseLevel {
    fn byte(self) -> u8 {
        self as u8
    }
}
impl ManualAdaptiveNoiseCancelingFromByte for AdaptiveNoiseCancelingNamedNoiseLevel {
    fn from_byte(byte: u8) -> Self {
        Self::from_repr(byte).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct AdaptiveNoiseCanceling(u8);

impl Default for AdaptiveNoiseCanceling {
    fn default() -> Self {
        Self::new(1)
    }
}

impl AdaptiveNoiseCanceling {
    pub fn new(value: u8) -> Self {
        Self(value.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

impl AdaptiveNoiseCancelingMarker for AdaptiveNoiseCanceling {}
impl ManualAdaptiveNoiseCancelingAsByte for AdaptiveNoiseCanceling {
    fn byte(self) -> u8 {
        self.0
    }
}
impl ManualAdaptiveNoiseCancelingFromByte for AdaptiveNoiseCanceling {
    fn from_byte(byte: u8) -> Self {
        Self::new(byte)
    }
}
