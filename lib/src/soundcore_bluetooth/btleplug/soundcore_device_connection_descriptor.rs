use crate::soundcore_bluetooth::traits::SoundcoreDeviceConnectionDescriptor;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BtlePlugSoundcoreDeviceConnectionDescriptor {
    name: String,
    mac_address: String,
}

impl BtlePlugSoundcoreDeviceConnectionDescriptor {
    pub fn new(name: String, mac_address: String) -> Self {
        Self { name, mac_address }
    }
}

impl SoundcoreDeviceConnectionDescriptor for BtlePlugSoundcoreDeviceConnectionDescriptor {
    fn name(&self) -> &String {
        &self.name
    }

    fn mac_address(&self) -> &String {
        &self.mac_address
    }
}
