use std::iter;

use crate::devices::soundcore::{
    common::{
        packet,
        structures::{
            AgeRange, CommonEqualizerConfiguration, CommonVolumeAdjustments, CustomHearId,
            OptionalVolumeAdjustmentsExt,
        },
    },
    d1402::structures::{ButtonAction, EqualizerUnknownByte},
};

pub fn set_button_double_press_action(maybe_action: Option<ButtonAction>) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([0x04, 0x81]),
        vec![0, 0, maybe_action.map_or(0xF, |action| action as u8)],
    )
}

/// When there is no hear id, the device stores each of its volume adjustment arrays as 0xFF for
/// every band except the last, which is 0. Writing back 0xFF for the last band as well (as the
/// common `CustomHearId` does) makes the last band's gain the maximum even with hear id disabled,
/// which sounds extremely loud and distorted.
pub fn hear_id_volume_adjustment_bytes<const BANDS: usize>(
    volume_adjustments: &[Option<CommonVolumeAdjustments<BANDS>>],
) -> impl Iterator<Item = u8> {
    volume_adjustments.iter().flat_map(|maybe_adjustments| {
        maybe_adjustments.map_or_else(
            || {
                let mut bytes = [0xFF; BANDS];
                if let Some(last) = bytes.last_mut() {
                    *last = 0;
                }
                bytes
            },
            |adjustments| adjustments.bytes(),
        )
    })
}

/// Same as `CustomHearId::bytes_with_music_genre_at_end`, but with the device's encoding of an
/// absent hear id.
pub fn hear_id_bytes_with_music_genre_at_end<const CHANNELS: usize, const BANDS: usize>(
    hear_id: &CustomHearId<CHANNELS, BANDS>,
) -> impl Iterator<Item = u8> {
    iter::once(u8::from(hear_id.is_enabled))
        .chain(hear_id_volume_adjustment_bytes(&hear_id.volume_adjustments))
        .chain(hear_id.time.to_be_bytes())
        .chain(iter::once(hear_id.hear_id_type as u8))
        .chain(hear_id_volume_adjustment_bytes(
            &hear_id.custom_volume_adjustments,
        ))
        .chain(hear_id.favorite_music_genre.bytes())
}

/// Same layout as the Space One (A3035)'s, including the DRC bands and the encoding of an absent
/// hear id, except that the two unknown bytes after the volume adjustments are the ones from the
/// state update packet (unknown byte and age range) rather than being derived from whether hear id
/// is enabled.
pub fn set_equalizer<
    const CHANNELS: usize,
    const BANDS: usize,
    const HEAR_ID_CHANNELS: usize,
    const HEAR_ID_BANDS: usize,
>(
    equalizer_configuration: &CommonEqualizerConfiguration<CHANNELS, BANDS>,
    hear_id: &CustomHearId<HEAR_ID_CHANNELS, HEAR_ID_BANDS>,
    equalizer_unknown_byte: EqualizerUnknownByte,
    age_range: AgeRange,
) -> packet::Outbound {
    let active_volume_adjustments = equalizer_configuration.volume_adjustments();

    let body = equalizer_configuration
        .preset_id()
        .to_le_bytes()
        .into_iter()
        .chain(hear_id.favorite_music_genre.bytes())
        .chain(active_volume_adjustments.iter().flat_map(|v| v.bytes()))
        .chain([equalizer_unknown_byte.0, age_range.0])
        .chain(iter::once(hear_id.is_enabled.into()))
        .chain(hear_id_volume_adjustment_bytes(&hear_id.volume_adjustments))
        .chain(hear_id.time.to_be_bytes())
        .chain(iter::once(hear_id.hear_id_type as u8))
        .chain(hear_id_volume_adjustment_bytes(
            &hear_id.custom_volume_adjustments,
        ))
        .chain(
            active_volume_adjustments
                .iter()
                .flat_map(|v| v.apply_drc().bytes()),
        )
        .chain(iter::once(0)) // unknown
        .collect();
    packet::Outbound::new(packet::Command([0x03, 0x87]), body)
}
