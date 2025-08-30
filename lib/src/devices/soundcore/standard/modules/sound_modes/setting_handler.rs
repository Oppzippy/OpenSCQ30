use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Range, Setting, SettingId, Value},
    devices::soundcore::standard::{
        settings_manager::{SettingHandler, SettingHandlerResult},
        structures::{CustomNoiseCanceling, NoiseCancelingMode, SoundModes},
    },
};

use super::{AvailableSoundModes, SoundModeSetting};

pub struct SoundModesSettingHandler {
    available_sound_modes: AvailableSoundModes,
}

impl SoundModesSettingHandler {
    pub fn new(available_sound_modes: AvailableSoundModes) -> Self {
        Self {
            available_sound_modes,
        }
    }
}

#[async_trait]
impl<T> SettingHandler<T> for SoundModesSettingHandler
where
    T: Has<SoundModes> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SoundModeSetting::iter()
            .filter(|setting| match setting {
                SoundModeSetting::AmbientSoundMode => {
                    !self.available_sound_modes.ambient_sound_modes.is_empty()
                }
                SoundModeSetting::TransparencyMode => {
                    !self.available_sound_modes.transparency_modes.is_empty()
                }
                SoundModeSetting::NoiseCancelingMode => {
                    !self.available_sound_modes.noise_canceling_modes.is_empty()
                }
                SoundModeSetting::CustomNoiseCanceling => self
                    .available_sound_modes
                    .noise_canceling_modes
                    .contains(&NoiseCancelingMode::Custom),
            })
            .map(Into::into)
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let sound_modes = state.get();
        let sound_mode_setting: SoundModeSetting = (*setting_id).try_into().ok()?;
        Some(match sound_mode_setting {
            SoundModeSetting::AmbientSoundMode => Setting::select_from_enum(
                &self.available_sound_modes.ambient_sound_modes,
                sound_modes.ambient_sound_mode,
            ),
            SoundModeSetting::NoiseCancelingMode => Setting::select_from_enum(
                &self.available_sound_modes.noise_canceling_modes,
                sound_modes.noise_canceling_mode,
            ),
            SoundModeSetting::TransparencyMode => Setting::select_from_enum(
                &self.available_sound_modes.transparency_modes,
                sound_modes.transparency_mode,
            ),
            SoundModeSetting::CustomNoiseCanceling => Setting::I32Range {
                setting: Range {
                    range: 0..=10,
                    step: 1,
                },
                value: sound_modes.custom_noise_canceling.value().into(),
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
            SoundModeSetting::CustomNoiseCanceling => {
                sound_modes.custom_noise_canceling =
                    CustomNoiseCanceling::new(value.try_as_i32()? as u8);
            }
        }
        Ok(())
    }
}
