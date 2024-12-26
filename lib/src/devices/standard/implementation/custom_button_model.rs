use std::sync::Mutex;

use crate::{
    devices::standard::{
        packets::outbound::SetCustomButtonModelPacket, state::DeviceState,
        structures::InternalCustomButtonModel,
    },
    soundcore_device::device::soundcore_command::CommandResponse,
};

use super::{CustomButtonActions, TwsStatus};

#[derive(Debug, Default)]
pub struct CustomButtonModelImplementation {
    data: Mutex<Option<InternalCustomButtonModel>>,
}

impl CustomButtonModelImplementation {
    pub(crate) fn set_custom_button_model(
        &self,
        tws_status: &TwsStatus,
        actions: CustomButtonActions,
    ) -> InternalCustomButtonModel {
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

        internal.clone()
    }

    pub(crate) fn set_internal_data(&self, data: InternalCustomButtonModel) {
        *self.data.lock().unwrap() = Some(data);
    }
}

pub fn set_custom_button_model(
    state: DeviceState,
    data: &CustomButtonModelImplementation,
    custom_button_model: CustomButtonActions,
) -> crate::Result<CommandResponse> {
    if !state.device_features.has_custom_button_model {
        return Err(crate::Error::FeatureNotSupported {
            feature_name: "custom button model",
        });
    }
    let Some(tws_status) = state.tws_status else {
        return Err(crate::Error::MissingData { name: "tws status" });
    };

    let prev_custom_button_model =
        state
            .custom_button_actions
            .ok_or(crate::Error::MissingData {
                name: "custom button model",
            })?;
    if custom_button_model == prev_custom_button_model {
        return Ok(CommandResponse {
            packets: Vec::new(),
            new_state: state,
        });
    }

    // We don't want to update the state directly since CustomButtonActions -> internal representation -> CustomButtonActions
    // is not guaranteed to be the same as the original
    let new_button_model = data.set_custom_button_model(&tws_status, custom_button_model);
    let packet = SetCustomButtonModelPacket::new(new_button_model);
    Ok(CommandResponse {
        packets: vec![packet.into()],
        new_state: DeviceState {
            custom_button_actions: Some(new_button_model.into()),
            ..state
        },
    })
}
