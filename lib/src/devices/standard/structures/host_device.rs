use nom::{
    combinator::map_opt,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    IResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::FromRepr;

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
    ) -> IResult<&'a [u8], HostDevice, E> {
        context("host device", map_opt(le_u8, HostDevice::from_repr))(input)
    }
}
