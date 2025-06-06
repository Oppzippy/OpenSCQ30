use crate::devices::soundcore::standard::{packets::Packet, structures::Command};

pub trait OutboundPacket {
    fn command(&self) -> Command;
    fn body(&self) -> Vec<u8>;
}

pub trait OutboundPacketBytesExt {
    fn bytes(self) -> Vec<u8>;
}

impl<T> OutboundPacketBytesExt for T
where
    T: OutboundPacket,
{
    fn bytes(self) -> Vec<u8> {
        Packet::from(self).bytes()
    }
}

impl<T> From<T> for Packet
where
    T: OutboundPacket,
{
    fn from(value: T) -> Self {
        Self {
            command: value.command(),
            body: value.body(),
        }
    }
}
