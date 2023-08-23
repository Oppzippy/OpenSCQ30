use crate::{
    packets::inbound::state_update_packet::StateUpdatePacket,
    state::{DeviceState, DeviceStateTransformer},
};

impl DeviceStateTransformer for StateUpdatePacket {
    fn transform(&self, state: &DeviceState) -> DeviceState {
        DeviceState {
            feature_flags: state.feature_flags,
            battery: state.battery,
            equalizer_configuration: state.equalizer_configuration,
            age_range: self.age_range.or(state.age_range),
            gender: self.gender.or(state.gender),
            custom_button_model: self.custom_button_model.or(state.custom_button_model),
            custom_hear_id: self.custom_hear_id.or(state.custom_hear_id),
            left_firmware_version: self
                .firmware_version
                .as_ref()
                .or(state.left_firmware_version.as_ref())
                .cloned(),
            right_firmware_version: state.right_firmware_version,
            serial_number: self
                .serial_number
                .as_ref()
                .or(state.serial_number.as_ref())
                .cloned(),
            sound_modes: self.sound_modes.or(state.sound_modes),
            dynamic_range_compression_min_firmware_version: state
                .dynamic_range_compression_min_firmware_version,
        }
    }
}
