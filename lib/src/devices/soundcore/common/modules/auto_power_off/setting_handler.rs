use std::{borrow::Cow, iter};

use async_trait::async_trait;
use openscq30_i18n::Translate;
use openscq30_lib_has::MaybeHas;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::soundcore::common::{
        modules::auto_power_off::AutoPowerOffSetting,
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        structures::{AutoPowerOff, AutoPowerOffDurationIndex},
    },
    i18n::fl,
};

pub struct AutoPowerOffSettingHandler<Duration: 'static> {
    durations: &'static [Duration],
}

impl<Duration> AutoPowerOffSettingHandler<Duration> {
    pub fn new(durations: &'static [Duration]) -> Self {
        Self { durations }
    }
}

#[async_trait]
impl<Duration, T> SettingHandler<T> for AutoPowerOffSettingHandler<Duration>
where
    Duration: Translate + Send + Sync,
    &'static str: for<'a> From<&'a Duration>,
    T: MaybeHas<AutoPowerOff> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        AutoPowerOffSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let auto_power_off = state.maybe_get()?;
        self.get_inner(auto_power_off, setting_id)
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
        self.set_inner(auto_power_off, setting_id, value)
    }
}

impl<Duration> AutoPowerOffSettingHandler<Duration>
where
    Duration: Translate + Send + Sync,
    &'static str: for<'a> From<&'a Duration>,
{
    #[inline(never)]
    fn get_inner(&self, auto_power_off: &AutoPowerOff, setting_id: &SettingId) -> Option<Setting> {
        let setting: AutoPowerOffSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            AutoPowerOffSetting::AutoPowerOff => Setting::Select {
                setting: settings::Select {
                    options: iter::once("disabled")
                        .chain(self.durations.iter().map(Into::into))
                        .map(Cow::from)
                        .collect(),
                    localized_options: iter::once(fl!("disabled"))
                        .chain(self.durations.iter().map(|duration| duration.translate()))
                        .collect(),
                },
                value: if let Some(duration) =
                    self.durations.get(auto_power_off.duration.0 as usize)
                    && auto_power_off.is_enabled
                {
                    <&'static str>::from(duration).into()
                } else {
                    "disabled".into()
                },
            },
        })
    }

    #[inline(never)]
    fn set_inner(
        &self,
        auto_power_off: &mut AutoPowerOff,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let setting: AutoPowerOffSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            AutoPowerOffSetting::AutoPowerOff => {
                let selection = value.try_as_str()?;
                if let Some(duration_index) = self.durations.iter().position(|duration| {
                    <&'static str>::from(duration).eq_ignore_ascii_case(selection)
                }) {
                    auto_power_off.is_enabled = true;
                    auto_power_off.duration = AutoPowerOffDurationIndex(duration_index as u8);
                } else {
                    auto_power_off.is_enabled = false;
                }
            }
        }
        Ok(())
    }
}
