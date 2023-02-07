use crate::packets::{
    inbound::StateUpdatePacket,
    structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SoundcoreDeviceState {
    ambient_sound_mode: AmbientSoundMode,
    noise_canceling_mode: NoiseCancelingMode,
    equalizer_configuration: EqualizerConfiguration,
}

impl SoundcoreDeviceState {
    pub fn new(
        ambient_sound_mode: &AmbientSoundMode,
        noise_canceling_mode: &NoiseCancelingMode,
        equalizer_configuration: &EqualizerConfiguration,
    ) -> Self {
        Self {
            ambient_sound_mode: ambient_sound_mode.to_owned(),
            noise_canceling_mode: noise_canceling_mode.to_owned(),
            equalizer_configuration: equalizer_configuration.to_owned(),
        }
    }

    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.ambient_sound_mode
    }

    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.noise_canceling_mode
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.equalizer_configuration
    }

    pub fn with_ambient_sound_mode(&self, ambient_sound_mode: AmbientSoundMode) -> Self {
        Self {
            ambient_sound_mode,
            noise_canceling_mode: self.noise_canceling_mode,
            equalizer_configuration: self.equalizer_configuration,
        }
    }

    pub fn with_noise_canceling_mode(&self, noise_canceling_mode: NoiseCancelingMode) -> Self {
        Self {
            ambient_sound_mode: self.ambient_sound_mode,
            noise_canceling_mode,
            equalizer_configuration: self.equalizer_configuration,
        }
    }

    pub fn with_equalizer_configuration(
        &self,
        equalizer_configuration: EqualizerConfiguration,
    ) -> Self {
        Self {
            ambient_sound_mode: self.ambient_sound_mode,
            noise_canceling_mode: self.noise_canceling_mode,
            equalizer_configuration,
        }
    }
}

impl From<&StateUpdatePacket> for SoundcoreDeviceState {
    fn from(packet: &StateUpdatePacket) -> Self {
        Self {
            ambient_sound_mode: packet.ambient_sound_mode(),
            noise_canceling_mode: packet.noise_canceling_mode(),
            equalizer_configuration: packet.equalizer_configuration(),
        }
    }
}

impl From<StateUpdatePacket> for SoundcoreDeviceState {
    fn from(packet: StateUpdatePacket) -> Self {
        (&packet).into()
    }
}
