use std::iter;

use crate::devices::soundcore::common::{
    packet,
    structures::{CommonEqualizerConfiguration, CustomHearId},
};

pub fn set_equalizer<
    const CHANNELS: usize,
    const BANDS: usize,
    const HEAR_ID_CHANNELS: usize,
    const HEAR_ID_BANDS: usize,
>(
    equalizer_configuration: &CommonEqualizerConfiguration<CHANNELS, BANDS>,
    hear_id: &CustomHearId<HEAR_ID_CHANNELS, HEAR_ID_BANDS>,
) -> packet::Outbound {
    let body = equalizer_configuration
        .preset_id()
        .to_le_bytes()
        .into_iter()
        .chain(hear_id.favorite_music_genre.bytes())
        .chain(
            equalizer_configuration
                .volume_adjustments_channel_1()
                .bytes(),
        )
        // This is the part that is different from the common set eq with hear id.
        // Rather than this being channel 2, it is dynamic range compression.
        .chain(
            equalizer_configuration
                .volume_adjustments_channel_1()
                .bytes(),
        )
        .chain([0, 1]) // gender, age range
        .chain(iter::once(hear_id.is_enabled.into()))
        .chain(hear_id.volume_adjustment_bytes())
        .chain(hear_id.time.to_be_bytes())
        .chain(iter::once(hear_id.hear_id_type as u8))
        .chain(
            hear_id
                .custom_volume_adjustment_bytes()
                .take(HEAR_ID_BANDS)
                // repeat the left side for the right side value rather than making use of the provided right side value
                .chain(hear_id.custom_volume_adjustment_bytes().take(HEAR_ID_BANDS)),
        )
        .chain(
            iter::repeat_n(
                equalizer_configuration
                    .volume_adjustments_channel_1()
                    .apply_drc()
                    .bytes(),
                2,
            )
            .flatten(),
        )
        .chain(iter::once(0)) // unknown
        .collect();
    packet::Outbound::new(packet::Command([0x03, 0x87]), body)
}
