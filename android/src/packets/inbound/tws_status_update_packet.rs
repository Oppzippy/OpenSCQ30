use openscq30_lib::packets::inbound::TwsStatusUpdatePacket as LibTwsStatusUpdatePacket;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TwsStatusUpdatePacket(LibTwsStatusUpdatePacket);

impl TwsStatusUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<TwsStatusUpdatePacket, String> {
        Err("do not construct directly".to_string())
    }

    #[generate_interface]
    pub fn host_device(&self) -> u8 {
        self.0.host_device
    }

    #[generate_interface]
    pub fn tws_status(&self) -> bool {
        self.0.tws_status
    }
}

impl From<LibTwsStatusUpdatePacket> for TwsStatusUpdatePacket {
    fn from(packet: LibTwsStatusUpdatePacket) -> Self {
        Self(packet)
    }
}
