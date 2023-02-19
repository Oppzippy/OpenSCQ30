use crate::{
    packets::inbound::AmbientSoundModeUpdatePacket,
    state::{DeviceState, DeviceStateTransformer},
};

impl DeviceStateTransformer for AmbientSoundModeUpdatePacket {
    fn transform(&self, state: &DeviceState) -> DeviceState {
        state
            .with_ambient_sound_mode(self.ambient_sound_mode())
            .with_noise_canceling_mode(self.noise_canceling_mode())
    }
}
