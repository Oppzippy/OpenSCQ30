use std::borrow::Cow;

use strum::IntoEnumIterator;

use crate::{
    api::settings::{Range, Setting, SettingId, Value},
    devices::standard::{
        settings_manager::SettingHandler,
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

impl<T> SettingHandler<T> for SoundModesSettingHandler
where
    T: AsMut<SoundModes> + AsRef<SoundModes>,
{
    fn settings(&self) -> Vec<SettingId<'static>> {
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
            .map(|mode| SettingId(Cow::Borrowed(mode.into())))
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let sound_modes = state.as_ref();
        let sound_mode_setting: SoundModeSetting = setting_id.0.parse().ok()?;
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
                    min: 0,
                    max: 10,
                    step: 1,
                },
                value: sound_modes.custom_noise_canceling.value().into(),
            },
        })
    }

    fn set(&self, state: &mut T, setting_id: &SettingId, value: Value) -> crate::Result<()> {
        let sound_modes = state.as_mut();
        let sound_mode_setting: SoundModeSetting = setting_id.0.parse().unwrap();
        match sound_mode_setting {
            SoundModeSetting::AmbientSoundMode => {
                let Some(selected_index) = value.try_as_u16() else {
                    panic!("got {value:?}")
                };
                sound_modes.ambient_sound_mode =
                    self.available_sound_modes.ambient_sound_modes[selected_index as usize];
            }
            SoundModeSetting::TransparencyMode => {
                let Some(selected_index) = value.try_as_u16() else {
                    panic!()
                };
                sound_modes.transparency_mode =
                    self.available_sound_modes.transparency_modes[selected_index as usize]
            }
            SoundModeSetting::NoiseCancelingMode => {
                let Some(selected_index) = value.try_as_u16() else {
                    panic!()
                };
                sound_modes.noise_canceling_mode =
                    self.available_sound_modes.noise_canceling_modes[selected_index as usize]
            }
            SoundModeSetting::CustomNoiseCanceling => {
                let Value::I32(value) = value else { panic!() };
                sound_modes.custom_noise_canceling = CustomNoiseCanceling::new(value as u8);
            }
        }
        Ok(())
    }
}
