use crate::{
    packets::inbound::StateUpdatePacket,
    state::{SoundcoreDeviceState, SoundcoreDeviceStateTransformer},
};

impl SoundcoreDeviceStateTransformer for StateUpdatePacket {
    fn transform(&self, _state: &SoundcoreDeviceState) -> SoundcoreDeviceState {
        self.into()
    }
}
