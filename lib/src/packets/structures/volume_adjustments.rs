#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct VolumeAdjustments {
    volume_adjustments: [i8; 8],
}

impl VolumeAdjustments {
    pub const MIN_VOLUME: i8 = -120;
    pub const MAX_VOLUME: i8 = 120;

    pub fn new(volume_adjustments: [i8; 8]) -> Self {
        let clamped_adjustments =
            volume_adjustments.map(|vol| vol.clamp(Self::MIN_VOLUME, Self::MAX_VOLUME));
        Self {
            volume_adjustments: clamped_adjustments,
        }
    }

    pub fn bytes(&self) -> [u8; 8] {
        self.volume_adjustments
            .map(Self::signed_adjustment_to_packet_byte)
    }

    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        Self::new(bytes.map(Self::packet_byte_to_signed_adjustment))
    }

    pub fn adjustments(&self) -> [i8; 8] {
        self.volume_adjustments
    }

    fn signed_adjustment_to_packet_byte(offset: i8) -> u8 {
        let clamped = offset.clamp(Self::MIN_VOLUME, Self::MAX_VOLUME);
        
        clamped.wrapping_add(Self::MIN_VOLUME.abs()) as u8
    }

    fn packet_byte_to_signed_adjustment(byte: u8) -> i8 {
        let clamped = byte.clamp(
            Self::signed_adjustment_to_packet_byte(Self::MIN_VOLUME),
            Self::signed_adjustment_to_packet_byte(Self::MAX_VOLUME),
        );
        
        clamped.wrapping_sub(Self::MIN_VOLUME.unsigned_abs()) as i8
    }
}

#[cfg(test)]
mod tests {
    use super::VolumeAdjustments;
    const TEST_BYTES: [u8; 8] = [0, 80, 100, 120, 140, 160, 180, 240];
    const TEST_ADJUSTMENTS: [i8; 8] = [-120, -40, -20, 0, 20, 40, 60, 120];

    #[test]
    fn converts_volume_adjustments_to_packet_bytes() {
        let band_adjustments = VolumeAdjustments::new(TEST_ADJUSTMENTS);
        assert_eq!(TEST_BYTES, band_adjustments.bytes());
    }

    #[test]
    fn from_bytes_converts_packet_bytes_to_adjustment() {
        let band_adjustments = VolumeAdjustments::from_bytes(TEST_BYTES);
        assert_eq!(TEST_ADJUSTMENTS, band_adjustments.adjustments());
    }

    #[test]
    fn it_clamps_bytes_outside_of_expected_range() {
        let band_adjustments =
            VolumeAdjustments::from_bytes([0, 255, 120, 120, 120, 120, 120, 120]);
        assert_eq!(
            [0, 240, 120, 120, 120, 120, 120, 120],
            band_adjustments.bytes()
        );
    }

    #[test]
    fn it_clamps_volume_adjustments_outside_of_expected_range() {
        let band_adjustments = VolumeAdjustments::new([-128, 127, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            [-120, 120, 0, 0, 0, 0, 0, 0],
            band_adjustments.adjustments()
        );
    }
}
