use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3957::structures::PressureSensitivity,
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
};

use super::PressureSensitivitySetting;

#[derive(Default)]
pub struct PressureSensitivitySettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for PressureSensitivitySettingHandler
where
    T: Has<PressureSensitivity> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        PressureSensitivitySetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let pressure_sensitivity: &PressureSensitivity = state.get();
        let _: PressureSensitivitySetting = (*setting_id).try_into().ok()?;
        Some(Setting::select_from_enum_all_variants(
            *pressure_sensitivity,
        ))
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let _: PressureSensitivitySetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        let pressure_sensitivity: &mut PressureSensitivity = state.get_mut();
        *pressure_sensitivity = value.try_as_enum_variant()?;
        Ok(())
    }
}
