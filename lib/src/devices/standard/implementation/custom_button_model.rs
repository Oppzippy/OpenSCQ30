use crate::{
    devices::standard::{
        packets::outbound::SetCustomButtonModelPacket, state::DeviceState,
        structures::CustomButtonModel,
    },
    soundcore_device::device::soundcore_command::CommandResponse,
};

pub fn set_custom_button_model(
    state: DeviceState,
    custom_button_model: CustomButtonModel,
) -> crate::Result<CommandResponse> {
    if !state.device_features.has_custom_button_model {
        return Err(crate::Error::FeatureNotSupported {
            feature_name: "custom button model",
        });
    }

    let prev_custom_button_model = state.custom_button_model.ok_or(crate::Error::MissingData {
        name: "custom button model",
    })?;
    if custom_button_model == prev_custom_button_model {
        return Ok(CommandResponse {
            packets: Vec::new(),
            new_state: state,
        });
    }

    let packet = SetCustomButtonModelPacket::new(custom_button_model);
    Ok(CommandResponse {
        packets: vec![packet.into()],
        new_state: DeviceState {
            custom_button_model: Some(custom_button_model),
            ..state
        },
    })
}
