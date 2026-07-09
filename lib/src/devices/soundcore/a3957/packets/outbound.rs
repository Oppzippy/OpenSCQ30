use itertools::Itertools;

use crate::devices::soundcore::common::{self, packet};

pub fn set_equalizer_configuration<
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>(
    equalizer_configuration: &common::structures::EqualizerConfiguration<
        CHANNELS,
        BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >,
    hear_id: &common::structures::CustomHearId<CHANNELS, BANDS>,
) -> packet::Outbound {
    // if hear id is favorite music genre, hear id initial should be modified in some unknown way
    // by favorite music genre. Since I don't know how that modification is performed, ignore it
    // for now. We don't support enabling hear id anyway, so it shouldn't matter.
    let active_volume_adjustments: Vec<u8> = if hear_id.is_enabled {
        if hear_id.hear_id_type == common::structures::HearIdType::Custom {
            // we could collect_array if we could do [u8; CHANNELS * BANDS], but unfortunately, that's not a thing
            hear_id.custom_volume_adjustment_bytes().collect()
        } else {
            hear_id.volume_adjustment_bytes().collect()
        }
    } else {
        equalizer_configuration
            .volume_adjustments()
            .iter()
            .flat_map(|v| v.bytes())
            .collect()
    };
    let active_volume_adjustments_len = active_volume_adjustments.len();
    let body = equalizer_configuration
        .preset_id()
        .to_le_bytes()
        .into_iter()
        .chain(hear_id.favorite_music_genre.bytes())
        .chain(
            equalizer_configuration
                .volume_adjustments()
                .iter()
                .flat_map(|v| v.bytes()),
        )
        .chain(std::iter::repeat_n(0, 2)) // unknown
        .chain(std::iter::once(u8::from(hear_id.is_enabled)))
        .chain(hear_id.volume_adjustment_bytes())
        .chain(hear_id.time.to_be_bytes())
        .chain(std::iter::once(hear_id.hear_id_type as u8))
        .chain(hear_id.custom_volume_adjustment_bytes())
        .chain(
            active_volume_adjustments
                .into_iter()
                .interleave(std::iter::repeat_n(0, active_volume_adjustments_len)),
        )
        .chain(std::iter::repeat_n(0, 2))
        .collect();
    packet::Outbound::new(packet::Command([3, 135]), body)
}
