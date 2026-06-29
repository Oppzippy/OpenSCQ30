use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::soundcore::{
        a3954::{self, structures::SoundModes},
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
    },
    i18n::fl,
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
            SoundModesSetting::ManualNoiseCanceling => (1..=5)
                .contains(&sound_modes.sound_mode_slider)
                .then(|| Setting::I32Range {
                    setting: settings::Range {
                        range: 1..=5,
                        step: 1,
                    },
                    // Strength is the slider's distance from the center, so flip it around for <6
                    value: i32::from(6 - sound_modes.sound_mode_slider),
                }),
            SoundModesSetting::ManualTransparency => (7..=11)
                .contains(&sound_modes.sound_mode_slider)
                .then(|| Setting::I32Range {
                    setting: settings::Range {
                        range: 1..=5,
                        step: 1,
                    },
                    value: i32::from(sound_modes.sound_mode_slider - 6),
                }),
            SoundModesSetting::AirplaneMode => Some(Setting::select_from_enum_all_variants(
                sound_modes.airplane_mode,
            )),
            SoundModesSetting::WindNoiseSuppression => Some(Setting::Toggle {
                value: sound_modes.wind_noise.is_suppression_enabled,
            }),
            SoundModesSetting::WindNoiseDetected => Some(Setting::Information {
                value: sound_modes.wind_noise.is_detected.to_string(),
                translated_value: if sound_modes.wind_noise.is_detected {
                    fl!("yes")
                } else {
                    fl!("no")
                },
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
                let ambient_sound_mode = value.try_as_enum_variant()?;
                if sound_modes.ambient_sound_mode != ambient_sound_mode {
                    sound_modes.ambient_sound_mode = ambient_sound_mode;
                    match sound_modes.ambient_sound_mode {
                        a3954::structures::AmbientSoundMode::NoiseCanceling => {
                            sound_modes.sound_mode_slider = 5
                        }
                        a3954::structures::AmbientSoundMode::Normal => {
                            sound_modes.sound_mode_slider = 6
                        }
                        a3954::structures::AmbientSoundMode::Transparency => {
                            sound_modes.sound_mode_slider = 7
                        }
                        a3954::structures::AmbientSoundMode::AirplaneMode => (),
                    }
                }
            }
            SoundModesSetting::ManualNoiseCanceling => {
                let sound_modes: &mut SoundModes = state.get_mut();
                if sound_modes.ambient_sound_mode
                    == a3954::structures::AmbientSoundMode::NoiseCanceling
                {
                    let value = u8::try_from(value.try_as_i32()?.clamp(1, 5)).expect("clamped");
                    // Strength is the slider's distance from the center, so flip it around for <6
                    let adjusted_value = 6 - value;
                    sound_modes.sound_mode_slider = adjusted_value;
                }
            }
            SoundModesSetting::ManualTransparency => {
                let sound_modes: &mut SoundModes = state.get_mut();
                if sound_modes.ambient_sound_mode
                    == a3954::structures::AmbientSoundMode::Transparency
                {
                    // normal is 6, so transparency is 7 to 11
                    let value = u8::try_from(value.try_as_i32()?.clamp(1, 5)).expect("clamped") + 6;
                    sound_modes.sound_mode_slider = value;
                }
            }
            SoundModesSetting::AirplaneMode => {
                let sound_modes: &mut SoundModes = state.get_mut();
                sound_modes.airplane_mode = value.try_as_enum_variant()?;
            }
            SoundModesSetting::WindNoiseSuppression => {
                let sound_modes: &mut SoundModes = state.get_mut();
                sound_modes.wind_noise.is_suppression_enabled = value.try_as_bool()?;
            }
            SoundModesSetting::WindNoiseDetected => return Err(SettingHandlerError::ReadOnly),
        }
        Ok(())
    }
}
