use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        structures::CaseBatteryLevel,
    },
};

use super::CaseBatteryLevelSetting;

pub struct CaseBatteryLevelSettingHandler {
    max_level: u8,
}

impl CaseBatteryLevelSettingHandler {
    pub fn new(max_level: u8) -> Self {
        Self { max_level }
    }
}

#[async_trait]
impl<T> SettingHandler<T> for CaseBatteryLevelSettingHandler
where
    T: Has<CaseBatteryLevel> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        CaseBatteryLevelSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let battery = state.get();
        let setting: CaseBatteryLevelSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            CaseBatteryLevelSetting::CaseBatteryLevel => Setting::Information {
                value: battery.0.0.to_string(),
                translated_value: format!("{}/{}", battery.0.0, self.max_level),
            },
        })
    }

    async fn set(
        &self,
        _state: &mut T,
        _setting_id: &SettingId,
        _value: Value,
    ) -> SettingHandlerResult<()> {
        Err(SettingHandlerError::ReadOnly)
    }
}
