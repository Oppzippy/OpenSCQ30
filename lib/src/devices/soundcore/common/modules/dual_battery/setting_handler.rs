use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        modules::dual_battery::DualBatteryConfiguration,
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        structures::DualBattery,
    },
    i18n::fl,
};

use super::BatterySetting;

pub struct BatterySettingHandler {
    max_level: u8,
    level_offset: u8,
}

impl BatterySettingHandler {
    pub fn new(configuration: &DualBatteryConfiguration) -> Self {
        Self {
            max_level: configuration.max_level,
            level_offset: configuration.level_offset,
        }
    }
}

#[async_trait]
impl<T> SettingHandler<T> for BatterySettingHandler
where
    T: Has<DualBattery> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        BatterySetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let battery = state.get();
        let setting: BatterySetting = (*setting_id).try_into().ok()?;
        // battery level 255 means TWS is disconnected and the host device is the other side
        // in other words, the side for which the battery level is 255 is disconnected
        let left_side_present = battery.left.level.0 != 255;
        let right_side_present = battery.right.level.0 != 255;
        match setting {
            BatterySetting::IsChargingLeft if left_side_present => Some(Setting::Information {
                value: battery.left.is_charging.to_string(),
                translated_value: if battery.left.is_charging.into() {
                    fl!("charging")
                } else {
                    fl!("not-charging")
                },
            }),
            BatterySetting::IsChargingRight if right_side_present => Some(Setting::Information {
                value: battery.right.is_charging.to_string(),
                translated_value: if battery.right.is_charging.into() {
                    fl!("charging")
                } else {
                    fl!("not-charging")
                },
            }),
            BatterySetting::BatteryLevelLeft if left_side_present => Some(Setting::Information {
                value: format!(
                    "{}/{}",
                    battery.left.level.0 + self.level_offset,
                    self.max_level
                ),
                translated_value: fl!(
                    "percent",
                    percent = ((i32::from(battery.left.level.0 + self.level_offset) * 100)
                        / i32::from(self.max_level))
                ),
            }),
            BatterySetting::BatteryLevelRight if right_side_present => Some(Setting::Information {
                value: format!(
                    "{}/{}",
                    battery.right.level.0 + self.level_offset,
                    self.max_level
                ),
                translated_value: fl!(
                    "percent",
                    percent = ((i32::from(battery.right.level.0 + self.level_offset) * 100)
                        / i32::from(self.max_level))
                ),
            }),
            _ => None,
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
