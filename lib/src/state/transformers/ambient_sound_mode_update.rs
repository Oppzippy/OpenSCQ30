use crate::{
    packets::inbound::SoundModeUpdatePacket,
    state::{DeviceState, DeviceStateTransformer},
};

impl DeviceStateTransformer for SoundModeUpdatePacket {
    fn transform(&self, state: &DeviceState) -> DeviceState {
        return DeviceState {
            ambient_sound_mode: self.ambient_sound_mode(),
            noise_canceling_mode: self.noise_canceling_mode(),
            ..*state
        };
    }
}
