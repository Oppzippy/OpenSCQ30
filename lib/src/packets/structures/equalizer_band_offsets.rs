#[derive(Clone, Copy, Debug)]
pub struct EqualizerBandOffsets {
    volume_offsets: [i8; 8],
}

const MIN_VOLUME: i8 = -60;
const MAX_VOLUME: i8 = 60;

impl EqualizerBandOffsets {
    pub fn new(volume_offsets: [i8; 8]) -> Self {
        let clamped_offsets = volume_offsets.map(|vol| vol.clamp(MIN_VOLUME, MAX_VOLUME));
        Self {
            volume_offsets: clamped_offsets,
        }
    }

    pub fn bytes(&self) -> [u8; 8] {
        self.volume_offsets.map(Self::signed_offset_to_packet_byte)
    }

    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        Self::new(bytes.map(Self::packet_byte_to_signed_offset))
    }

    pub fn volume_offsets(&self) -> [i8; 8] {
        self.volume_offsets
    }

    fn signed_offset_to_packet_byte(offset: i8) -> u8 {
        // output should be in the 60-180 range
        let clamped = offset.clamp(MIN_VOLUME, MAX_VOLUME);
        let unsigned = (clamped + 60) as u8;
        unsigned + 60
    }

    fn packet_byte_to_signed_offset(byte: u8) -> i8 {
        let clamped = byte.clamp(
            Self::signed_offset_to_packet_byte(MIN_VOLUME),
            Self::signed_offset_to_packet_byte(MAX_VOLUME),
        );
        let signed = (clamped - 60) as i8;
        signed - 60
    }
}

#[cfg(test)]
mod tests {
    use super::EqualizerBandOffsets;
    const TEST_BYTES: [u8; 8] = [60, 80, 100, 120, 140, 160, 180, 120];
    const TEST_OFFSETS: [i8; 8] = [-60, -40, -20, 0, 20, 40, 60, 0];

    #[test]
    fn converts_volume_offsets_to_packet_bytes() {
        let band_offsets = EqualizerBandOffsets::new(TEST_OFFSETS);
        assert_eq!(TEST_BYTES, band_offsets.bytes());
    }

    #[test]
    fn from_bytes_converts_packet_bytes_to_offset() {
        let band_offsets = EqualizerBandOffsets::from_bytes(TEST_BYTES);
        assert_eq!(TEST_OFFSETS, band_offsets.volume_offsets());
    }

    #[test]
    fn it_clamps_bytes_outside_of_expected_range() {
        let band_offsets = EqualizerBandOffsets::from_bytes([0, 255, 120, 120, 120, 120, 120, 120]);
        assert_eq!(
            [60, 180, 120, 120, 120, 120, 120, 120],
            band_offsets.bytes()
        );
    }

    #[test]
    fn it_clamps_volume_offsets_outside_of_expected_range() {
        let band_offsets = EqualizerBandOffsets::new([-128, 127, 0, 0, 0, 0, 0, 0]);
        assert_eq!([-60, 60, 0, 0, 0, 0, 0, 0], band_offsets.volume_offsets());
    }
}
