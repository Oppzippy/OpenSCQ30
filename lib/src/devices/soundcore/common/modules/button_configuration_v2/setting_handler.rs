use std::borrow::Cow;

use async_trait::async_trait;
use openscq30_lib_has::Has;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        modules::button_configuration_v2::{ButtonConfigurationSettings, ButtonDisableMode},
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        structures::{
            TwsStatus,
            button_configuration_v2::{Button, ButtonStatusCollection},
        },
    },
    settings,
};

pub struct ButtonConfigurationSettingHandler<const NUM_BUTTONS: usize, const NUM_PRESS_KINDS: usize>
{
    settings: &'static ButtonConfigurationSettings<NUM_BUTTONS, NUM_PRESS_KINDS>,
}

impl<const NUM_BUTTONS: usize, const NUM_PRESS_KINDS: usize>
    ButtonConfigurationSettingHandler<NUM_BUTTONS, NUM_PRESS_KINDS>
{
    pub fn new(
        settings: &'static ButtonConfigurationSettings<NUM_BUTTONS, NUM_PRESS_KINDS>,
    ) -> Self {
        Self { settings }
    }
}

#[async_trait]
impl<T, const NUM_BUTTONS: usize, const NUM_PRESS_KINDS: usize> SettingHandler<T>
    for ButtonConfigurationSettingHandler<NUM_BUTTONS, NUM_PRESS_KINDS>
where
    T: Has<ButtonStatusCollection<NUM_BUTTONS>> + Has<TwsStatus> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        self.settings.order.map(|button| button.into()).to_vec()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let button = Button::try_from(*setting_id).ok()?;
        let tws_status: TwsStatus = *state.get();
        let statuses: &ButtonStatusCollection<_> = state.get();

        let position = self.settings.position(button)?;
        let status = statuses.0[position];
        let button_settings = self
            .settings
            .button_settings(button)
            .expect("setting id already validated by caller");

        let select = settings::Select {
            options: button_settings
                .available_actions
                .iter()
                .map(|action| Cow::from(action.name))
                .collect(),
            localized_options: button_settings
                .available_actions
                .iter()
                .map(|action| (action.localized_name)())
                .collect(),
        };
        let value = status.current_action_id(tws_status).and_then(|action_id| {
            button_settings
                .available_actions
                .iter()
                .find(|a| a.id == action_id)
                .map(|action| Cow::from(action.name))
        });

        // Show optional select if the button isn't disablable, but we don't have a value for it. This can happen
        // if the action id is set to an invalid action or if the button is disabled anyway.
        if button_settings.disable_mode == ButtonDisableMode::NotDisablable
            && let Some(value) = value
        {
            Some(settings::Setting::Select {
                setting: select,
                value,
            })
        } else {
            Some(settings::Setting::OptionalSelect {
                setting: select,
                value,
            })
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let button = Button::try_from(*setting_id)
            .expect("already filtered to valid values only by SettingsManager");
        let tws_status: TwsStatus = *state.get();
        let statuses: &mut ButtonStatusCollection<_> = state.get_mut();

        let position = self
            .settings
            .position(button)
            .ok_or(SettingHandlerError::MissingData)?;
        let status = &mut statuses.0[position];
        let button_settings = self
            .settings
            .button_settings(button)
            .expect("setting id already validated by caller");

        let maybe_action = value.try_as_optional_str()?.and_then(|action_name| {
            button_settings
                .available_actions
                .iter()
                .find(|action| action.name == action_name)
        });
        match button_settings.disable_mode {
            ButtonDisableMode::NotDisablable => {
                if let Some(action) = maybe_action {
                    *status = status.with_current_action_id(tws_status, Some(action.id));
                }
            }
            ButtonDisableMode::IndividualDisable => {
                *status =
                    status.with_current_action_id(tws_status, maybe_action.map(|action| action.id));
            }
            ButtonDisableMode::DisablingOneSideDisablesOther => {
                *status =
                    status.with_current_action_id(tws_status, maybe_action.map(|action| action.id));
                let other_side_pos = self
                    .settings
                    .order
                    .iter()
                    .position(|b| {
                        b.press_kind() == button.press_kind() && b.side() != button.side()
                    })
                    .expect("both sides should be listed");

                if !statuses.0[other_side_pos].is_enabled(tws_status) {
                    if let Some(action) = maybe_action {
                        let current_action_id =
                            statuses.0[other_side_pos].action.current(tws_status);
                        statuses.0[other_side_pos] = statuses.0[other_side_pos]
                            .with_current_action_id(
                                tws_status,
                                Some(if current_action_id != 0xF {
                                    current_action_id
                                } else {
                                    action.id
                                }),
                            )
                    }
                } else if maybe_action.is_none() {
                    statuses.0[other_side_pos] =
                        statuses.0[other_side_pos].with_current_action_id(tws_status, None)
                }
            }
        }

        Ok(())
    }
}
