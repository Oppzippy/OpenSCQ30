use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        d1301::{modules::alarms::AlarmsSetting, structures::Alarm},
    },
};

#[derive(Default)]
pub struct AlarmsSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for AlarmsSettingHandler
where
    T: Has<Vec<Alarm>> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        AlarmsSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let alarms = state.get();
        let alarms_setting: AlarmsSetting = (*setting_id).try_into().ok()?;

        match alarms_setting {
            AlarmsSetting::Alarms => {
                let text = format!("not yet implemented\n{alarms:?}");
                Some(Setting::Information {
                    value: text.clone(),
                    translated_value: text,
                })
            }
        }
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
