use openscq30_lib::packets::inbound::SetEqualizerWithDrcOkPacket as LibSetEqualizerWithDrcOkPacket;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetEqualizerWithDrcOkPacket(LibSetEqualizerWithDrcOkPacket);

impl SetEqualizerWithDrcOkPacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<SetEqualizerWithDrcOkPacket, String> {
        Err("do not construct directly".to_string())
    }
}

impl From<LibSetEqualizerWithDrcOkPacket> for SetEqualizerWithDrcOkPacket {
    fn from(packet: LibSetEqualizerWithDrcOkPacket) -> Self {
        Self(packet)
    }
}
