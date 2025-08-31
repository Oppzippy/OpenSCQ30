use nom::{
    IResult, Parser,
    combinator::{map, map_opt},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
    sequence::pair,
};
use openscq30_i18n_macros::Translate;
use strum::{Display, FromRepr};

use crate::devices::soundcore::common::packet::parsing::take_bool;

#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Hash, Default)]
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
                |(host_device, is_connected)| Self {
                    is_connected,
                    host_device,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 2] {
        [self.host_device as u8, self.is_connected as u8]
    }
}

#[repr(u8)]
#[derive(
    Default, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromRepr, Display, Translate,
)]
pub enum HostDevice {
    #[default]
    Left = 0,
    Right = 1,
}

impl HostDevice {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("host device", map_opt(le_u8, Self::from_repr)).parse_complete(input)
    }
}
