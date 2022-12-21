use super::{AmbientSoundModeUpdatePacket, OkPacket, StateUpdatePacket};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InboundPacket {
    StateUpdate(StateUpdatePacket),
    AmbientSoundModeUpdate(AmbientSoundModeUpdatePacket),
    Ok(OkPacket),
}

impl InboundPacket {
    pub fn new(bytes: &[u8]) -> Option<InboundPacket> {
        StateUpdatePacket::new(bytes)
            .map(|packet| InboundPacket::StateUpdate(packet))
            .or_else(|| {
                AmbientSoundModeUpdatePacket::new(bytes).map(InboundPacket::AmbientSoundModeUpdate)
            })
            .or_else(|| OkPacket::new(bytes).map(InboundPacket::Ok))
    }
}
