use macaddr::MacAddr6;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::common::packet::parsing::take_bool;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DualConnections {
    pub is_enabled: bool,
    pub devices: Vec<DualConnectionsDevice>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DualConnectionsDevice {
    pub is_connected: bool,
    pub mac_address: MacAddr6,
    pub name: String,
}

impl DualConnectionsDevice {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("dual connection device", |input| {
            let (input, (length, is_connected, mac_address_bytes)) = (
                le_u8, // number of bytes from here (including this byte) to the end. name is remaining_length-8.
                take_bool,
                (le_u8, le_u8, le_u8, le_u8, le_u8, le_u8),
            )
                .parse_complete(input)?;
            let name_length = length - 8;
            // Name, right padded with 0s
            let (input, name_bytes) = take(usize::from(name_length)).parse_complete(input)?;

            let mac_address = MacAddr6::from(<[u8; 6]>::from(mac_address_bytes));

            // Name without 0 padding
            let name_str_bytes = name_bytes
                .iter()
                .position(|b| *b == 0)
                .map_or(name_bytes, |index| &name_bytes[..index]);
            // Bluetooth device names are supposed to be utf8
            let name = str::from_utf8(name_str_bytes)
                .map_err(|_| {
                    nom::Err::Error(E::from_error_kind(input, nom::error::ErrorKind::Char))
                })?
                .to_owned();

            Ok((
                input,
                Self {
                    is_connected,
                    mac_address,
                    name,
                },
            ))
        })
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [0x28, self.is_connected as u8]
            .into_iter()
            .chain(self.mac_address.into_array())
            .chain(self.name.as_bytes().iter().take(32).copied())
            .chain(std::iter::repeat_n(
                0,
                32usize.saturating_sub(self.name.len()),
            ))
    }
}
