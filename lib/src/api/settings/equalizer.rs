#[derive(Clone, Debug)]
pub struct Equalizer {
    pub band_hz: &'static [u16],
    pub fraction_digits: i16,
    pub min: i16,
    pub max: i16,
}
