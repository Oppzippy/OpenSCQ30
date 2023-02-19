use crate::{
    packets::inbound::StateUpdatePacket,
    state::{DeviceState, DeviceStateTransformer},
};

impl DeviceStateTransformer for StateUpdatePacket {
    fn transform(&self, _state: &DeviceState) -> DeviceState {
        self.into()
    }
}
