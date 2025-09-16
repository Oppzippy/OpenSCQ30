use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3959,
        common::{
            settings_manager::{SettingHandler, SettingHandlerResult},
            structures::TwsStatus,
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
    T: Has<a3959::structures::MultiButtonConfiguration> + Has<TwsStatus> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        ButtonConfigurationSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let button_config: &a3959::structures::MultiButtonConfiguration = state.get();
        let tws_status: &TwsStatus = state.get();
        let setting: ButtonConfigurationSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            ButtonConfigurationSetting::LeftSinglePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .left_single_click
                        .active_action(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::LeftDoublePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .left_double_click
                        .active_action(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::LeftTriplePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .left_triple_click
                        .active_action(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::LeftLongPress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .left_long_press
                        .active_action(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::RightSinglePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .right_single_click
                        .active_action(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::RightDoublePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .right_double_click
                        .active_action(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::RightTriplePress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .right_triple_click
                        .active_action(tws_status.is_connected),
                )
            }
            ButtonConfigurationSetting::RightLongPress => {
                Setting::optional_select_from_enum_all_variants(
                    button_config
                        .right_long_press
                        .active_action(tws_status.is_connected),
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
        let tws_status: TwsStatus = *state.get();
        let button_config: &mut a3959::structures::MultiButtonConfiguration = state.get_mut();
        let setting: ButtonConfigurationSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");

        match setting {
            ButtonConfigurationSetting::LeftSinglePress => {
                let action = value.try_as_optional_enum_variant()?;
                button_config
                    .left_single_click
                    .set_action(action, tws_status.is_connected);
            }
            ButtonConfigurationSetting::LeftDoublePress => {
                let action = value.try_as_optional_enum_variant()?;
                button_config
                    .left_double_click
                    .set_action(action, tws_status.is_connected);
            }
            ButtonConfigurationSetting::LeftTriplePress => {
                let action = value.try_as_optional_enum_variant()?;
                button_config
                    .left_triple_click
                    .set_action(action, tws_status.is_connected);
            }
            ButtonConfigurationSetting::LeftLongPress => {
                let action = value.try_as_optional_enum_variant()?;
                button_config
                    .left_long_press
                    .set_action(action, tws_status.is_connected);
            }
            ButtonConfigurationSetting::RightSinglePress => {
                let action = value.try_as_optional_enum_variant()?;
                button_config
                    .right_single_click
                    .set_action(action, tws_status.is_connected);
            }
            ButtonConfigurationSetting::RightDoublePress => {
                let action = value.try_as_optional_enum_variant()?;
                button_config
                    .right_double_click
                    .set_action(action, tws_status.is_connected);
            }
            ButtonConfigurationSetting::RightTriplePress => {
                let action = value.try_as_optional_enum_variant()?;
                button_config
                    .right_triple_click
                    .set_action(action, tws_status.is_connected);
            }
            ButtonConfigurationSetting::RightLongPress => {
                let action = value.try_as_optional_enum_variant()?;
                button_config
                    .right_long_press
                    .set_action(action, tws_status.is_connected);
            }
        }
        Ok(())
    }
}
