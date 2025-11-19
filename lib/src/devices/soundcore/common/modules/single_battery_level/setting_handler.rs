use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        structures::BatteryLevel,
    },
};

use super::BatteryLevelSetting;

pub struct BatteryLevelSettingHandler {
    max_level: u8,
}

impl BatteryLevelSettingHandler {
    pub fn new(max_level: u8) -> Self {
        Self { max_level }
    }
}

#[async_trait]
impl<T> SettingHandler<T> for BatteryLevelSettingHandler
where
    T: Has<BatteryLevel> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        BatteryLevelSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let battery_level = state.get();
        let setting: BatteryLevelSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            BatteryLevelSetting::BatteryLevel => {
                let text = format!("{}/{}", battery_level.0, self.max_level);
                Setting::Information {
                    value: text.clone(),
                    translated_value: text,
                }
            }
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
