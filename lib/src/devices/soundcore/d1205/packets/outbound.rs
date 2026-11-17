use std::iter;

use crate::devices::soundcore::{
    common::packet,
    d1205::structures::{EqualizerFilter, SpatialAudio},
};

pub const SET_EQUALIZER_COMMAND: packet::Command = packet::Command([0x03, 0x8a]);
pub const SET_SPATIAL_AUDIO_COMMAND: packet::Command = packet::Command([0x10, 0x81]);

/// Example for the custom profile:
/// ```text
/// fe fe              // preset id
/// 00 00 00 00 00 00  // unknown, always 0 in captures
/// 08                 // filter count
/// 01 cdcc8c40 0000c842 3333333f  // filter type, gain dB, frequency Hz, Q (little endian f32)
/// ...
/// ```
pub fn set_equalizer(preset_id: u16, filters: &[EqualizerFilter]) -> packet::Outbound {
    packet::Outbound::new(
        SET_EQUALIZER_COMMAND,
        preset_id
            .to_le_bytes()
            .into_iter()
            .chain(iter::repeat_n(0, 6))
            .chain(iter::once(
                u8::try_from(filters.len()).expect("there are at most 8 filters"),
            ))
            .chain(filters.iter().flat_map(EqualizerFilter::bytes))
            .collect(),
    )
}

pub fn set_spatial_audio(spatial_audio: &SpatialAudio) -> packet::Outbound {
    packet::Outbound::new(
        SET_SPATIAL_AUDIO_COMMAND,
        vec![u8::from(spatial_audio.is_enabled), 0, spatial_audio.mode],
    )
}
