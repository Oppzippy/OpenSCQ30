use async_trait::async_trait;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::standard::{settings_manager::SettingHandler, structures::DualBattery},
    i18n::fl,
};

use super::BatterySetting;

#[derive(Default)]
pub struct AmbientBatteryCycleSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for AmbientBatteryCycleSettingHandler
where
    T: AsMut<DualBattery> + AsRef<DualBattery> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        BatterySetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let battery = state.as_ref();
        let setting: BatterySetting = setting_id.try_into().ok()?;
        Some(match setting {
            BatterySetting::IsChargingLeft => Setting::Information {
                text: if battery.left.is_charging.into() {
                    fl!("charging")
                } else {
                    fl!("not-charging")
                },
            },
            BatterySetting::IsChargingRight => Setting::Information {
                text: if battery.right.is_charging.into() {
                    fl!("charging")
                } else {
                    fl!("not-charging")
                },
            },
            BatterySetting::BatteryLevelLeft => Setting::Information {
                text: format!("{}/5", battery.left.level.0),
            },
            BatterySetting::BatteryLevelRight => Setting::Information {
                text: format!("{}/5", battery.left.level.0),
            },
        })
    }

    async fn set(
        &self,
        _state: &mut T,
        _setting_id: &SettingId,
        _value: Value,
    ) -> crate::Result<()> {
        unimplemented!("battery is read only")
    }
}
