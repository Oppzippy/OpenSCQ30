use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
};
use std::sync::atomic::{self, AtomicI32};

use crate::devices::standard::{
    packets::parsing::{take_equalizer_configuration, take_volume_adjustments, ParseResult},
    structures::{EqualizerConfiguration, StereoVolumeAdjustments},
};

#[derive(Debug, Default)]
pub struct TwoExtraEqBands {
    // The official app only displays 8 bands, so I have no idea what bands 9 and 10 do. We'll just keep track
    // of their initial value and resend that.
    extra_bands: AtomicI32,
}

impl TwoExtraEqBands {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_values(&self, extra_bands: TwoExtraEqBandsValues) {
        self.extra_bands
            .store(extra_bands.into(), atomic::Ordering::Relaxed);
    }

    pub fn values(&self) -> TwoExtraEqBandsValues {
        self.extra_bands.load(atomic::Ordering::Relaxed).into()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TwoExtraEqBandsValues {
    pub left_extra_1: u8,
    pub left_extra_2: u8,
    pub right_extra_1: u8,
    pub right_extra_2: u8,
}

impl TwoExtraEqBandsValues {
    pub fn left(&self) -> [u8; 2] {
        [self.left_extra_1, self.left_extra_2]
    }

    pub fn right(&self) -> [u8; 2] {
        [self.right_extra_1, self.right_extra_2]
    }
}

impl From<i32> for TwoExtraEqBandsValues {
    fn from(value: i32) -> Self {
        let bytes = value.to_ne_bytes();
        Self {
            left_extra_1: bytes[0],
            left_extra_2: bytes[1],
            right_extra_1: bytes[2],
            right_extra_2: bytes[3],
        }
    }
}

impl From<TwoExtraEqBandsValues> for i32 {
    fn from(value: TwoExtraEqBandsValues) -> Self {
        i32::from_ne_bytes([
            value.left_extra_1,
            value.left_extra_2,
            value.right_extra_1,
            value.right_extra_2,
        ])
    }
}

pub fn take_stereo_equalizer_configuration_with_two_extra_bands<
    'a,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
>(
    num_bands: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<(EqualizerConfiguration, TwoExtraEqBandsValues), E> {
    move |input| {
        context(
            "stereo volume adjustments",
            map(
                tuple((
                    take_equalizer_configuration(num_bands),
                    le_u8,
                    le_u8,
                    take_volume_adjustments(num_bands),
                    le_u8,
                    le_u8,
                )),
                |(
                    left_equalizer_configuration,
                    left_extra_1,
                    left_extra_2,
                    right,
                    right_extra_1,
                    right_extra_2,
                )| {
                    (
                        left_equalizer_configuration,
                        TwoExtraEqBandsValues {
                            left_extra_1,
                            left_extra_2,
                            right_extra_1,
                            right_extra_2,
                        },
                    )
                },
            ),
        )(input)
    }
}
