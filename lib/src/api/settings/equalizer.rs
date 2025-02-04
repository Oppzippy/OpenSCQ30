#[derive(Clone, Debug)]
pub struct Equalizer {
    pub num_bands: u8,
    pub decimal_places: u8,
    pub min: i16,
    pub max: i16,
}
