use openscq30_lib::packets::inbound::FirmwareVersionUpdatePacket as LibFirmwareVersionUpdatePacket;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

use crate::FirmwareVersion;

#[generate_interface_doc]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FirmwareVersionUpdatePacket(LibFirmwareVersionUpdatePacket);

impl FirmwareVersionUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<FirmwareVersionUpdatePacket, String> {
        Err("do not construct directly".to_string())
    }

    #[generate_interface]
    pub fn firmware_version(&self) -> FirmwareVersion {
        self.0
            .left_firmware_version
            .min(self.0.right_firmware_version)
            .into()
    }

    #[generate_interface]
    pub fn serial_number(&self) -> String {
        self.0.serial_number.to_string()
    }
}

impl From<LibFirmwareVersionUpdatePacket> for FirmwareVersionUpdatePacket {
    fn from(packet: LibFirmwareVersionUpdatePacket) -> Self {
        Self(packet)
    }
}
