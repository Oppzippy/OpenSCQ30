use super::PacketType;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct PacketHeader {
    pub packet_type: PacketType,
    pub length: u16,
}
