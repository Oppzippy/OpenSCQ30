#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PacketType {
    SoundModeUpdate,
    SetSoundModeOk,
    SetEqualizerOk,
    StateUpdate,
}
