use macaddr::MacAddr6;
use nom::{
    IResult, Parser,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::common::{
    self,
    packet::{self, Command, outbound::ToPacket, parsing::take_bool},
};

use super::FromPacketBody;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DualConnectionsDevicePacket {
    pub total_devices: u8,
    pub index: u8,
    pub device: common::structures::DualConnectionsDevice,
}

impl DualConnectionsDevicePacket {
    pub const COMMAND: Command = Command([0x0b, 0x01]);
}

impl ToPacket for DualConnectionsDevicePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> packet::Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        [
            self.total_devices,
            self.index,
            0x28,
            self.device.is_connected as u8,
        ]
        .into_iter()
        .chain(self.device.mac_address.into_array())
        .chain(self.device.name.as_bytes().iter().take(32).cloned())
        .chain(std::iter::repeat_n(
            0,
            32usize.saturating_sub(self.device.name.len()),
        ))
        .collect()
    }
}

impl FromPacketBody for DualConnectionsDevicePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("dual connection device", |input| {
            let (input, (total_devices, index, _remaining_length, is_connected, mac_address_bytes)) =
                (
                    le_u8,
                    le_u8,
                    le_u8, // number of bytes from here (including this byte) to the end. name is remaining_length-8.
                    take_bool,
                    (le_u8, le_u8, le_u8, le_u8, le_u8, le_u8),
                )
                    .parse_complete(input)?;
            let mac_address = MacAddr6::from(<[u8; 6]>::from(mac_address_bytes));
            let name_bytes = input
                .iter()
                .position(|b| *b == 0)
                .map_or(input, |index| &input[..index]);
            let name = str::from_utf8(name_bytes)
                .map_err(|_| {
                    nom::Err::Error(E::from_error_kind(input, nom::error::ErrorKind::Char))
                })?
                .to_owned();
            Ok((
                &[] as &[u8], // name extends all the way to the end of the packet, so this empties out input
                Self {
                    total_devices,
                    index,
                    device: common::structures::DualConnectionsDevice {
                        is_connected,
                        mac_address,
                        name,
                    },
                },
            ))
        })
        .parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use super::*;

    #[test]
    fn round_trip() {
        let initial = [
            0x6, 0x1, 0x28, 0x1, 0x66, 0x14, 0x92, 0x2c, 0x3a, 0xd4, 0x50, 0x69, 0x78, 0x65, 0x6c,
            0x20, 0x36, 0x61, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ];
        let (remaining, parsed) =
            DualConnectionsDevicePacket::take::<VerboseError<_>>(&initial).unwrap();
        assert_eq!(remaining.len(), 0);
        assert_eq!(parsed.to_packet().body, initial);
    }

    #[test]
    fn no_string_null_termination() {
        let initial = [
            0x6, 0x1, 0x28, 0x1, 0x66, 0x14, 0x92, 0x2c, 0x3a, 0xd4, 0x50, 0x69, 0x78, 0x65, 0x6c,
            0x20, 0x36, 0x61, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
            65, 65, 65, 65, 65, 65, 65,
        ];
        let (remaining, parsed) =
            DualConnectionsDevicePacket::take::<VerboseError<_>>(&initial).unwrap();
        assert_eq!(remaining.len(), 0);
        assert_eq!(parsed.device.name, "Pixel 6aAAAAAAAAAAAAAAAAAAAAAAAA");
    }
}
