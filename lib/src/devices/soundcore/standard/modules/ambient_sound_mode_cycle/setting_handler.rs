use async_trait::async_trait;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::standard::{
        settings_manager::{SettingHandler, SettingHandlerResult},
        structures::AmbientSoundModeCycle,
    },
};

use super::SoundModeCycleSetting;

pub struct AmbientSoundModeCycleSettingHandler {}

impl AmbientSoundModeCycleSettingHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl<T> SettingHandler<T> for AmbientSoundModeCycleSettingHandler
where
    T: AsMut<AmbientSoundModeCycle> + AsRef<AmbientSoundModeCycle> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SoundModeCycleSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let cycle = state.as_ref();
        let setting: SoundModeCycleSetting = setting_id.try_into().ok()?;
        Some(match setting {
            SoundModeCycleSetting::NormalModeInCycle => Setting::Toggle {
                value: cycle.normal_mode,
            },
            SoundModeCycleSetting::TransparencyModeInCycle => Setting::Toggle {
                value: cycle.transparency_mode,
            },
            SoundModeCycleSetting::NoiseCancelingModeInCycle => Setting::Toggle {
                value: cycle.noise_canceling_mode,
            },
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let cycle = state.as_mut();
        let setting: SoundModeCycleSetting = setting_id
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            SoundModeCycleSetting::NormalModeInCycle => {
                cycle.normal_mode = value.try_as_bool()?;
            }
            SoundModeCycleSetting::TransparencyModeInCycle => {
                cycle.transparency_mode = value.try_as_bool()?;
            }
            SoundModeCycleSetting::NoiseCancelingModeInCycle => {
                cycle.noise_canceling_mode = value.try_as_bool()?;
            }
        }
        Ok(())
    }
}
