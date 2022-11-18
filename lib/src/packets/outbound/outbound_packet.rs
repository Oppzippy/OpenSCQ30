pub trait OutboundPacket {
    fn bytes(&self) -> Vec<u8>;
}
