use async_trait::async_trait;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::standard::{
        settings_manager::SettingHandler,
        structures::{InternalMultiButtonConfiguration, TwsStatus},
    },
};

use super::ButtonConfigurationSetting;

pub struct ButtonConfigurationSettingHandler {}

impl ButtonConfigurationSettingHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl<T> SettingHandler<T> for ButtonConfigurationSettingHandler
where
    T: AsMut<InternalMultiButtonConfiguration> + AsRef<InternalMultiButtonConfiguration> + Send,
    T: AsRef<TwsStatus>,
{
    fn settings(&self) -> Vec<SettingId> {
        ButtonConfigurationSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let button_config: &InternalMultiButtonConfiguration = state.as_ref();
        let setting: ButtonConfigurationSetting = setting_id.try_into().ok()?;
        Some(match setting {
            ButtonConfigurationSetting::LeftSinglePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config.left_single_click.enabled_action(),
                )
            }
            ButtonConfigurationSetting::LeftDoublePress => Setting::select_from_enum_all_variants(
                button_config.left_double_click.active_action(),
            ),
            ButtonConfigurationSetting::LeftLongPress => Setting::select_from_enum_all_variants(
                button_config.left_long_press.active_action(),
            ),
            ButtonConfigurationSetting::RightSinglePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config.right_single_click.enabled_action(),
                )
            }
            ButtonConfigurationSetting::RightDoublePress => Setting::select_from_enum_all_variants(
                button_config.right_double_click.active_action(),
            ),
            ButtonConfigurationSetting::RightLongPress => Setting::select_from_enum_all_variants(
                button_config.right_long_press.active_action(),
            ),
        })
    }

    async fn set(&self, state: &mut T, setting_id: &SettingId, value: Value) -> crate::Result<()> {
        let tws_status: TwsStatus = *state.as_ref();
        let button_config: &mut InternalMultiButtonConfiguration = state.as_mut();
        let setting: ButtonConfigurationSetting = setting_id
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            ButtonConfigurationSetting::LeftSinglePress => {
                if let Some(action) = value.try_as_optional_enum_variant()? {
                    button_config.left_single_click.action = action;
                    button_config.left_single_click.is_enabled = true;
                }
            }
            ButtonConfigurationSetting::LeftDoublePress => {
                button_config
                    .left_double_click
                    .set_action(value.try_as_enum_variant()?, tws_status.is_connected);
            }
            ButtonConfigurationSetting::LeftLongPress => {
                button_config
                    .left_long_press
                    .set_action(value.try_as_enum_variant()?, tws_status.is_connected);
            }
            ButtonConfigurationSetting::RightSinglePress => {
                if let Some(action) = value.try_as_optional_enum_variant()? {
                    button_config.right_single_click.action = action;
                    button_config.right_single_click.is_enabled = true;
                }
            }
            ButtonConfigurationSetting::RightDoublePress => {
                button_config
                    .right_double_click
                    .set_action(value.try_as_enum_variant()?, tws_status.is_connected);
            }
            ButtonConfigurationSetting::RightLongPress => {
                button_config
                    .right_long_press
                    .set_action(value.try_as_enum_variant()?, tws_status.is_connected);
            }
        }
        Ok(())
    }
}
