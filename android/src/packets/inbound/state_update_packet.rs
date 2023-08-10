use crate::{EqualizerConfiguration, SoundModes};
use openscq30_lib::packets::inbound::state_update_packet::StateUpdatePacket as LibStateUpdatePacket;
use rifgen::rifgen_attr::generate_interface;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateUpdatePacket(LibStateUpdatePacket);

impl StateUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<StateUpdatePacket, String> {
        Err("do not construct directly".to_string())
    }

    #[generate_interface]
    pub fn feature_flags(&self) -> i32 {
        self.0.feature_flags.bits() as i32
    }

    #[generate_interface]
    pub fn sound_modes(&self) -> Option<SoundModes> {
        self.0.sound_modes.map(Into::into)
    }

    #[generate_interface]
    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.0.equalizer_configuration.into()
    }

    #[generate_interface]
    pub fn firmware_version(&self) -> Option<i32> {
        self.0
            .firmware_version
            .map(|firmware_version| firmware_version.number().into())
    }

    #[generate_interface]
    pub fn serial_number(&self) -> Option<String> {
        self.0
            .serial_number
            .as_ref()
            .map(|serial_number| serial_number.0.to_string())
    }
}

impl From<LibStateUpdatePacket> for StateUpdatePacket {
    fn from(packet: LibStateUpdatePacket) -> Self {
        Self(packet)
    }
}

impl From<StateUpdatePacket> for LibStateUpdatePacket {
    fn from(value: StateUpdatePacket) -> Self {
        value.0
    }
}
