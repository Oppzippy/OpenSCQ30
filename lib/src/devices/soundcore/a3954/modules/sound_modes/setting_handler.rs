use std::sync::Mutex;

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

/// The sound mode slider is positioned from 1 to 11. 6 is normal mode, and the further it is from 6, the stronger
/// the effect. 1 to 5 is noise canceling, 7 to 11 is transparency.
#[derive(Default)]
pub struct SoundModesSettingHandler {
    /// The device only stores the strength of the active mode, so switching to another mode would lose it. We
    /// remember the last strength of each mode so that switching back doesn't reset it to the minimum.
    remembered_strengths: Mutex<RememberedStrengths>,
}

#[derive(Default, Clone, Copy)]
struct RememberedStrengths {
    noise_canceling: Option<u8>,
    transparency: Option<u8>,
}

impl RememberedStrengths {
    /// Remembers the strength represented by the slider's position, if it is the strength of a mode.
    fn remember(&mut self, sound_mode_slider: u8) {
        match sound_mode_slider {
            // Strength is the slider's distance from the center, so flip it around for <6
            1..=5 => self.noise_canceling = Some(6 - sound_mode_slider),
            7..=11 => self.transparency = Some(sound_mode_slider - 6),
            _ => (),
        }
    }

    fn noise_canceling_slider(&self) -> u8 {
        6 - self.noise_canceling.unwrap_or(1)
    }

    fn transparency_slider(&self) -> u8 {
        6 + self.transparency.unwrap_or(1)
    }
}

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
        // The device can change modes or strengths without us, such as from a button press on the earbuds. Settings
        // are read whenever the state changes, so this is how we find out about those changes.
        self.remembered_strengths
            .lock()
            .unwrap()
            .remember(sound_modes.sound_mode_slider);
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
                    let mut remembered_strengths = self.remembered_strengths.lock().unwrap();
                    // the slider is about to be repositioned, so remember the strength of the mode we're leaving
                    remembered_strengths.remember(sound_modes.sound_mode_slider);
                    sound_modes.ambient_sound_mode = ambient_sound_mode;
                    match sound_modes.ambient_sound_mode {
                        a3954::structures::AmbientSoundMode::NoiseCanceling => {
                            sound_modes.sound_mode_slider =
                                remembered_strengths.noise_canceling_slider();
                        }
                        a3954::structures::AmbientSoundMode::Normal => {
                            sound_modes.sound_mode_slider = 6;
                        }
                        a3954::structures::AmbientSoundMode::Transparency => {
                            sound_modes.sound_mode_slider =
                                remembered_strengths.transparency_slider();
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
                    self.remembered_strengths
                        .lock()
                        .unwrap()
                        .remember(adjusted_value);
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
                    self.remembered_strengths.lock().unwrap().remember(value);
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

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::a3954::structures::AmbientSoundMode;

    use super::*;

    #[derive(openscq30_lib_macros::Has)]
    struct TestState {
        sound_modes: SoundModes,
    }

    fn state(ambient_sound_mode: AmbientSoundMode, sound_mode_slider: u8) -> TestState {
        TestState {
            sound_modes: SoundModes {
                ambient_sound_mode,
                sound_mode_slider,
                ..Default::default()
            },
        }
    }

    async fn set_mode(
        handler: &SoundModesSettingHandler,
        state: &mut TestState,
        mode: &'static str,
    ) {
        set(
            handler,
            state,
            SettingId::AmbientSoundMode,
            Value::from(mode),
        )
        .await;
    }

    async fn set_strength(
        handler: &SoundModesSettingHandler,
        state: &mut TestState,
        setting_id: SettingId,
        strength: i32,
    ) {
        set(handler, state, setting_id, Value::from(strength)).await;
    }

    async fn set(
        handler: &SoundModesSettingHandler,
        state: &mut TestState,
        setting_id: SettingId,
        value: Value,
    ) {
        SettingHandler::<TestState>::set(handler, state, &setting_id, value)
            .await
            .unwrap();
    }

    fn strength(
        handler: &SoundModesSettingHandler,
        state: &TestState,
        setting_id: SettingId,
    ) -> Option<i32> {
        match SettingHandler::<TestState>::get(handler, state, &setting_id)? {
            Setting::I32Range { value, .. } => Some(value),
            setting => panic!("expected I32Range, got {setting:?}"),
        }
    }

    #[tokio::test(start_paused = true)]
    async fn returning_to_anc_restores_previous_strength() {
        let handler = SoundModesSettingHandler::default();
        let mut state = state(AmbientSoundMode::Normal, 6);
        set_mode(&handler, &mut state, "NoiseCanceling").await;
        set_strength(&handler, &mut state, SettingId::ManualNoiseCanceling, 5).await;

        set_mode(&handler, &mut state, "Transparency").await;
        set_mode(&handler, &mut state, "NoiseCanceling").await;

        assert_eq!(
            strength(&handler, &state, SettingId::ManualNoiseCanceling),
            Some(5)
        );
        assert_eq!(state.sound_modes.sound_mode_slider, 1);
    }

    #[tokio::test(start_paused = true)]
    async fn returning_to_transparency_restores_previous_strength() {
        let handler = SoundModesSettingHandler::default();
        let mut state = state(AmbientSoundMode::Normal, 6);
        set_mode(&handler, &mut state, "Transparency").await;
        set_strength(&handler, &mut state, SettingId::ManualTransparency, 4).await;

        set_mode(&handler, &mut state, "NoiseCanceling").await;
        set_mode(&handler, &mut state, "Transparency").await;

        assert_eq!(
            strength(&handler, &state, SettingId::ManualTransparency),
            Some(4)
        );
        assert_eq!(state.sound_modes.sound_mode_slider, 10);
    }

    #[tokio::test(start_paused = true)]
    async fn strength_is_restored_after_passing_through_normal_mode() {
        let handler = SoundModesSettingHandler::default();
        let mut state = state(AmbientSoundMode::NoiseCanceling, 2);

        set_mode(&handler, &mut state, "Normal").await;
        assert_eq!(state.sound_modes.sound_mode_slider, 6);
        set_mode(&handler, &mut state, "NoiseCanceling").await;

        assert_eq!(
            strength(&handler, &state, SettingId::ManualNoiseCanceling),
            Some(4)
        );
    }

    #[tokio::test(start_paused = true)]
    async fn anc_and_transparency_strengths_are_remembered_independently() {
        let handler = SoundModesSettingHandler::default();
        let mut state = state(AmbientSoundMode::Normal, 6);
        set_mode(&handler, &mut state, "NoiseCanceling").await;
        set_strength(&handler, &mut state, SettingId::ManualNoiseCanceling, 4).await;
        set_mode(&handler, &mut state, "Transparency").await;
        set_strength(&handler, &mut state, SettingId::ManualTransparency, 2).await;

        for _ in 0..2 {
            set_mode(&handler, &mut state, "NoiseCanceling").await;
            assert_eq!(
                strength(&handler, &state, SettingId::ManualNoiseCanceling),
                Some(4)
            );
            set_mode(&handler, &mut state, "Transparency").await;
            assert_eq!(
                strength(&handler, &state, SettingId::ManualTransparency),
                Some(2)
            );
        }
    }

    #[tokio::test(start_paused = true)]
    async fn strength_changed_on_the_device_is_restored() {
        let handler = SoundModesSettingHandler::default();
        // the device reports strengths that we never set ourselves
        let anc_state_from_device = state(AmbientSoundMode::NoiseCanceling, 2);
        assert_eq!(
            strength(
                &handler,
                &anc_state_from_device,
                SettingId::ManualNoiseCanceling
            ),
            Some(4)
        );
        let mut transparency_state_from_device = state(AmbientSoundMode::Transparency, 9);
        assert_eq!(
            strength(
                &handler,
                &transparency_state_from_device,
                SettingId::ManualTransparency
            ),
            Some(3)
        );

        set_mode(
            &handler,
            &mut transparency_state_from_device,
            "NoiseCanceling",
        )
        .await;

        assert_eq!(
            strength(
                &handler,
                &transparency_state_from_device,
                SettingId::ManualNoiseCanceling
            ),
            Some(4)
        );
    }

    #[tokio::test(start_paused = true)]
    async fn modes_that_were_never_used_start_at_minimum_strength() {
        let handler = SoundModesSettingHandler::default();
        let mut state = state(AmbientSoundMode::Normal, 6);

        set_mode(&handler, &mut state, "NoiseCanceling").await;
        assert_eq!(
            strength(&handler, &state, SettingId::ManualNoiseCanceling),
            Some(1)
        );
        set_mode(&handler, &mut state, "Transparency").await;
        assert_eq!(
            strength(&handler, &state, SettingId::ManualTransparency),
            Some(1)
        );
    }
}
