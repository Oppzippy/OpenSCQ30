use rifgen::rifgen_attr::generate_interface;

use crate::packets::{
    inbound::StateUpdatePacket,
    structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SoundcoreDeviceState {
    state: openscq30_lib::state::DeviceState,
}

impl SoundcoreDeviceState {
    #[generate_interface(constructor)]
    pub fn new_from_packet(packet: &StateUpdatePacket) -> SoundcoreDeviceState {
        packet.into()
    }

    #[generate_interface(constructor)]
    pub fn new(
        ambient_sound_mode: &AmbientSoundMode,
        noise_canceling_mode: &NoiseCancelingMode,
        equalizer_configuration: &EqualizerConfiguration,
    ) -> SoundcoreDeviceState {
        Self {
            state: openscq30_lib::state::DeviceState {
                ambient_sound_mode: ambient_sound_mode.to_owned().into(),
                noise_canceling_mode: noise_canceling_mode.to_owned().into(),
                transparency_mode: Default::default(),
                custom_noise_canceling: Default::default(),
                equalizer_configuration: equalizer_configuration.to_owned().into(),
            },
        }
    }

    #[generate_interface]
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.state.ambient_sound_mode.into()
    }

    #[generate_interface]
    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.state.noise_canceling_mode.into()
    }

    #[generate_interface]
    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.state.equalizer_configuration.into()
    }

    #[generate_interface]
    pub fn with_ambient_sound_mode(
        &self,
        ambient_sound_mode: &AmbientSoundMode,
    ) -> SoundcoreDeviceState {
        openscq30_lib::state::DeviceState {
            ambient_sound_mode: ambient_sound_mode.to_owned().into(),
            ..self.state
        }
        .into()
    }

    #[generate_interface]
    pub fn with_noise_canceling_mode(
        &self,
        noise_canceling_mode: &NoiseCancelingMode,
    ) -> SoundcoreDeviceState {
        openscq30_lib::state::DeviceState {
            noise_canceling_mode: noise_canceling_mode.to_owned().into(),
            ..self.state
        }
        .into()
    }

    #[generate_interface]
    pub fn with_equalizer_configuration(
        &self,
        equalizer_configuration: &EqualizerConfiguration,
    ) -> SoundcoreDeviceState {
        openscq30_lib::state::DeviceState {
            equalizer_configuration: equalizer_configuration.to_owned().into(),
            ..self.state
        }
        .into()
    }
}

impl From<&StateUpdatePacket> for SoundcoreDeviceState {
    fn from(packet: &StateUpdatePacket) -> Self {
        Self {
            state: Into::<openscq30_lib::packets::inbound::StateUpdatePacket>::into(
                packet.to_owned(),
            )
            .into(),
        }
    }
}

impl From<openscq30_lib::state::DeviceState> for SoundcoreDeviceState {
    fn from(state: openscq30_lib::state::DeviceState) -> Self {
        Self { state }
    }
}
