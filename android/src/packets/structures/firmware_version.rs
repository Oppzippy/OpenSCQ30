use openscq30_lib::packets::structures::FirmwareVersion as LibFirmwareVersion;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct FirmwareVersion(LibFirmwareVersion);

impl FirmwareVersion {
    #[generate_interface(constructor)]
    pub fn new(major: u8, minor: u8) -> FirmwareVersion {
        Self(LibFirmwareVersion::new(major, minor))
    }

    #[generate_interface]
    pub fn major(&self) -> u8 {
        self.0.major()
    }

    #[generate_interface]
    pub fn minor(&self) -> u8 {
        self.0.minor()
    }

    #[generate_interface]
    pub fn number(&self) -> u16 {
        self.0.number()
    }

    #[generate_interface]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<LibFirmwareVersion> for FirmwareVersion {
    fn from(value: LibFirmwareVersion) -> Self {
        Self(value)
    }
}

impl From<FirmwareVersion> for LibFirmwareVersion {
    fn from(value: FirmwareVersion) -> Self {
        value.0
    }
}
