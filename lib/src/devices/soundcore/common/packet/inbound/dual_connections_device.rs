use macaddr::MacAddr6;
use nom::{
    IResult, Parser,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::common::{
    self,
    packet::{self, Command, outbound::ToPacket},
    structures::DualConnectionsDevice,
};

use super::FromPacketBody;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DualConnectionsDevicePacket {
    pub total_packets: u8,
    /// Index starts from 1
    pub current_packet_index: u8,
    pub devices: Vec<common::structures::DualConnectionsDevice>,
}

impl DualConnectionsDevicePacket {
    pub const COMMAND: Command = Command([0x0b, 0x01]);

    pub fn demo() -> Self {
        DualConnectionsDevicePacket {
            total_packets: 1,
            current_packet_index: 1,
            devices: vec![
                DualConnectionsDevice {
                    is_connected: true,
                    mac_address: MacAddr6::new(0, 0, 0, 0, 0, 0),
                    name: "Device 1".to_string(),
                },
                DualConnectionsDevice {
                    is_connected: false,
                    mac_address: MacAddr6::new(0, 0, 0, 0, 0, 1),
                    name: "Device 2".to_string(),
                },
                DualConnectionsDevice {
                    is_connected: false,
                    mac_address: MacAddr6::new(0, 0, 0, 0, 0, 2),
                    name: "Device 3".to_string(),
                },
            ],
        }
    }
}

impl ToPacket for DualConnectionsDevicePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> packet::Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        [self.total_packets, self.current_packet_index]
            .into_iter()
            .chain(self.devices.iter().flat_map(|device| device.bytes()))
            .collect()
    }
}

impl FromPacketBody for DualConnectionsDevicePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("dual connection device", |input: &'a [u8]| {
            let (mut input, (total_packets, current_packet_index)) =
                (le_u8, le_u8).parse_complete(input)?;

            let mut devices = Vec::new();
            while !input.is_empty() {
                let (remaining, device) = DualConnectionsDevice::take(input)?;
                devices.push(device);
                input = remaining;
            }

            Ok((
                &[] as &[u8], // name extends all the way to the end of the packet, so this empties out input
                Self {
                    total_packets,
                    current_packet_index,
                    devices,
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
        assert_eq!(parsed.devices.len(), 1);
        assert_eq!(parsed.devices[0].name, "Pixel 6aAAAAAAAAAAAAAAAAAAAAAAAA");
    }

    #[test]
    fn multiple_devices_in_one_packet() {
        let initial = [
            0x1, 0x1, // current/total
            0x28, 0x1, 0x66, 0x14, 0x92, 0x2c, 0x3a, 0xd4, 0x50, 0x69, 0x78, 0x65, 0x6c, 0x20,
            0x36, 0x61, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // device 1
            0x28, 0x1, 0x67, 0x14, 0x92, 0x2c, 0x3a, 0xd4, 0x50, 0x69, 0x78, 0x65, 0x6c, 0x20,
            0x37, 0x61, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // device 2
        ];
        let (remaining, parsed) =
            DualConnectionsDevicePacket::take::<VerboseError<_>>(&initial).unwrap();
        assert_eq!(remaining.len(), 0);
        assert_eq!(parsed.devices[0].name, "Pixel 6a");
        assert_eq!(parsed.devices[1].name, "Pixel 7a");
    }
}
