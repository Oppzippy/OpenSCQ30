use nom::{combinator::all_consuming, error::VerboseError};

use crate::devices::standard::{
    packets::inbound::state_update_packet::StateUpdatePacket, state::DeviceState,
};

pub fn state_update_handler(input: &[u8], state: DeviceState) -> DeviceState {
    let result: Result<_, nom::Err<VerboseError<_>>> =
        all_consuming(StateUpdatePacket::take)(input);
    let packet = match result {
        Ok((_, packet)) => packet,
        Err(err) => {
            tracing::error!("failed to parse packet: {err:?}");
            return state;
        }
    };

    DeviceState {
        device_profile: state.device_profile,
        battery: state.battery,
        equalizer_configuration: state.equalizer_configuration.to_owned(),
        age_range: packet.age_range.or(state.age_range),
        gender: packet.gender.or(state.gender),
        custom_button_model: packet.custom_button_model.or(state.custom_button_model),
        hear_id: packet.hear_id.to_owned().or(state.hear_id.to_owned()),
        firmware_version: packet
            .firmware_version
            .as_ref()
            .or(state.firmware_version.as_ref())
            .cloned(),
        serial_number: packet
            .serial_number
            .as_ref()
            .or(state.serial_number.as_ref())
            .cloned(),
        sound_modes: packet.sound_modes.or(state.sound_modes),
        sound_modes_type_two: packet.sound_modes_type_two.or(state.sound_modes_type_two),
        ambient_sound_mode_cycle: packet
            .ambient_sound_mode_cycle
            .or(state.ambient_sound_mode_cycle),
    }
}
