use nom::{
    combinator::{map, map_opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::pair,
    IResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::FromRepr;

use crate::devices::standard::packets::parsing::take_bool;

#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TwsStatus {
    pub is_connected: bool,
    pub host_device: HostDevice,
}

impl TwsStatus {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "tws status",
            map(
                pair(HostDevice::take, take_bool),
                |(host_device, is_connected)| TwsStatus {
                    is_connected,
                    host_device,
                },
            ),
        )(input)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromRepr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HostDevice {
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
