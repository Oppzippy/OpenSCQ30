use crate::devices::soundcore::common::packet;

pub trait ToPacket {
    type DirectionMarker: packet::HasDirection;

    fn command(&self) -> packet::Command;
    fn body(&self) -> Vec<u8>;

    fn to_packet(&self) -> packet::Packet<Self::DirectionMarker> {
        packet::Packet::new(self.command(), self.body())
    }
}
