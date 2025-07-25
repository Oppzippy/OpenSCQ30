use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::standard::{
    packet::{Command, outbound::OutboundPacket},
    structures::{DualFirmwareVersion, SerialNumber},
};

use super::InboundPacket;

// TODO think of a better name. this could be misleading since this does not update the firmware on the device,
// it simply updates our state with the version number of the firmware running on the device.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SerialNumberAndFirmwareVersionUpdatePacket {
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
}

impl SerialNumberAndFirmwareVersionUpdatePacket {
    pub const COMMAND: Command = Command([0x01, 0x05]);
}

impl InboundPacket for SerialNumberAndFirmwareVersionUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "FirmwareVersionUpdatePacket",
            all_consuming(map(
                (DualFirmwareVersion::take, SerialNumber::take),
                |(dual_firmware_version, serial_number)| Self {
                    dual_firmware_version,
                    serial_number,
                },
            )),
        )
        .parse_complete(input)
    }
}

impl OutboundPacket for SerialNumberAndFirmwareVersionUpdatePacket {
    fn command(&self) -> Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.dual_firmware_version
            .bytes()
            .chain(self.serial_number.0.to_string().into_bytes())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::standard::{
        packet::{
            Packet,
            inbound::{InboundPacket, SerialNumberAndFirmwareVersionUpdatePacket},
        },
        structures::{FirmwareVersion, SerialNumber},
    };

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x05, 0x24, 0x00, 0x31, 0x32, 0x2e, 0x33, 0x34,
            0x32, 0x33, 0x2e, 0x34, 0x35, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
            0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0xc9,
        ];
        let (_, packet) = Packet::take::<VerboseError<_>>(input).unwrap();
        let packet =
            SerialNumberAndFirmwareVersionUpdatePacket::take::<VerboseError<_>>(&packet.body)
                .unwrap()
                .1;
        assert_eq!(
            FirmwareVersion::new(12, 34),
            packet.dual_firmware_version.left
        );
        assert_eq!(
            FirmwareVersion::new(23, 45),
            packet.dual_firmware_version.right
        );
        assert_eq!(
            SerialNumber("0123456789ABCDEF".into()),
            packet.serial_number
        );
    }
}
