use std::sync::Mutex;

use crate::{
    devices::standard::{
        packets::outbound::SetMultiButtonConfigurationPacket, state::DeviceState,
        structures::InternalMultiButtonConfiguration,
    },
    soundcore_device::device::soundcore_command::CommandResponse,
};

use super::{MultiButtonConfiguration, TwsStatus};

#[derive(Debug, Default)]
pub struct ButtonConfigurationImplementation {
    data: Mutex<Option<InternalMultiButtonConfiguration>>,
}

impl ButtonConfigurationImplementation {
    pub(crate) fn set_multi_button_configuration(
        &self,
        tws_status: &TwsStatus,
        actions: MultiButtonConfiguration,
    ) -> InternalMultiButtonConfiguration {
        let mut internal = self
            .data
            .lock()
            .unwrap()
            .expect("internal data should be set during initialization");

        internal.left_single_click.action = actions.left_single_click.action;
        internal.left_single_click.is_enabled = actions.left_single_click.is_enabled;
        internal
            .left_double_click
            .set_action(actions.left_double_click.action, tws_status.is_connected);
        internal
            .left_long_press
            .set_action(actions.left_long_press.action, tws_status.is_connected);
        internal.right_single_click.action = actions.right_single_click.action;
        internal.right_single_click.is_enabled = actions.right_single_click.is_enabled;
        internal
            .right_double_click
            .set_action(actions.right_double_click.action, tws_status.is_connected);
        internal
            .right_long_press
            .set_action(actions.right_long_press.action, tws_status.is_connected);

        internal
    }

    pub(crate) fn set_internal_data(&self, data: InternalMultiButtonConfiguration) {
        *self.data.lock().unwrap() = Some(data);
    }
}

pub fn set_multi_button_configuration(
    state: DeviceState,
    implementation: &ButtonConfigurationImplementation,
    button_configuration: MultiButtonConfiguration,
) -> crate::Result<CommandResponse> {
    if !state.device_features.has_button_configuration {
        return Err(crate::Error::FeatureNotSupported {
            feature_name: "custom button model",
        });
    }
    let Some(tws_status) = state.tws_status else {
        return Err(crate::Error::MissingData { name: "tws status" });
    };

    let prev_button_configuration =
        state
            .button_configuration
            .ok_or(crate::Error::MissingData {
                name: "custom button model",
            })?;
    if button_configuration == prev_button_configuration {
        return Ok(CommandResponse {
            packets: Vec::new(),
            new_state: state,
        });
    }

    // We don't want to update the state directly since MultiButtonConfiguration -> internal representation -> MultiButtonConfiguration
    // is not guaranteed to be the same as the original
    let new_button_configuration =
        implementation.set_multi_button_configuration(&tws_status, button_configuration);
    let packet = SetMultiButtonConfigurationPacket::new(new_button_configuration);
    Ok(CommandResponse {
        packets: vec![packet.into()],
        new_state: DeviceState {
            button_configuration: Some(new_button_configuration.into()),
            ..state
        },
    })
}
