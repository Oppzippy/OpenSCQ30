#[derive(Clone, Copy)]
pub struct EqualizerBandOffsets {
    volume_offsets: [i8; 8],
}

impl EqualizerBandOffsets {
    pub fn new(volume_offsets: [i8; 8]) -> Self {
        Self { volume_offsets }
    }

    pub fn bytes(&self) -> [u8; 8] {
        [
            Self::signed_offset_to_packet_byte(self.volume_offsets[0]),
            Self::signed_offset_to_packet_byte(self.volume_offsets[1]),
            Self::signed_offset_to_packet_byte(self.volume_offsets[2]),
            Self::signed_offset_to_packet_byte(self.volume_offsets[3]),
            Self::signed_offset_to_packet_byte(self.volume_offsets[4]),
            Self::signed_offset_to_packet_byte(self.volume_offsets[5]),
            Self::signed_offset_to_packet_byte(self.volume_offsets[6]),
            Self::signed_offset_to_packet_byte(self.volume_offsets[7]),
        ]
    }

    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        Self::new([
            Self::packet_byte_to_signed_offset(bytes[0]),
            Self::packet_byte_to_signed_offset(bytes[1]),
            Self::packet_byte_to_signed_offset(bytes[2]),
            Self::packet_byte_to_signed_offset(bytes[3]),
            Self::packet_byte_to_signed_offset(bytes[4]),
            Self::packet_byte_to_signed_offset(bytes[5]),
            Self::packet_byte_to_signed_offset(bytes[6]),
            Self::packet_byte_to_signed_offset(bytes[7]),
        ])
    }

    pub fn volume_offsets(&self) -> [i8; 8] {
        self.volume_offsets
    }

    fn signed_offset_to_packet_byte(offset: i8) -> u8 {
        // output should be in the 60-180 range
        let clamped = offset.clamp(-60, 60);
        let unsigned = (clamped + 60) as u8;
        unsigned + 60
    }

    fn packet_byte_to_signed_offset(byte: u8) -> i8 {
        let clamped = byte.clamp(60, 180);
        let signed = (clamped - 60) as i8;
        signed - 60
    }
}
