use openscq30_lib::packets::inbound::SetEqualizerOkPacket as LibSetEqualizerOkPacket;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetEqualizerOkPacket(LibSetEqualizerOkPacket);

impl SetEqualizerOkPacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<SetEqualizerOkPacket, String> {
        Err("do not construct directly".to_string())
    }
}

impl From<LibSetEqualizerOkPacket> for SetEqualizerOkPacket {
    fn from(packet: LibSetEqualizerOkPacket) -> Self {
        Self(packet)
    }
}
