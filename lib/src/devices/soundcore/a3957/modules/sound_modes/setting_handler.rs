use async_trait::async_trait;
use openscq30_i18n::Translate;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::soundcore::{
        a3957::structures::{AncPersonalizedToEarCanal, ManualNoiseCanceling, SoundModes},
        common::{
            self,
            settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        },
    },
    i18n::fl,
};

use super::SoundModeSetting;

#[derive(Default)]
pub struct SoundModesSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for SoundModesSettingHandler
where
    T: Has<SoundModes> + Has<AncPersonalizedToEarCanal> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SoundModeSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let sound_modes: &SoundModes = state.get();
        let sound_mode_setting: SoundModeSetting = (*setting_id).try_into().ok()?;
        match sound_mode_setting {
            SoundModeSetting::AmbientSoundMode => Some(Setting::select_from_enum_all_variants(
                sound_modes.ambient_sound_mode,
            )),
            SoundModeSetting::TransparencyMode => Some(Setting::select_from_enum_all_variants(
                sound_modes.transparency_mode,
            )),
            SoundModeSetting::NoiseCancelingMode => Some(Setting::select_from_enum_all_variants(
                sound_modes.noise_canceling_mode,
            )),
            SoundModeSetting::AdaptiveNoiseCanceling => Some(Setting::Information {
                value: <&'static str>::from(sound_modes.adaptive_noise_canceling).to_owned(),
                translated_value: sound_modes.adaptive_noise_canceling.translate(),
            }),
            SoundModeSetting::ManualNoiseCanceling => Some(Setting::I32Range {
                setting: settings::Range {
                    range: 1..=5,
                    step: 1,
                },
                value: sound_modes.manual_noise_canceling.inner().into(),
            }),
            SoundModeSetting::WindNoiseSuppression => Some(Setting::Toggle {
                value: sound_modes.wind_noise.is_suppression_enabled,
            }),
            SoundModeSetting::WindNoiseDetected => Some(Setting::Information {
                value: sound_modes.wind_noise.is_detected.to_string(),
                translated_value: if sound_modes.wind_noise.is_detected {
                    fl!("yes")
                } else {
                    fl!("no")
                },
            }),
            SoundModeSetting::MultiSceneNoiseCanceling => Some(Setting::select_from_enum(
                &[
                    common::structures::NoiseCancelingMode::Transport,
                    common::structures::NoiseCancelingMode::Outdoor,
                    common::structures::NoiseCancelingMode::Indoor,
                ],
                sound_modes.multi_scene_anc,
            )),
            SoundModeSetting::AncPersonalizedToEarCanal => {
                let anc_personalized_to_ear_canal: &AncPersonalizedToEarCanal = state.get();
                Some(Setting::Toggle {
                    value: anc_personalized_to_ear_canal.0,
                })
            }
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let sound_mode_setting: SoundModeSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match sound_mode_setting {
            SoundModeSetting::AmbientSoundMode => {
                let sound_modes: &mut SoundModes = state.get_mut();
                sound_modes.ambient_sound_mode = value.try_as_enum_variant()?;
            }
            SoundModeSetting::TransparencyMode => {
                let sound_modes: &mut SoundModes = state.get_mut();
                sound_modes.transparency_mode = value.try_as_enum_variant()?;
            }
            SoundModeSetting::NoiseCancelingMode => {
                let sound_modes: &mut SoundModes = state.get_mut();
                sound_modes.noise_canceling_mode = value.try_as_enum_variant()?;
            }
            SoundModeSetting::AdaptiveNoiseCanceling => return Err(SettingHandlerError::ReadOnly),
            SoundModeSetting::ManualNoiseCanceling => {
                let sound_modes: &mut SoundModes = state.get_mut();
                sound_modes.manual_noise_canceling =
                    ManualNoiseCanceling::new(value.try_as_i32()? as u8);
            }
            SoundModeSetting::WindNoiseSuppression => {
                let sound_modes: &mut SoundModes = state.get_mut();
                sound_modes.wind_noise.is_suppression_enabled = value.try_as_bool()?;
            }
            SoundModeSetting::WindNoiseDetected => return Err(SettingHandlerError::ReadOnly),
            SoundModeSetting::MultiSceneNoiseCanceling => {
                let sound_modes: &mut SoundModes = state.get_mut();
                sound_modes.multi_scene_anc = value.try_as_enum_variant()?;
            }
            SoundModeSetting::AncPersonalizedToEarCanal => {
                let anc_personalized_to_ear_canal: &mut AncPersonalizedToEarCanal = state.get_mut();
                *anc_personalized_to_ear_canal = AncPersonalizedToEarCanal(value.try_as_bool()?);
            }
        }
        Ok(())
    }
}
