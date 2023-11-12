use openscq30_lib::devices::standard::packets::inbound::ChineseVoicePromptStateUpdatePacket as LibChineseVoicePromptStateUpdatePacket;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChineseVoicePromptStateUpdatePacket(LibChineseVoicePromptStateUpdatePacket);

impl ChineseVoicePromptStateUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<ChineseVoicePromptStateUpdatePacket, String> {
        Err("do not construct directly".to_string())
    }

    #[generate_interface]
    pub fn is_enabled(&self) -> bool {
        self.0.is_enabled
    }
}

impl From<LibChineseVoicePromptStateUpdatePacket> for ChineseVoicePromptStateUpdatePacket {
    fn from(packet: LibChineseVoicePromptStateUpdatePacket) -> Self {
        Self(packet)
    }
}
