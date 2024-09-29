use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::devices::standard::packets::parsing::ParseResult;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct CustomNoiseCanceling {
    value: u8,
}

impl CustomNoiseCanceling {
    pub fn new(value: u8) -> Self {
        // Not sure what 255 means here, but it is allowed in addition to 0-10
        let clamped_value = if value == 255 {
            value
        } else {
            value.clamp(0, 10)
        };
        Self {
            value: clamped_value,
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<CustomNoiseCanceling, E> {
        context(
            "custom noise canceling",
            map(le_u8, |custom_noise_canceling_level| {
                CustomNoiseCanceling::new(custom_noise_canceling_level)
            }),
        )(input)
    }
}
