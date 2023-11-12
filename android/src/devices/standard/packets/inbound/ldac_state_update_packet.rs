use openscq30_lib::devices::standard::packets::inbound::LdacStateUpdatePacket as LibLdacStateUpdatePacket;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LdacStateUpdatePacket(LibLdacStateUpdatePacket);

impl LdacStateUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<LdacStateUpdatePacket, String> {
        Err("do not construct directly".to_string())
    }

    #[generate_interface]
    pub fn is_enabled(&self) -> bool {
        self.0.is_enabled
    }
}

impl From<LibLdacStateUpdatePacket> for LdacStateUpdatePacket {
    fn from(packet: LibLdacStateUpdatePacket) -> Self {
        Self(packet)
    }
}
