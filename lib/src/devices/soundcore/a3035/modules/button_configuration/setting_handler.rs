use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3035,
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
    },
    i18n::fl,
};

use super::ButtonConfigurationSetting;

#[derive(Default)]
pub struct ButtonConfigurationSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for ButtonConfigurationSettingHandler
where
    T: Has<a3035::structures::ButtonConfiguration> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        ButtonConfigurationSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let button_configuration = state.get();
        let button_configuration_setting: ButtonConfigurationSetting =
            (*setting_id).try_into().ok()?;
        Some(match button_configuration_setting {
            ButtonConfigurationSetting::SinglePress => Setting::Information {
                value: "AmbientSoundMode".to_string(),
                translated_value: fl!("ambient-sound-mode"),
            },
            ButtonConfigurationSetting::DoublePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_configuration.double_press_action,
                )
            }
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let button_configuration = state.get_mut();
        let button_configuration_setting: ButtonConfigurationSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match button_configuration_setting {
            ButtonConfigurationSetting::SinglePress => return Err(SettingHandlerError::ReadOnly),
            ButtonConfigurationSetting::DoublePress => {
                button_configuration.double_press_action = value.try_as_optional_enum_variant()?;
            }
        }
        Ok(())
    }
}
