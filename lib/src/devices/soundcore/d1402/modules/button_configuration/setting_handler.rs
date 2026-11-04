use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        d1402,
    },
    i18n::fl,
};

use super::ButtonConfigurationSetting;

#[derive(Default)]
pub struct ButtonConfigurationSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for ButtonConfigurationSettingHandler
where
    T: Has<d1402::structures::ButtonConfiguration>
        + Has<d1402::structures::MfbButtonConfiguration>
        + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        ButtonConfigurationSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let button_configuration: &d1402::structures::ButtonConfiguration = state.get();
        let mfb_button_configuration: &d1402::structures::MfbButtonConfiguration = state.get();
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
            ButtonConfigurationSetting::MfbLongPress => {
                Setting::optional_select_from_enum_all_variants(mfb_button_configuration.action)
            }
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let button_configuration_setting: ButtonConfigurationSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match button_configuration_setting {
            ButtonConfigurationSetting::SinglePress => return Err(SettingHandlerError::ReadOnly),
            ButtonConfigurationSetting::DoublePress => {
                let button_configuration: &mut d1402::structures::ButtonConfiguration =
                    state.get_mut();
                button_configuration.set_double_press_action(value.try_as_optional_enum_variant()?);
            }
            ButtonConfigurationSetting::MfbLongPress => {
                let mfb_button_configuration: &mut d1402::structures::MfbButtonConfiguration =
                    state.get_mut();
                mfb_button_configuration.action = value.try_as_optional_enum_variant()?;
            }
        }
        Ok(())
    }
}
