use std::iter;

use crate::devices::soundcore::{
    a3035,
    common::{
        packet,
        structures::{CommonEqualizerConfiguration, CustomHearId},
    },
};

pub fn set_button_double_press_action(
    maybe_action: Option<a3035::structures::ButtonAction>,
) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([0x04, 0x81]),
        vec![0, 0, maybe_action.map_or(0xF, |action| action as u8)],
    )
}

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
                .volume_adjustments()
                .iter()
                .flat_map(|v| v.bytes()),
        )
        .chain([0, 2]) // unknown
        .chain(iter::once(hear_id.is_enabled.into()))
        .chain(hear_id.volume_adjustment_bytes())
        .chain(hear_id.time.to_be_bytes())
        .chain(iter::once(hear_id.hear_id_type as u8))
        .chain(hear_id.custom_volume_adjustment_bytes())
        .chain(
            equalizer_configuration
                .volume_adjustments()
                .iter()
                .flat_map(|v| v.apply_drc().bytes()),
        )
        .chain(iter::once(0)) // unknown
        .collect();
    packet::Outbound::new(packet::Command([0x03, 0x87]), body)
}
