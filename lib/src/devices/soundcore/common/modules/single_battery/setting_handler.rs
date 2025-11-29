use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        structures::SingleBattery,
    },
    i18n::fl,
};

use super::BatterySetting;

pub struct BatterySettingHandler {
    max_level: u8,
}

impl BatterySettingHandler {
    pub fn new(max_level: u8) -> Self {
        Self { max_level }
    }
}

#[async_trait]
impl<T> SettingHandler<T> for BatterySettingHandler
where
    T: Has<SingleBattery> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        BatterySetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let battery = state.get();
        let setting: BatterySetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            BatterySetting::IsCharging => Setting::Information {
                value: battery.is_charging.to_string(),
                translated_value: if battery.is_charging.into() {
                    fl!("charging")
                } else {
                    fl!("not-charging")
                },
            },
            BatterySetting::BatteryLevel => Setting::Information {
                value: format!("{}/5", battery.level.0),
                translated_value: fl!(
                    "percent",
                    percent = ((i32::from(battery.level.0) * 100) / i32::from(self.max_level))
                ),
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
