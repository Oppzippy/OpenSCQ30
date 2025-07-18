use async_trait::async_trait;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3959::structures::{A3959MultiButtonConfiguration, A3959TwsButtonAction},
        standard::{
            settings_manager::{SettingHandler, SettingHandlerResult},
            structures::{ButtonAction, TwsStatus},
        },
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
    T: AsMut<A3959MultiButtonConfiguration> + AsRef<A3959MultiButtonConfiguration> + Send,
    T: AsRef<TwsStatus>,
{
    fn settings(&self) -> Vec<SettingId> {
        ButtonConfigurationSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let button_config: &A3959MultiButtonConfiguration = state.as_ref();
        let tws_status: &TwsStatus = state.as_ref();
        let setting: ButtonConfigurationSetting = setting_id.try_into().ok()?;
        Some(match setting {
            ButtonConfigurationSetting::LeftSinglePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .left_single_click
                        .action_if_enabled(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::LeftDoublePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .left_double_click
                        .action_if_enabled(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::LeftLongPress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .left_long_press
                        .action_if_enabled(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::RightSinglePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .right_single_click
                        .action_if_enabled(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::RightDoublePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .right_double_click
                        .action_if_enabled(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::RightLongPress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .right_long_press
                        .action_if_enabled(tws_status.is_connected),
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
        let tws_status: TwsStatus = *state.as_ref();
        let button_config: &mut A3959MultiButtonConfiguration = state.as_mut();
        let setting: ButtonConfigurationSetting = setting_id
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");

        fn set_button(
            button: &mut A3959TwsButtonAction,
            action: Option<ButtonAction>,
            is_tws_connected: bool,
        ) {
            button.set_enabled(action.is_some(), is_tws_connected);
            if let Some(action) = action {
                button.set_action(action, is_tws_connected);
            }
        }

        match setting {
            ButtonConfigurationSetting::LeftSinglePress => {
                let action = value.try_as_optional_enum_variant()?;
                set_button(
                    &mut button_config.left_single_click,
                    action,
                    tws_status.is_connected,
                );
            }
            ButtonConfigurationSetting::LeftDoublePress => {
                let action = value.try_as_optional_enum_variant()?;
                set_button(
                    &mut button_config.left_double_click,
                    action,
                    tws_status.is_connected,
                );
            }
            ButtonConfigurationSetting::LeftLongPress => {
                let action = value.try_as_optional_enum_variant()?;
                set_button(
                    &mut button_config.left_long_press,
                    action,
                    tws_status.is_connected,
                );
            }
            ButtonConfigurationSetting::RightSinglePress => {
                let action = value.try_as_optional_enum_variant()?;
                set_button(
                    &mut button_config.right_single_click,
                    action,
                    tws_status.is_connected,
                );
            }
            ButtonConfigurationSetting::RightDoublePress => {
                let action = value.try_as_optional_enum_variant()?;
                set_button(
                    &mut button_config.right_double_click,
                    action,
                    tws_status.is_connected,
                );
            }
            ButtonConfigurationSetting::RightLongPress => {
                let action = value.try_as_optional_enum_variant()?;
                set_button(
                    &mut button_config.right_long_press,
                    action,
                    tws_status.is_connected,
                );
            }
        }
        Ok(())
    }
}
