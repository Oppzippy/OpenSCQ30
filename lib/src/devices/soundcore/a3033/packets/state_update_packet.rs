use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a3033::state::A3033State,
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket},
        structures::{
            CommonEqualizerConfiguration, FirmwareVersion, SerialNumber, SingleBattery,
            WearingDetection,
        },
    },
};

// A3033 and A3033EU
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3033StateUpdatePacket {
    pub battery: SingleBattery,
    pub equalizer_configuration: CommonEqualizerConfiguration<1, 8>,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub wearing_detection: WearingDetection,
}

impl FromPacketBody for A3033StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3033 state update packet",
            map(
                (
                    SingleBattery::take,
                    CommonEqualizerConfiguration::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    WearingDetection::take,
                ),
                |(
                    battery,
                    equalizer_configuration,
                    firmware_version,
                    serial_number,
                    wearing_detection,
                )| {
                    Self {
                        battery,
                        equalizer_configuration,
                        firmware_version,
                        serial_number,
                        wearing_detection,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3033StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        [self.battery.is_charging as u8, self.battery.level.0]
            .into_iter()
            .chain(self.equalizer_configuration.bytes())
            .chain(self.firmware_version.to_string().into_bytes())
            .chain(self.serial_number.bytes())
            .chain(self.wearing_detection.bytes())
            .collect()
    }
}

state_update_packet_module!(A3033State, A3033StateUpdatePacket);

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::common::packet::inbound::TryToPacket;

    use super::*;

    #[test]
    fn serialize_and_deserialize() {
        let bytes = A3033StateUpdatePacket::default()
            .to_packet()
            .bytes_with_checksum();
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(&bytes).unwrap();
        let _: A3033StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
