use crate::packets::structures::{
    equalizer_band_offsets::EqualizerBandOffsets, equalizer_configuration::EqualizerConfiguration,
    equalizer_profile_id::EqualizerProfileId,
};

use super::{outbound_packet::OutboundPacket, utils};

pub struct SetEqualizerPacket {
    configuration: EqualizerConfiguration,
}

impl SetEqualizerPacket {
    pub fn new(configuration: EqualizerConfiguration) -> Self {
        Self { configuration }
    }
}

impl OutboundPacket for SetEqualizerPacket {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0x08, 0xEE, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00];

        bytes.extend(self.configuration.profile_id().0);
        bytes.extend(self.configuration.band_offsets().bytes());

        bytes.push(utils::calculate_checksum(&bytes));

        bytes
    }
}
