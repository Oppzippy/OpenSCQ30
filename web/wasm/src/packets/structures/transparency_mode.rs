use openscq30_lib::packets::structures::TransparencyMode as LibTransparencyMode;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
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
