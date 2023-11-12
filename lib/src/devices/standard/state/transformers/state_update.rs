use crate::devices::standard::{
    packets::inbound::state_update_packet::StateUpdatePacket,
    state::{DeviceState, DeviceStateTransformer},
};
impl DeviceStateTransformer for StateUpdatePacket {
    fn transform(&self, state: &DeviceState) -> DeviceState {
        DeviceState {
            device_profile: state.device_profile,
            battery: state.battery,
            equalizer_configuration: state.equalizer_configuration.to_owned(),
            age_range: self.age_range.or(state.age_range),
            gender: self.gender.or(state.gender),
            custom_button_model: self.custom_button_model.or(state.custom_button_model),
            hear_id: self.hear_id.to_owned().or(state.hear_id.to_owned()),
            firmware_version: self
                .firmware_version
                .as_ref()
                .or(state.firmware_version.as_ref())
                .cloned(),
            serial_number: self
                .serial_number
                .as_ref()
                .or(state.serial_number.as_ref())
                .cloned(),
            sound_modes: self.sound_modes.or(state.sound_modes),
        }
    }
}
