use itertools::Itertools;

use crate::devices::soundcore::{
    a3954,
    common::{self, packet},
};

pub fn set_case_features(case_features: &a3954::structures::CaseFeatures) -> packet::Outbound {
    packet::Outbound::new(packet::Command([7, 135]), case_features.bytes().collect())
}

pub fn set_case_language(case_language: &a3954::structures::CaseLanguage) -> packet::Outbound {
    packet::Outbound::new(packet::Command([7, 138]), case_language.bytes().collect())
}

pub fn set_easy_chat(easy_chat: &a3954::structures::EasyChat) -> packet::Outbound {
    packet::Outbound::new(packet::Command([16, 157]), easy_chat.bytes().collect())
}

pub fn set_spatial_audio(spatial_audio: &a3954::structures::SpatialAudio) -> packet::Outbound {
    packet::Outbound::new(packet::Command([16, 129]), spatial_audio.bytes().collect())
}

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
    /*
    Example:
    254, 254, // preset id
    0, 0, // hear id favorite music genre
    120, 120, 120, 120, 120, 120, 120, 120, 120, 120, // eq left
    120, 120, 120, 120, 120, 120, 120, 120, 120, 120, // eq right
    0,   // unknown
    0,   // unknown
    1,   // is hear id enabled
    145, 147, 139, 141, 122, 111, 105, 100, 60, 60, // hear id eq left
    145, 147, 139, 141, 122, 111, 105, 100, 60, 60, // hear id eq right
    0, 0, 0, 0, // hear id timestamp
    1, // hear id type
    145, 147, 139, 130, 122, 133, 175, 114, 60, 0, // hear id custom eq left
    145, 147, 139, 130, 122, 133, 175, 114, 60, 0, // hear id custom eq right
    145, 0, 147, 0, 139, 0, 130, 0, 122, 0, 133, 0, 175, 0, 114, 0, 60, 0, 0, 0, // effective eq left with interleaved 0s
    145, 0, 147, 0, 139, 0, 130, 0, 122, 0, 133, 0, 175, 0, 114, 0, 60, 0, 0, 0, // effective eq right with interleaved 0s
    0, // unknown
    0, // unknown
    */

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
