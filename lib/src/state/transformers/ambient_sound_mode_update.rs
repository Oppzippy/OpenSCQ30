use crate::{
    packets::inbound::AmbientSoundModeUpdatePacket,
    state::{SoundcoreDeviceState, SoundcoreDeviceStateTransformer},
};

impl SoundcoreDeviceStateTransformer for AmbientSoundModeUpdatePacket {
    fn transform(&self, state: &SoundcoreDeviceState) -> SoundcoreDeviceState {
        state
            .with_ambient_sound_mode(self.ambient_sound_mode())
            .with_noise_canceling_mode(self.noise_canceling_mode())
    }
}
