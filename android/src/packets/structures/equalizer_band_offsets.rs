use openscq30_lib::packets::structures;
use rifgen::rifgen_attr::generate_interface;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct EqualizerBandOffsets {
    inner: structures::EqualizerBandOffsets,
}

impl EqualizerBandOffsets {
    #[generate_interface(constructor)]
    pub fn new(volume_offsets: &[i8]) -> EqualizerBandOffsets {
        Self {
            inner: structures::EqualizerBandOffsets::new(volume_offsets.try_into().unwrap()),
        }
    }

    #[generate_interface]
    pub fn volume_offsets(&self) -> Vec<i8> {
        self.inner.volume_offsets().into()
    }
}

impl From<structures::EqualizerBandOffsets> for EqualizerBandOffsets {
    fn from(value: structures::EqualizerBandOffsets) -> Self {
        Self { inner: value }
    }
}

impl From<EqualizerBandOffsets> for structures::EqualizerBandOffsets {
    fn from(value: EqualizerBandOffsets) -> Self {
        value.inner
    }
}

#[cfg(test)]
mod tests {
    use crate::EqualizerBandOffsets;

    #[test]
    #[should_panic]
    fn new_panics_if_slice_length_is_less_than_8() {
        EqualizerBandOffsets::new(&vec![0; 7]);
    }

    #[test]
    #[should_panic]
    fn new_panics_if_slice_length_is_greater_than_8() {
        EqualizerBandOffsets::new(&vec![0; 9]);
    }

    #[test]
    fn new_does_not_panic_if_slice_length_is_8() {
        EqualizerBandOffsets::new(&vec![0; 8]);
        // It didn't panic, no assertions needed
    }
}
