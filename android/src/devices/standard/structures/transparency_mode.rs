use openscq30_lib::devices::standard::structures::TransparencyMode as LibTransparencyMode;
use rifgen::rifgen_attr::generate_interface;

#[generate_interface]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TransparencyMode {
    FullyTransparent,
    VocalMode,
}

impl From<LibTransparencyMode> for TransparencyMode {
    fn from(value: LibTransparencyMode) -> Self {
        match value {
            LibTransparencyMode::FullyTransparent => Self::FullyTransparent,
            LibTransparencyMode::VocalMode => Self::VocalMode,
        }
    }
}

impl From<TransparencyMode> for LibTransparencyMode {
    fn from(value: TransparencyMode) -> Self {
        match value {
            TransparencyMode::FullyTransparent => Self::FullyTransparent,
            TransparencyMode::VocalMode => Self::VocalMode,
        }
    }
}
