use crate::{
    AgeRange, CustomHearId, DeviceFeatureFlags, EqualizerConfiguration, FirmwareVersion, Gender,
    SoundModes,
};
use openscq30_lib::packets::{
    inbound::state_update_packet::StateUpdatePacket as LibStateUpdatePacket, structures::HearId,
};
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateUpdatePacket(LibStateUpdatePacket);

impl StateUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<StateUpdatePacket, String> {
        Err("do not construct directly".to_string())
    }

    #[generate_interface]
    pub fn feature_flags(&self) -> DeviceFeatureFlags {
        self.0.feature_flags.into()
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
    pub fn firmware_version(&self) -> Option<FirmwareVersion> {
        self.0.firmware_version.map(Into::into)
    }

    #[generate_interface]
    pub fn serial_number(&self) -> Option<String> {
        self.0
            .serial_number
            .as_ref()
            .map(|serial_number| serial_number.0.to_string())
    }

    #[generate_interface]
    pub fn age_range(&self) -> Option<AgeRange> {
        self.0.age_range.map(Into::into)
    }

    #[generate_interface]
    pub fn dynamic_range_compression_min_firmware_version(&self) -> Option<FirmwareVersion> {
        self.0
            .dynamic_range_compression_min_firmware_version
            .map(Into::into)
    }

    #[generate_interface]
    pub fn custom_hear_id(&self) -> Option<CustomHearId> {
        if let Some(HearId::Custom(custom_hear_id)) = self.0.custom_hear_id {
            Some(custom_hear_id.into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn gender(&self) -> Option<Gender> {
        self.0.gender.map(Into::into)
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
