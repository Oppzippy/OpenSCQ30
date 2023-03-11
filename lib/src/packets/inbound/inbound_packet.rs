use super::{
    AmbientSoundModeUpdatePacket, SetAmbientModeOkPacket, SetEqualizerOkPacket, StateUpdatePacket,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InboundPacket {
    StateUpdate(StateUpdatePacket),
    AmbientSoundModeUpdate(AmbientSoundModeUpdatePacket),
    SetAmbientModeOk(SetAmbientModeOkPacket),
    SetEqualizerOk(SetEqualizerOkPacket),
}

impl InboundPacket {
    pub fn new(bytes: &[u8]) -> Option<InboundPacket> {
        StateUpdatePacket::new(bytes)
            .map(InboundPacket::StateUpdate)
            .or_else(|| {
                AmbientSoundModeUpdatePacket::new(bytes).map(InboundPacket::AmbientSoundModeUpdate)
            })
            .or_else(|| SetAmbientModeOkPacket::new(bytes).map(InboundPacket::SetAmbientModeOk))
            .or_else(|| SetEqualizerOkPacket::new(bytes).map(InboundPacket::SetEqualizerOk))
    }
}

#[cfg(test)]
mod tests {
    use super::InboundPacket;

    #[test]
    fn it_returns_none_when_nothing_matches() {
        let packet = InboundPacket::new(&vec![1, 2, 3]);
        assert_eq!(None, packet);
    }
}
