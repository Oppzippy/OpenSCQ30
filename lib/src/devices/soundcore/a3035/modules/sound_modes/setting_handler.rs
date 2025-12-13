use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3035,
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
    },
    settings,
};

use super::SoundModeSetting;

#[derive(Default)]
pub struct SoundModesSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for SoundModesSettingHandler
where
    T: Has<a3035::structures::SoundModes> + Send,
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
                value: format!("{}/5", sound_modes.adaptive_noise_canceling.inner()),
                translated_value: format!("{}/5", sound_modes.adaptive_noise_canceling.inner()),
            },
            SoundModeSetting::ManualNoiseCanceling => Setting::I32Range {
                setting: settings::Range {
                    range: 1..=5,
                    step: 1,
                },
                value: sound_modes.custom_noise_canceling.inner().into(),
            },
            SoundModeSetting::WindNoiseSuppression => Setting::Toggle {
                value: sound_modes.wind_noise_reduction.0,
            },
            SoundModeSetting::ManualTransparency => Setting::I32Range {
                setting: settings::Range {
                    range: 1..=5,
                    step: 1,
                },
                value: sound_modes.custom_transparency.inner().into(),
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
                sound_modes.custom_noise_canceling = a3035::structures::CustomNoiseCanceling::new(
                    value.try_as_i32()?.clamp(u8::MIN as i32, u8::MAX as i32) as u8,
                );
            }
            SoundModeSetting::ManualTransparency => {
                sound_modes.custom_transparency = a3035::structures::CustomTransparency::new(
                    value.try_as_i32()?.clamp(u8::MIN as i32, u8::MAX as i32) as u8,
                );
            }
            SoundModeSetting::WindNoiseSuppression => {
                sound_modes.wind_noise_reduction.0 = value.try_as_bool()?;
            }
        }
        Ok(())
    }
}
