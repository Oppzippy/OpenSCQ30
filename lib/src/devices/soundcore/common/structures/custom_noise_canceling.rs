use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "custom noise canceling",
            map(le_u8, |custom_noise_canceling_level| {
                Self::new(custom_noise_canceling_level)
            }),
        )
        .parse_complete(input)
    }
}
