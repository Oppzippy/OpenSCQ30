use std::sync::atomic::{self, AtomicI32};

#[derive(Debug, Default)]
pub struct TwoExtraEqBands {
    // The official app only displays 8 bands, so I have no idea what bands 9 and 10 do. We'll just keep track
    // of their initial value and resend that.
    extra_bands: AtomicI32,
}

impl TwoExtraEqBands {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_values(&self, extra_bands: TwoExtraEqBandsValues) {
        self.extra_bands
            .store(extra_bands.into(), atomic::Ordering::Relaxed);
    }

    pub fn values(&self) -> TwoExtraEqBandsValues {
        self.extra_bands.load(atomic::Ordering::Relaxed).into()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TwoExtraEqBandsValues {
    pub left_band_9: u8,
    pub left_band_10: u8,
    pub right_band_9: u8,
    pub right_band_10: u8,
}

impl TwoExtraEqBandsValues {
    pub fn left(&self) -> [u8; 2] {
        [self.left_band_9, self.left_band_10]
    }

    pub fn right(&self) -> [u8; 2] {
        [self.right_band_9, self.right_band_10]
    }
}

impl From<i32> for TwoExtraEqBandsValues {
    fn from(value: i32) -> Self {
        let bytes = value.to_ne_bytes();
        Self {
            left_band_9: bytes[0],
            left_band_10: bytes[1],
            right_band_9: bytes[2],
            right_band_10: bytes[3],
        }
    }
}

impl From<TwoExtraEqBandsValues> for i32 {
    fn from(value: TwoExtraEqBandsValues) -> Self {
        i32::from_ne_bytes([
            value.left_band_9,
            value.left_band_10,
            value.right_band_9,
            value.right_band_10,
        ])
    }
}
