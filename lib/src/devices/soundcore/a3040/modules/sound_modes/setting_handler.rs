use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::soundcore::{
        a3040::structures::{ManualNoiseCanceling, ManualTransparency, SoundModes},
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
};

use super::SoundModeSetting;

#[derive(Default)]
pub struct SoundModesSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for SoundModesSettingHandler
where
    T: Has<SoundModes> + Send,
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
            SoundModeSetting::TransparencyMode => {
                Setting::select_from_enum_all_variants(sound_modes.transparency_mode)
            }
            SoundModeSetting::NoiseCancelingMode => {
                Setting::select_from_enum_all_variants(sound_modes.noise_canceling_mode)
            }
            SoundModeSetting::ManualNoiseCanceling => Setting::I32Range {
                setting: settings::Range {
                    range: 1..=5,
                    step: 1,
                },
                value: sound_modes.manual_noise_canceling.inner().into(),
            },
            SoundModeSetting::WindNoiseSuppression => Setting::Toggle {
                value: sound_modes.wind_noise_reduction,
            },
            SoundModeSetting::ManualTransparency => Setting::I32Range {
                setting: settings::Range {
                    range: 1..=5,
                    step: 1,
                },
                value: sound_modes.manual_transparency.inner().into(),
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
            SoundModeSetting::TransparencyMode => {
                sound_modes.transparency_mode = value.try_as_enum_variant()?;
            }
            SoundModeSetting::NoiseCancelingMode => {
                sound_modes.noise_canceling_mode = value.try_as_enum_variant()?;
            }
            SoundModeSetting::ManualNoiseCanceling => {
                sound_modes.manual_noise_canceling =
                    ManualNoiseCanceling::new(value.try_as_i32()? as u8);
            }
            SoundModeSetting::WindNoiseSuppression => {
                sound_modes.wind_noise_reduction = value.try_as_bool()?;
            }
            SoundModeSetting::ManualTransparency => {
                sound_modes.manual_transparency =
                    ManualTransparency::new(value.try_as_i32()? as u8);
            }
        }
        Ok(())
    }
}
