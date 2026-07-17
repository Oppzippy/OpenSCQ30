use crate::devices::soundcore::{
    a3955::structures::{AncPersonalizedToEarCanal, ImmersiveExperience},
    common::{self, packet, structures::OptionalVolumeAdjustmentsExt},
};

pub fn set_anc_personalized_to_hear_canal(
    anc_personalized_to_ear_canal: &AncPersonalizedToEarCanal,
) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([3, 144]),
        anc_personalized_to_ear_canal.bytes().to_vec(),
    )
}

pub fn set_immersive_experience(immersive_experience: ImmersiveExperience) -> packet::Outbound {
    packet::Outbound::new(packet::Command([18, 129]), vec![immersive_experience as u8])
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
    is_hear_id_initialized: bool,
) -> packet::Outbound {
    let body = equalizer_configuration
        .preset_id()
        .to_le_bytes()
        .into_iter()
        .chain(hear_id.favorite_music_genre.bytes())
        .chain(equalizer_configuration.volume_adjustments_bytes())
        .chain(if is_hear_id_initialized {
            [0; 2]
        } else {
            [255; 2]
        })
        .chain(std::iter::once(u8::from(hear_id.is_enabled)))
        .chain(hear_id.volume_adjustments.iter().flat_map(|v| {
            let mut bytes = v.bytes();
            if v.is_none() {
                bytes[bytes.len() - 1] = 0;
            }
            bytes
        }))
        .chain(hear_id.time.to_be_bytes())
        .chain(std::iter::once(hear_id.hear_id_type as u8))
        .chain(hear_id.custom_volume_adjustments.iter().flat_map(|v| {
            let mut bytes = v.bytes();
            if v.is_none() {
                bytes[bytes.len() - 1] = 0;
            }
            bytes
        }))
        .chain(
            equalizer_configuration
                .volume_adjustments()
                .iter()
                .flat_map(|v| v.apply_drc().bytes()),
        )
        .chain(std::iter::once(0))
        .collect();

    packet::Outbound::new(packet::Command([3, 135]), body)
}
