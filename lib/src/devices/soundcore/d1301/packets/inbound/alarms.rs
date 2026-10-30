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
        let (input, alarms) = nom::multi::many0(Alarm::take)
            .map(Self)
            .parse_complete(input)?;

        let alarm_ids = alarms.0.iter().map(|alarm| alarm.id);
        if !alarms.0.iter().map(|alarm| alarm.id).is_sorted() {
            tracing::warn!(
                "Alarms are not sorted! Please open an issue if you see this, and say what you did that triggered this message. Got alarm id order: {alarm_ids:?}"
            );
        } else if alarms
            .0
            .iter()
            .enumerate()
            .any(|(index, alarm)| usize::from(alarm.id) != index)
        {
            tracing::warn!(
                "Alarm ids do not always match their index! Please open an issue if you see this. Got alarm ids: {alarm_ids:?}"
            );
        }
        Ok((input, alarms))
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
