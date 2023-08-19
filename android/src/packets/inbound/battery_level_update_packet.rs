use openscq30_lib::packets::inbound::BatteryLevelUpdatePacket as LibBatteryLevelUpdatePacket;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatteryLevelUpdatePacket(LibBatteryLevelUpdatePacket);

impl BatteryLevelUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<BatteryLevelUpdatePacket, String> {
        Err("do not construct directly".to_string())
    }

    #[generate_interface]
    pub fn left_level(&self) -> u8 {
        self.0.left.0
    }

    #[generate_interface]
    pub fn right_level(&self) -> u8 {
        self.0.right.map(|level| level.0).unwrap_or_default()
    }
}

impl From<LibBatteryLevelUpdatePacket> for BatteryLevelUpdatePacket {
    fn from(packet: LibBatteryLevelUpdatePacket) -> Self {
        Self(packet)
    }
}
