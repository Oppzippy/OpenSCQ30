use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3954::{self, modules::air_pressure::AirPressureSetting},
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
    },
    i18n::fl,
};

#[derive(Default)]
pub struct AirPressureSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for AirPressureSettingHandler
where
    T: Has<a3954::structures::AirPressure> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        AirPressureSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let air_pressure: &a3954::structures::AirPressure = state.get();
        let setting: AirPressureSetting = (*setting_id).try_into().ok()?;
        match setting {
            AirPressureSetting::AirPressure => Some(Setting::Information {
                value: air_pressure.to_string(),
                translated_value: fl!(
                    "x-standard-atmospheres",
                    pressure = air_pressure.to_string()
                ),
            }),
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
