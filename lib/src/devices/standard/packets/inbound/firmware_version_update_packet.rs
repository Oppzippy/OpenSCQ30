use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    sequence::tuple,
    IResult,
};

use crate::devices::standard::structures::{Command, FirmwareVersion, SerialNumber};

use super::InboundPacket;

// TODO think of a better name. this could be misleading since this does not update the firmware on the device,
// it simply updates our state with the version number of the firmware running on the device.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FirmwareVersionUpdatePacket {
    pub left_firmware_version: FirmwareVersion,
    pub right_firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
}

impl InboundPacket for FirmwareVersionUpdatePacket {
    fn command() -> Command {
        Command::new([0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x05])
    }

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], FirmwareVersionUpdatePacket, E> {
        context(
            "FirmwareVersionUpdatePacket",
            all_consuming(map(
                tuple((
                    FirmwareVersion::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                )),
                |(left_firmware_version, right_firmware_version, serial_number)| {
                    FirmwareVersionUpdatePacket {
                        left_firmware_version,
                        right_firmware_version,
                        serial_number,
                    }
                },
            )),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::{
        packets::inbound::{
            take_inbound_packet_header, FirmwareVersionUpdatePacket, InboundPacket,
        },
        structures::{FirmwareVersion, SerialNumber},
    };

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x05, 0x25, 0x00, 0x31, 0x32, 0x2e, 0x33, 0x34,
            0x32, 0x33, 0x2e, 0x34, 0x35, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
            0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0xca,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = FirmwareVersionUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;
        assert_eq!(FirmwareVersion::new(12, 34), packet.left_firmware_version);
        assert_eq!(FirmwareVersion::new(23, 45), packet.right_firmware_version);
        assert_eq!(
            SerialNumber("0123456789ABCDEF".into()),
            packet.serial_number
        );
    }
}
