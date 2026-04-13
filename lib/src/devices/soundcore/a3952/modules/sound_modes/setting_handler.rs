use async_trait::async_trait;
use openscq30_i18n::Translate;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3952,
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
    },
};

use super::SoundModeSetting;

#[derive(Default)]
pub struct SoundModesSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for SoundModesSettingHandler
where
    T: Has<a3952::structures::SoundModes> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SoundModeSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let sound_modes = state.get();
        let sound_mode_setting: SoundModeSetting = (*setting_id).try_into().ok()?;
        Some(match sound_mode_setting {
            SoundModeSetting::AmbientSoundMode => {
                Setting::select_from_enum_all_variants(sound_modes.ambient_sound_mode)
            }
            SoundModeSetting::NoiseCancelingMode => {
                Setting::select_from_enum_all_variants(sound_modes.noise_canceling_mode)
            }
            SoundModeSetting::AdaptiveNoiseCanceling => Setting::Information {
                value: sound_modes.adaptive_noise_canceling.to_string(),
                translated_value: sound_modes.adaptive_noise_canceling.translate(),
            },
            SoundModeSetting::ManualNoiseCanceling => {
                Setting::select_from_enum_all_variants(sound_modes.manual_noise_canceling)
            }
            SoundModeSetting::TransparencyMode => {
                Setting::select_from_enum_all_variants(sound_modes.transparency_mode)
            }
            SoundModeSetting::WindNoiseSuppression => Setting::Toggle {
                value: sound_modes.wind_noise.is_suppression_enabled,
            },
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let sound_modes = state.get_mut();
        let sound_mode_setting: SoundModeSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match sound_mode_setting {
            SoundModeSetting::AmbientSoundMode => {
                sound_modes.ambient_sound_mode = value.try_as_enum_variant()?;
            }
            SoundModeSetting::NoiseCancelingMode => {
                sound_modes.noise_canceling_mode = value.try_as_enum_variant()?;
            }
            SoundModeSetting::AdaptiveNoiseCanceling => {
                return Err(SettingHandlerError::ReadOnly);
            }
            SoundModeSetting::ManualNoiseCanceling => {
                sound_modes.manual_noise_canceling = value.try_as_enum_variant()?;
            }
            SoundModeSetting::TransparencyMode => {
                sound_modes.transparency_mode = value.try_as_enum_variant()?;
            }
            SoundModeSetting::WindNoiseSuppression => {
                sound_modes.wind_noise.is_suppression_enabled = value.try_as_bool()?;
            }
        }
        Ok(())
    }
}
