use openscq30_lib::devices::standard::structures::{
    CustomHearId as LibCustomHearId, HearIdMusicType, HearIdType,
};
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

use crate::StereoVolumeAdjustments;

#[generate_interface_doc]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CustomHearId {
    inner: LibCustomHearId,
}

impl CustomHearId {
    #[generate_interface(constructor)]
    pub fn new(
        is_enabled: bool,
        volume_adjustments: StereoVolumeAdjustments,
        time: i32,
        hear_id_type: u8,
        hear_id_music_type: u8,
        custom_volume_adjustments: Option<StereoVolumeAdjustments>,
    ) -> CustomHearId {
        Self {
            inner: LibCustomHearId {
                is_enabled,
                volume_adjustments: volume_adjustments.into(),
                time,
                hear_id_type: HearIdType(hear_id_type),
                hear_id_music_type: HearIdMusicType(hear_id_music_type),
                custom_volume_adjustments: custom_volume_adjustments.map(Into::into),
            },
        }
    }
}

impl From<LibCustomHearId> for CustomHearId {
    fn from(inner: LibCustomHearId) -> Self {
        Self { inner }
    }
}
impl From<CustomHearId> for LibCustomHearId {
    fn from(value: CustomHearId) -> Self {
        value.inner
    }
}
