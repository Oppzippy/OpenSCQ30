use nom::{
    combinator::map_opt,
    error::{context, ContextError, ParseError},
    IResult,
};
use std::{fmt::Display, sync::Arc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::devices::standard::packets::parsing::take_str;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SerialNumber(pub Arc<str>);

impl SerialNumber {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn model_number(&self) -> &str {
        &self.0[0..4]
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], SerialNumber, E> {
        context(
            "serial number",
            map_opt(take_str(16usize), |s| {
                // Serial number is 4 digit model number followed by mac address with pairs in reverse order.
                // ie. device model 1234 with max address 55:66:77:88:99:AA would be 1234AA9988776655
                // The mac address is hex, and the model number is base 10 digits only, so we can use that
                // to try to avoid parsing things that aren't serial numbers as one.
                if s.chars().all(|c| c.is_ascii_hexdigit()) {
                    Some(SerialNumber::from(s))
                } else {
                    None
                }
            }),
        )(input)
    }
}

impl Default for SerialNumber {
    fn default() -> Self {
        Self("0000000000000000".into())
    }
}

impl From<&str> for SerialNumber {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl Display for SerialNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
