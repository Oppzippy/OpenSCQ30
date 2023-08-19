use openscq30_lib::packets::inbound::SetSoundModeOkPacket as LibSetSoundModeOkPacket;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetSoundModeOkPacket(LibSetSoundModeOkPacket);

impl SetSoundModeOkPacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<SetSoundModeOkPacket, String> {
        Err("do not construct directly".to_string())
    }
}

impl From<LibSetSoundModeOkPacket> for SetSoundModeOkPacket {
    fn from(packet: LibSetSoundModeOkPacket) -> Self {
        Self(packet)
    }
}
