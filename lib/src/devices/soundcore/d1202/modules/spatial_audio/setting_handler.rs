use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        common::settings_manager::{SettingHandler, SettingHandlerResult},
        d1202::structures::SpatialAudio,
    },
};

use super::SpatialAudioSetting;

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
            SpatialAudioSetting::SpatialAudioMode => {
                Some(Setting::select_from_enum_all_variants(spatial_audio.mode))
            }
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
        match spatial_audio_setting {
            SpatialAudioSetting::SpatialAudio => {
                let spatial_audio = state.get_mut();
                spatial_audio.is_enabled = value.try_as_bool()?;
            }
            SpatialAudioSetting::SpatialAudioMode => {
                let spatial_audio = state.get_mut();
                spatial_audio.mode = value.try_as_enum_variant()?;
            }
        }
        Ok(())
    }
}
