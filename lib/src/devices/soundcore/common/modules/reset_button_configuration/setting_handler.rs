use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        settings_manager::{SettingHandler, SettingHandlerResult},
    },
};

use super::ResetButtonConfigurationSetting;

pub struct ResetButtonConfigurationSettingHandler {}

impl ResetButtonConfigurationSettingHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl<T> SettingHandler<T> for ResetButtonConfigurationSettingHandler
where
    T: Has<ResetButtonConfigurationPending> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        ResetButtonConfigurationSetting::iter()
            .map(Into::into)
            .collect()
    }

    fn get(&self, _state: &T, setting_id: &SettingId) -> Option<Setting> {
        let setting: ResetButtonConfigurationSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            ResetButtonConfigurationSetting::ResetButtonsToDefault => Setting::Action,
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        _value: Value,
    ) -> SettingHandlerResult<()> {
        let setting: ResetButtonConfigurationSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            ResetButtonConfigurationSetting::ResetButtonsToDefault => {
                let pending = state.get_mut();
                pending.0 = true;
            }
        }
        Ok(())
    }
}
