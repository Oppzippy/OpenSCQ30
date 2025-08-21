use std::{borrow::Cow, iter};

use async_trait::async_trait;
use openscq30_i18n::Translate;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::soundcore::{
        a3028::{
            modules::auto_power_off::AutoPowerOffSetting,
            packets::{AutoPowerOff, AutoPowerOffDuration},
        },
        standard::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
    },
    i18n::fl,
};

pub struct AutoPowerOffSettingHandler {}

impl AutoPowerOffSettingHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl<T> SettingHandler<T> for AutoPowerOffSettingHandler
where
    T: AsMut<Option<AutoPowerOff>> + AsRef<Option<AutoPowerOff>> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        AutoPowerOffSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let Some(auto_power_off) = state.as_ref() else {
            return None;
        };
        let setting: AutoPowerOffSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            AutoPowerOffSetting::AutoPowerOff => Setting::Select {
                setting: settings::Select {
                    options: iter::once("disabled")
                        .chain(AutoPowerOffDuration::iter().map(Into::into))
                        .map(Cow::from)
                        .collect(),
                    localized_options: iter::once(fl!("disabled"))
                        .chain(AutoPowerOffDuration::iter().map(|duration| duration.translate()))
                        .collect(),
                },
                value: if auto_power_off.enabled {
                    Cow::Borrowed(auto_power_off.duration.into())
                } else {
                    "disabled".into()
                },
            },
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let Some(auto_power_off) = state.as_mut() else {
            return Err(SettingHandlerError::DoesNotExist);
        };
        let setting: AutoPowerOffSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            AutoPowerOffSetting::AutoPowerOff => {
                let selection = value.try_as_str()?;
                if let Some(duration) = AutoPowerOffDuration::iter().find(|duration| {
                    selection == (<AutoPowerOffDuration as Into<&'static str>>::into(*duration))
                }) {
                    auto_power_off.enabled = true;
                    auto_power_off.duration = duration;
                } else {
                    auto_power_off.enabled = false;
                }
            }
        }
        Ok(())
    }
}
