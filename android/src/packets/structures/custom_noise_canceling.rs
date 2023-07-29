use openscq30_lib::packets::structures::CustomNoiseCanceling as LibCustomNoiseCanceling;
use rifgen::rifgen_attr::generate_interface;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct CustomNoiseCanceling(LibCustomNoiseCanceling);

impl CustomNoiseCanceling {
    #[generate_interface(constructor)]
    pub fn new(value: u8) -> CustomNoiseCanceling {
        Self(LibCustomNoiseCanceling::new(value))
    }

    #[generate_interface]
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
