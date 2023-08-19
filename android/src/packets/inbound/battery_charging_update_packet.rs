use openscq30_lib::packets::inbound::BatteryChargingUpdatePacket as LibBatteryChargingUpdatePacket;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatteryChargingUpdatePacket(LibBatteryChargingUpdatePacket);

impl BatteryChargingUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<BatteryChargingUpdatePacket, String> {
        Err("do not construct directly".to_string())
    }

    #[generate_interface]
    pub fn is_left_charging(&self) -> bool {
        self.0.left.into()
    }

    #[generate_interface]
    pub fn is_right_charging(&self) -> bool {
        self.0.right.map(Into::into).unwrap_or_default()
    }
}

impl From<LibBatteryChargingUpdatePacket> for BatteryChargingUpdatePacket {
    fn from(packet: LibBatteryChargingUpdatePacket) -> Self {
        Self(packet)
    }
}
