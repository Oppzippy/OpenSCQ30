use openscq30_lib::packets::structures;
use rifgen::rifgen_attr::generate_interface;

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
