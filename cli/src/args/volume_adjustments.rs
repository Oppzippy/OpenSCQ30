use std::ops::RangeInclusive;

use openscq30_lib::devices::standard::structures::VolumeAdjustments as LibVolumeAdjustments;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VolumeAdjustments(pub Vec<i16>);

impl VolumeAdjustments {
    pub fn range() -> RangeInclusive<i64> {
        let min = (LibVolumeAdjustments::MIN_VOLUME * 10.0).round() as i64;
        let max = (LibVolumeAdjustments::MAX_VOLUME * 10.0).round() as i64;
        min..=max
    }
}

impl From<VolumeAdjustments> for LibVolumeAdjustments {
    fn from(value: VolumeAdjustments) -> Self {
        Self::new(value.0.into_iter().map(|v| v as f64 / 10.0))
            .expect("CLI parser should verify correct number of bands")
    }
}
