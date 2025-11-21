use std::borrow::Cow;

use async_trait::async_trait;
use openscq30_i18n::Translate;
use openscq30_lib_has::MaybeHas;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::soundcore::{
        a3116::{self, modules::auto_power_off::AutoPowerOffSetting},
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
    },
};

pub struct AutoPowerOffSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for AutoPowerOffSettingHandler
where
    T: MaybeHas<a3116::structures::AutoPowerOffDuration> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        AutoPowerOffSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let auto_power_off = state.maybe_get()?;
        let setting: AutoPowerOffSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            AutoPowerOffSetting::AutoPowerOff => Setting::Select {
                setting: settings::Select {
                    options: a3116::structures::AutoPowerOffDuration::iter()
                        .map(Into::into)
                        .map(Cow::Borrowed)
                        .collect(),
                    localized_options: a3116::structures::AutoPowerOffDuration::iter()
                        .map(|duration| duration.translate())
                        .collect(),
                },
                value: Cow::Borrowed(auto_power_off.into()),
            },
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let auto_power_off = state
            .maybe_get_mut()
            .ok_or(SettingHandlerError::MissingData)?;
        let setting: AutoPowerOffSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            AutoPowerOffSetting::AutoPowerOff => {
                *auto_power_off = value.try_as_enum_variant()?;
            }
        }
        Ok(())
    }
}
