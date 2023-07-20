use crate::packets::{
    inbound::StateUpdatePacket,
    structures::{
        AmbientSoundMode, CustomNoiseCanceling, EqualizerConfiguration, NoiseCancelingMode,
        TransparencyMode,
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DeviceState {
    pub ambient_sound_mode: AmbientSoundMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub transparency_mode: TransparencyMode,
    pub custom_noise_canceling: CustomNoiseCanceling,
    pub equalizer_configuration: EqualizerConfiguration,
}

impl From<&StateUpdatePacket> for DeviceState {
    fn from(packet: &StateUpdatePacket) -> Self {
        Self {
            ambient_sound_mode: packet.ambient_sound_mode(),
            noise_canceling_mode: packet.noise_canceling_mode(),
            transparency_mode: packet.transparency_mode(),
            custom_noise_canceling: packet.custom_noise_canceling(),
            equalizer_configuration: packet.equalizer_configuration(),
        }
    }
}

impl From<StateUpdatePacket> for DeviceState {
    fn from(packet: StateUpdatePacket) -> Self {
        (&packet).into()
    }
}
