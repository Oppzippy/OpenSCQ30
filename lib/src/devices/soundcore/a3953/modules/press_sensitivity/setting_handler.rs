use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::soundcore::{
        a3953::structures::PressSensitivity,
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
};

use super::PressSensitivitySetting;

#[derive(Default)]
pub struct PressSensitivitySettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for PressSensitivitySettingHandler
where
    T: Has<PressSensitivity> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        PressSensitivitySetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let press_sensitivity: &PressSensitivity = state.get();
        let _: PressSensitivitySetting = (*setting_id).try_into().ok()?;
        Some(Setting::I32Range {
            setting: settings::Range {
                range: 0..=4,
                step: 1,
            },
            value: i32::from(press_sensitivity.0),
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let _: PressSensitivitySetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        let press_sensitivity: &mut PressSensitivity = state.get_mut();
        press_sensitivity.0 = u8::try_from(value.try_as_i32()?.clamp(0, 4)).expect("clamped");
        Ok(())
    }
}
