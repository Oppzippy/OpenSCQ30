pub trait OutboundPacket {
    fn bytes(&self) -> Vec<i8>;
}
