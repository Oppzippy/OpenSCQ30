use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3953::structures::SpatialAudio,
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
};

use super::SpatialAudioSetting;

// SpatialAudioMode maps to sound_mode (Music/Movie) and SpatialAudioMusicMode maps to effect_mode
// (Fixed/HeadTracking), matching the naming this project already uses for A3954's
// otherwise-unrelated (differently encoded) spatial audio feature.
#[derive(Default)]
pub struct SpatialAudioSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for SpatialAudioSettingHandler
where
    T: Has<SpatialAudio> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SpatialAudioSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let spatial_audio: &SpatialAudio = state.get();
        let spatial_audio_setting: SpatialAudioSetting = (*setting_id).try_into().ok()?;
        match spatial_audio_setting {
            SpatialAudioSetting::SpatialAudio => Some(Setting::Toggle {
                value: spatial_audio.is_enabled,
            }),
            SpatialAudioSetting::SpatialAudioMode => Some(Setting::select_from_enum_all_variants(
                spatial_audio.sound_mode,
            )),
            SpatialAudioSetting::SpatialAudioMusicMode => Some(
                Setting::select_from_enum_all_variants(spatial_audio.effect_mode),
            ),
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let spatial_audio_setting: SpatialAudioSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        let spatial_audio: &mut SpatialAudio = state.get_mut();
        match spatial_audio_setting {
            SpatialAudioSetting::SpatialAudio => {
                spatial_audio.is_enabled = value.try_as_bool()?;
            }
            SpatialAudioSetting::SpatialAudioMode => {
                spatial_audio.sound_mode = value.try_as_enum_variant()?;
            }
            SpatialAudioSetting::SpatialAudioMusicMode => {
                spatial_audio.effect_mode = value.try_as_enum_variant()?;
            }
        }
        Ok(())
    }
}
