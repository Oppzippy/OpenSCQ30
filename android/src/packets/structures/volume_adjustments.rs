use openscq30_lib::packets::structures;
use rifgen::rifgen_attr::generate_interface;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct VolumeAdjustments {
    inner: structures::VolumeAdjustments,
}

impl VolumeAdjustments {
    #[generate_interface(constructor)]
    pub fn new(volume_adjustments: &[i8]) -> VolumeAdjustments {
        Self {
            inner: structures::VolumeAdjustments::new(volume_adjustments.try_into().unwrap()),
        }
    }

    #[generate_interface]
    pub fn adjustments(&self) -> Vec<i8> {
        self.inner.adjustments().into()
    }

    #[generate_interface]
    pub fn min_volume() -> i8 {
        structures::VolumeAdjustments::MIN_VOLUME
    }

    #[generate_interface]
    pub fn max_volume() -> i8 {
        structures::VolumeAdjustments::MAX_VOLUME
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

#[cfg(test)]
mod tests {
    use crate::VolumeAdjustments;

    #[test]
    #[should_panic]
    fn new_panics_if_slice_length_is_less_than_8() {
        VolumeAdjustments::new(&vec![0; 7]);
    }

    #[test]
    #[should_panic]
    fn new_panics_if_slice_length_is_greater_than_8() {
        VolumeAdjustments::new(&vec![0; 9]);
    }

    #[test]
    fn new_does_not_panic_if_slice_length_is_8() {
        VolumeAdjustments::new(&vec![0; 8]);
        // It didn't panic, no assertions needed
    }
}
