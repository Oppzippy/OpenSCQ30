use std::iter;

use crate::devices::soundcore::common::{
    packet,
    structures::{CustomHearId, EqualizerConfiguration},
};

pub fn set_equalizer<
    const CHANNELS: usize,
    const BANDS: usize,
    const HEAR_ID_CHANNELS: usize,
    const HEAR_ID_BANDS: usize,
>(
    equalizer_configuration: &EqualizerConfiguration<CHANNELS, BANDS>,
    hear_id: &CustomHearId<HEAR_ID_CHANNELS, HEAR_ID_BANDS>,
) -> packet::Outbound {
    let body = equalizer_configuration
        .profile_id()
        .to_le_bytes()
        .into_iter()
        .chain(hear_id.hear_id_preset_profile_id.to_le_bytes())
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
        .chain(hear_id.volume_adjustments.iter().flat_map(|v| v.bytes()))
        .chain(hear_id.time.to_le_bytes())
        .chain(iter::once(hear_id.hear_id_type.0))
        .chain(
            // repeat the left side for the right side value rather than making use of the provided right side value
            iter::repeat_n(
                hear_id.custom_volume_adjustments.expect(
                    "hear id custom volume adjustments should always be present for the a3040",
                )[0]
                .bytes(),
                2,
            )
            .flatten(),
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
