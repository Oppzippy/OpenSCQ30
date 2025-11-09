use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        settings_manager::{SettingHandler, SettingHandlerResult},
        structures::GamingMode,
    },
};

use super::GamingModeSetting;

#[derive(Default)]
pub struct GamingModeSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for GamingModeSettingHandler
where
    T: Has<GamingMode> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        GamingModeSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let gaming_mode = state.get();
        let setting: GamingModeSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            GamingModeSetting::GamingMode => Setting::Toggle {
                value: gaming_mode.is_enabled,
            },
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let gaming_mode = state.get_mut();
        let setting: GamingModeSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            GamingModeSetting::GamingMode => {
                let is_enabled = value.try_as_bool()?;
                *gaming_mode = GamingMode { is_enabled };
            }
        }
        Ok(())
    }
}
