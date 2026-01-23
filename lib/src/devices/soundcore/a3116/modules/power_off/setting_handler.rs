use async_trait::async_trait;
use openscq30_lib_has::MaybeHas;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3116::{self, modules::power_off::PowerOffSetting},
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
    },
};

pub struct PowerOffSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for PowerOffSettingHandler
where
    T: MaybeHas<a3116::structures::PowerOffPending> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        PowerOffSetting::iter().map(Into::into).collect()
    }

    fn get(&self, _state: &T, setting_id: &SettingId) -> Option<Setting> {
        let setting: PowerOffSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            PowerOffSetting::PowerOff => Setting::Action,
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        _value: Value,
    ) -> SettingHandlerResult<()> {
        let power_off_pending = state
            .maybe_get_mut()
            .ok_or(SettingHandlerError::MissingData)?;
        let setting: PowerOffSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            PowerOffSetting::PowerOff => {
                *power_off_pending = a3116::structures::PowerOffPending(true);
            }
        }
        Ok(())
    }
}
