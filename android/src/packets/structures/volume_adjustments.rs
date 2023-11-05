use openscq30_lib::packets::structures;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct VolumeAdjustments {
    inner: structures::VolumeAdjustments,
}

impl VolumeAdjustments {
    #[generate_interface(constructor)]
    pub fn new(volume_adjustments: &[f64]) -> Result<VolumeAdjustments, String> {
        structures::VolumeAdjustments::new(volume_adjustments.iter().cloned())
            .map(|inner| Self { inner })
            .map_err(|err| err.to_string())
    }

    #[generate_interface]
    pub fn adjustments(&self) -> Vec<f64> {
        self.inner.adjustments().to_vec()
    }

    #[generate_interface]
    pub fn min_volume() -> f64 {
        structures::VolumeAdjustments::MIN_VOLUME
    }

    #[generate_interface]
    pub fn max_volume() -> f64 {
        structures::VolumeAdjustments::MAX_VOLUME
    }

    #[generate_interface]
    pub fn step() -> f64 {
        structures::VolumeAdjustments::STEP
    }
}

impl From<structures::VolumeAdjustments> for VolumeAdjustments {
    fn from(value: structures::VolumeAdjustments) -> Self {
        Self { inner: value }
    }
}

impl From<VolumeAdjustments> for structures::VolumeAdjustments {
    fn from(value: VolumeAdjustments) -> Self {
        value.inner
    }
}
