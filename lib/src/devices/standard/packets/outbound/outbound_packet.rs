use crate::soundcore_device::device::Packet;

pub trait OutboundPacket {
    fn command(&self) -> [u8; 7];
    fn body(&self) -> Vec<u8>;
}

pub trait SendableBytes {
    fn bytes(self) -> Vec<u8>;
}

impl<T> SendableBytes for T
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
