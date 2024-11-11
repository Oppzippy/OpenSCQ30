use nom::{
    combinator::map_opt,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::FromRepr;

use crate::devices::standard::packets::parsing::ParseResult;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromRepr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HostDevice {
    NotApplicable = 0,
    Left = 1,
    Right = 2,
}

impl HostDevice {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<HostDevice, E> {
        context("host device", map_opt(le_u8, |i| HostDevice::from_repr(i)))(input)
    }
}
