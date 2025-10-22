use crate::devices::soundcore::common::packet;

pub trait IntoPacket {
    type DirectionMarker: packet::HasDirection;

    fn command(&self) -> packet::Command;
    fn body(&self) -> Vec<u8>;

    fn into_packet(&self) -> packet::Packet<Self::DirectionMarker> {
        packet::Packet::new(self.command(), self.body())
    }
}
