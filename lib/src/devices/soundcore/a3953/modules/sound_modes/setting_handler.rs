use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3953::structures::SoundModes,
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
};

use super::SoundModesSetting;

#[derive(Default)]
pub struct SoundModesSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for SoundModesSettingHandler
where
    T: Has<SoundModes> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SoundModesSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let sound_modes: &SoundModes = state.get();
        let sound_mode_setting: SoundModesSetting = (*setting_id).try_into().ok()?;
        match sound_mode_setting {
            SoundModesSetting::AmbientSoundMode => Some(Setting::select_from_enum_all_variants(
                sound_modes.ambient_sound_mode,
            )),
            SoundModesSetting::WindNoiseSuppression => Some(Setting::Toggle {
                value: sound_modes.wind_noise.is_suppression_enabled,
            }),
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let sound_mode_setting: SoundModesSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match sound_mode_setting {
            SoundModesSetting::AmbientSoundMode => {
                let sound_modes: &mut SoundModes = state.get_mut();
                sound_modes.ambient_sound_mode = value.try_as_enum_variant()?;
            }
            SoundModesSetting::WindNoiseSuppression => {
                let sound_modes: &mut SoundModes = state.get_mut();
                sound_modes.wind_noise.is_suppression_enabled = value.try_as_bool()?;
            }
        }
        Ok(())
    }
}
