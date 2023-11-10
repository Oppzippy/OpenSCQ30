use openscq30_lib::device_profiles::{
    DeviceProfile as LibDeviceProfile, NoiseCancelingModeType as LibNoiseCancelingModeType,
    SoundModeProfile as LibSoundModeProfile, TransparencyModeType as LibTransparencyModeType,
};
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

use crate::FirmwareVersion;

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct DeviceProfile(LibDeviceProfile);

impl DeviceProfile {
    #[generate_interface(constructor)]
    pub fn new(
        sound_mode: Option<SoundModeProfile>,
        has_hear_id: bool,
        num_equalizer_channels: usize,
        num_equalizer_bands: usize,
        has_dynamic_range_compression: bool,
        has_custom_button_model: bool,
        has_wear_detection: bool,
        has_touch_tone: bool,
        has_auto_power_off: bool,
        dynamic_range_compression_min_firmware_version: Option<FirmwareVersion>,
    ) -> DeviceProfile {
        DeviceProfile(LibDeviceProfile {
            sound_mode: sound_mode.map(Into::into),
            has_hear_id,
            num_equalizer_channels,
            num_equalizer_bands,
            has_dynamic_range_compression,
            has_custom_button_model,
            has_wear_detection,
            has_touch_tone,
            has_auto_power_off,
            dynamic_range_compression_min_firmware_version:
                dynamic_range_compression_min_firmware_version.map(Into::into),
        })
    }

    #[generate_interface]
    pub fn sound_mode(&self) -> Option<SoundModeProfile> {
        self.0.sound_mode.map(Into::into)
    }

    #[generate_interface]
    pub fn has_hear_id(&self) -> bool {
        self.0.has_hear_id
    }

    #[generate_interface]
    pub fn num_equalizer_channels(&self) -> usize {
        self.0.num_equalizer_channels
    }

    #[generate_interface]
    pub fn num_equalizer_bands(&self) -> usize {
        self.0.num_equalizer_bands
    }

    #[generate_interface]
    pub fn has_dynamic_range_compression(&self) -> bool {
        self.0.has_dynamic_range_compression
    }

    #[generate_interface]
    pub fn has_custom_button_model(&self) -> bool {
        self.0.has_custom_button_model
    }

    #[generate_interface]
    pub fn has_wear_detection(&self) -> bool {
        self.0.has_wear_detection
    }

    #[generate_interface]
    pub fn has_touch_tone(&self) -> bool {
        self.0.has_touch_tone
    }

    #[generate_interface]
    pub fn has_auto_power_off(&self) -> bool {
        self.0.has_auto_power_off
    }

    #[generate_interface]
    pub fn dynamic_range_compression_min_firmware_version(&self) -> Option<FirmwareVersion> {
        self.0
            .dynamic_range_compression_min_firmware_version
            .map(Into::into)
    }
}

impl From<LibDeviceProfile> for DeviceProfile {
    fn from(value: LibDeviceProfile) -> Self {
        Self(value)
    }
}

impl From<DeviceProfile> for LibDeviceProfile {
    fn from(value: DeviceProfile) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SoundModeProfile(LibSoundModeProfile);

impl SoundModeProfile {
    #[generate_interface(constructor)]
    pub fn new(
        noise_canceling_mode_type: NoiseCancelingModeType,
        transparency_mode_type: TransparencyModeType,
    ) -> SoundModeProfile {
        Self(LibSoundModeProfile {
            noise_canceling_mode_type: noise_canceling_mode_type.into(),
            transparency_mode_type: transparency_mode_type.into(),
        })
    }

    #[generate_interface]
    pub fn noise_canceling_mode_type(&self) -> NoiseCancelingModeType {
        self.0.noise_canceling_mode_type.into()
    }

    #[generate_interface]
    pub fn transparency_mode_type(&self) -> TransparencyModeType {
        self.0.transparency_mode_type.into()
    }
}

impl From<LibSoundModeProfile> for SoundModeProfile {
    fn from(value: LibSoundModeProfile) -> Self {
        Self(value)
    }
}

impl From<SoundModeProfile> for LibSoundModeProfile {
    fn from(value: SoundModeProfile) -> Self {
        value.0
    }
}

#[generate_interface]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NoiseCancelingModeType {
    None,
    Basic,
    Custom,
}

impl From<LibNoiseCancelingModeType> for NoiseCancelingModeType {
    fn from(value: LibNoiseCancelingModeType) -> Self {
        match value {
            LibNoiseCancelingModeType::None => NoiseCancelingModeType::None,
            LibNoiseCancelingModeType::Basic => NoiseCancelingModeType::Basic,
            LibNoiseCancelingModeType::Custom => NoiseCancelingModeType::Custom,
        }
    }
}

impl From<NoiseCancelingModeType> for LibNoiseCancelingModeType {
    fn from(value: NoiseCancelingModeType) -> Self {
        match value {
            NoiseCancelingModeType::None => LibNoiseCancelingModeType::None,
            NoiseCancelingModeType::Basic => LibNoiseCancelingModeType::Basic,
            NoiseCancelingModeType::Custom => LibNoiseCancelingModeType::Custom,
        }
    }
}

#[generate_interface]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransparencyModeType {
    Basic,
    Custom,
}

impl From<LibTransparencyModeType> for TransparencyModeType {
    fn from(value: LibTransparencyModeType) -> Self {
        match value {
            LibTransparencyModeType::Basic => TransparencyModeType::Basic,
            LibTransparencyModeType::Custom => TransparencyModeType::Custom,
        }
    }
}

impl From<TransparencyModeType> for LibTransparencyModeType {
    fn from(value: TransparencyModeType) -> Self {
        match value {
            TransparencyModeType::Basic => LibTransparencyModeType::Basic,
            TransparencyModeType::Custom => LibTransparencyModeType::Custom,
        }
    }
}
