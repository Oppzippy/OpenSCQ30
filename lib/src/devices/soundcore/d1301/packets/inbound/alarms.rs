use nom::Parser;

use crate::devices::soundcore::{
    common::packet::{self, inbound::FromPacketBody, outbound::ToPacket},
    d1301::structures::Alarm,
};

pub struct AlarmsPacket(pub Vec<Alarm>);

impl AlarmsPacket {
    pub const COMMAND: packet::Command = packet::Command([20, 1]);
}

impl FromPacketBody for AlarmsPacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self, E> {
        nom::multi::many0(Alarm::take)
            .map(Self)
            .parse_complete(input)
    }
}

impl ToPacket for AlarmsPacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> packet::Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.0.iter().flat_map(|alarm| alarm.bytes()).collect()
    }
}
