#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CustomNoiseCanceling {
    value: u8,
}

impl CustomNoiseCanceling {
    pub fn new(value: u8) -> Self {
        // Not sure what 255 means here, but it is allowed in addition to 0-10
        let clamped_value = if value == 255 {
            value
        } else {
            value.clamp(0, 10)
        };
        Self {
            value: clamped_value,
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}
