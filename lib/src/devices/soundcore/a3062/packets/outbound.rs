use std::iter;

use crate::devices::soundcore::{
    a3062,
    common::{
        packet,
        structures::{CommonEqualizerConfiguration, CustomHearId, OptionalVolumeAdjustmentsExt},
    },
};

pub fn set_button_double_press_action(
    maybe_action: Option<a3062::structures::ButtonAction>,
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
    // this should check if hear id is enabled, but that's not really necessary since set_equalizer
    // will never be called with hear id enabled.
    let active_volume_adjustments = equalizer_configuration.volume_adjustments();

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
        .chain([0, 0]) // unknown
        .chain(iter::once(hear_id.is_enabled.into()))
        .chain(hear_id.volume_adjustment_bytes())
        .chain(hear_id.time.to_be_bytes())
        .chain(iter::once(hear_id.hear_id_type as u8))
        .chain(hear_id.custom_volume_adjustment_bytes())
        .chain(active_volume_adjustments.iter().flat_map(|v| v.bytes()))
        .chain(iter::once(0)) // unknown
        .collect();
    packet::Outbound::new(packet::Command([0x03, 0x87]), body)
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::structures::{
        HearIdMusicGenre, HearIdType, VolumeAdjustments,
    };

    use super::*;

    #[test]
    fn set_equalizer_matches_known_good_packet() {
        let packet = set_equalizer(
            &CommonEqualizerConfiguration::new_all_bands_present(
                0xfefe,
                [VolumeAdjustments::from_bytes([
                    60, 125, 60, 148, 113, 95, 144, 111, 120, 120,
                ])],
            ),
            &CustomHearId {
                is_enabled: false,
                volume_adjustments: [Some(VolumeAdjustments::from_bytes([
                    120, 120, 120, 120, 120, 120, 120, 120, 120, 60,
                ]))],
                time: 0,
                hear_id_type: HearIdType::FavoriteMusicGenre,
                favorite_music_genre: HearIdMusicGenre(0),
                custom_volume_adjustments: [Some(VolumeAdjustments::from_bytes([
                    130, 130, 130, 130, 130, 130, 130, 130, 130, 60,
                ]))],
            },
        );

        assert_eq!(
            packet,
            packet::Outbound::new(
                packet::Command([3, 135]),
                vec![
                    254, 254, // preset
                    0, 0, // hear id favorite music genre
                    60, 125, 60, 148, 113, 95, 144, 111, 120, 120, // volume adjustments
                    0, 0, // unknkown
                    0, // hear id enabled
                    120, 120, 120, 120, 120, 120, 120, 120, 120,
                    60, // hear id volume adjustments
                    0, 0, 0, 0, // hear id time
                    2, // hear id type
                    130, 130, 130, 130, 130, 130, 130, 130, 130,
                    60, // hear id custom volume adjustments
                    60, 125, 60, 148, 113, 95, 144, 111, 120,
                    120, // active volume adjustments
                    0,   // unknown
                ]
            )
        )
    }
}
