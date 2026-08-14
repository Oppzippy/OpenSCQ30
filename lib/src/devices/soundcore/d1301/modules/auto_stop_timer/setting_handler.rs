use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        common::settings_manager::{SettingHandler, SettingHandlerResult},
        d1301::structures::AutoStopTimer,
    },
    settings,
};

use super::AutoStopTimerSetting;

#[derive(Default)]
pub struct AutoStopTimerSettingHandler;

const MIN_DURATION: i32 = 1;
const MAX_DURATION: i32 = 240;

#[async_trait]
impl<T> SettingHandler<T> for AutoStopTimerSettingHandler
where
    T: Has<AutoStopTimer> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        AutoStopTimerSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let auto_stop_timer: &AutoStopTimer = state.get();
        let auto_stop_timer_setting: AutoStopTimerSetting = (*setting_id).try_into().ok()?;

        match auto_stop_timer_setting {
            AutoStopTimerSetting::AutoStopTimer => Some(Setting::Toggle {
                value: auto_stop_timer.is_enabled,
            }),
            AutoStopTimerSetting::AutoStopTimerDuration => Some(Setting::I32Range {
                setting: settings::Range {
                    range: MIN_DURATION..=MAX_DURATION,
                    step: 1,
                },
                value: auto_stop_timer.duration_in_minutes.into(),
            }),
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let spatial_audio_setting: AutoStopTimerSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match spatial_audio_setting {
            AutoStopTimerSetting::AutoStopTimer => {
                let auto_stop_timer = state.get_mut();
                auto_stop_timer.is_enabled = value.try_as_bool()?;
            }
            AutoStopTimerSetting::AutoStopTimerDuration => {
                let auto_stop_timer = state.get_mut();
                auto_stop_timer.duration_in_minutes =
                    i16::try_from(value.try_as_i32()?.clamp(MIN_DURATION, MAX_DURATION))
                        .expect("clamped to subset of i16");
            }
        }
        Ok(())
    }
}
