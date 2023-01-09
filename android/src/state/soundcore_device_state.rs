use rifgen::rifgen_attr::generate_interface;

use crate::packets::{
    inbound::StateUpdatePacket,
    structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SoundcoreDeviceState {
    state: openscq30_lib::state::SoundcoreDeviceState,
}

impl SoundcoreDeviceState {
    #[generate_interface(constructor)]
    pub fn new(packet: &StateUpdatePacket) -> SoundcoreDeviceState {
        packet.into()
    }

    #[generate_interface]
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.state.ambient_sound_mode().into()
    }

    #[generate_interface]
    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.state.noise_canceling_mode().into()
    }

    #[generate_interface]
    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.state.equalizer_configuration().into()
    }

    #[generate_interface]
    pub fn with_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> SoundcoreDeviceState {
        self.state
            .with_ambient_sound_mode(ambient_sound_mode.into())
            .into()
    }

    #[generate_interface]
    pub fn with_noise_canceling_mode(
        &self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> SoundcoreDeviceState {
        self.state
            .with_noise_canceling_mode(noise_canceling_mode.into())
            .into()
    }

    #[generate_interface]
    pub fn with_equalizer_configuration(
        &self,
        equalizer_configuration: EqualizerConfiguration,
    ) -> SoundcoreDeviceState {
        self.state
            .with_equalizer_configuration(equalizer_configuration.into())
            .into()
    }
}

impl From<&StateUpdatePacket> for SoundcoreDeviceState {
    fn from(packet: &StateUpdatePacket) -> Self {
        Self {
            state: Into::<openscq30_lib::packets::inbound::StateUpdatePacket>::into(*packet).into(),
        }
    }
}

impl From<openscq30_lib::state::SoundcoreDeviceState> for SoundcoreDeviceState {
    fn from(state: openscq30_lib::state::SoundcoreDeviceState) -> Self {
        Self { state }
    }
}
