use openscq30_lib::packets::structures::CustomNoiseCanceling as LibCustomNoiseCanceling;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct CustomNoiseCanceling(LibCustomNoiseCanceling);

#[wasm_bindgen]
impl CustomNoiseCanceling {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u8) -> Self {
        Self(LibCustomNoiseCanceling::new(value))
    }

    #[wasm_bindgen]
    pub fn value(&self) -> u8 {
        self.0.value()
    }
}

impl From<CustomNoiseCanceling> for LibCustomNoiseCanceling {
    fn from(value: CustomNoiseCanceling) -> Self {
        value.0
    }
}

impl From<LibCustomNoiseCanceling> for CustomNoiseCanceling {
    fn from(value: LibCustomNoiseCanceling) -> Self {
        Self(value)
    }
}
