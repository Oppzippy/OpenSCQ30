use nom::{Parser, combinator::map};

use crate::devices::soundcore::{
    common::packet::{self, inbound::FromPacketBody, outbound::ToPacket},
    d1301::structures::AutoStopTimer,
};

pub struct AutoStopTimerPacket(pub AutoStopTimer);

impl AutoStopTimerPacket {
    pub const COMMAND: packet::Command = packet::Command([21, 3]);
}

impl FromPacketBody for AutoStopTimerPacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> nom::IResult<&'a [u8], Self, E> {
        map(AutoStopTimer::take, Self).parse_complete(input)
    }
}

impl ToPacket for AutoStopTimerPacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> packet::Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.0.bytes().collect()
    }
}
