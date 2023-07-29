use super::{HearIdMusicType, HearIdType, StereoVolumeAdjustments};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct CustomHearId {
    pub is_enabled: bool,
    pub volume_adjustments: StereoVolumeAdjustments,
    pub time: i32,
    pub hear_id_type: HearIdType,
    pub hear_id_music_type: HearIdMusicType,
    pub custom_volume_adjustments: Option<StereoVolumeAdjustments>,
}
