use async_trait::async_trait;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Range, Setting, SettingId, Value},
    devices::soundcore::{
        a3936::structures::A3936SoundModes,
        standard::settings_manager::{SettingHandler, SettingHandlerResult},
    },
};

use super::SoundModeSetting;

#[derive(Default)]
pub struct SoundModesSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for SoundModesSettingHandler
where
    T: AsMut<A3936SoundModes> + AsRef<A3936SoundModes> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SoundModeSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let sound_modes = state.as_ref();
        let sound_mode_setting: SoundModeSetting = setting_id.try_into().ok()?;
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
            SoundModeSetting::AdaptiveNoiseCanceling => {
                Setting::select_from_enum_all_variants(sound_modes.adaptive_noise_canceling)
            }
            SoundModeSetting::ManualNoiseCanceling => {
                Setting::select_from_enum_all_variants(sound_modes.manual_noise_canceling)
            }
            SoundModeSetting::WindNoiseSuppression => Setting::Toggle {
                value: sound_modes.wind_noise_suppression,
            },
            SoundModeSetting::AdaptiveNoiseCancelingSensitivityLevel => Setting::I32Range {
                setting: Range {
                    range: 0..=10,
                    step: 1,
                },
                value: sound_modes
                    .noise_canceling_adaptive_sensitivity_level
                    .into(),
            },
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let sound_modes = state.as_mut();
        let sound_mode_setting: SoundModeSetting = setting_id
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match sound_mode_setting {
            SoundModeSetting::AmbientSoundMode => {
                sound_modes.ambient_sound_mode = value.try_as_enum_variant()?
            }
            SoundModeSetting::TransparencyMode => {
                sound_modes.transparency_mode = value.try_as_enum_variant()?
            }
            SoundModeSetting::NoiseCancelingMode => {
                sound_modes.noise_canceling_mode = value.try_as_enum_variant()?
            }
            SoundModeSetting::AdaptiveNoiseCanceling => {
                sound_modes.adaptive_noise_canceling = value.try_as_enum_variant()?
            }
            SoundModeSetting::ManualNoiseCanceling => {
                sound_modes.manual_noise_canceling = value.try_as_enum_variant()?
            }
            SoundModeSetting::WindNoiseSuppression => {
                sound_modes.wind_noise_suppression = value.try_as_bool()?
            }
            SoundModeSetting::AdaptiveNoiseCancelingSensitivityLevel => {
                sound_modes.noise_canceling_adaptive_sensitivity_level = value.try_as_i32()? as u8
            }
        }
        Ok(())
    }
}
