use crate::soundcore_bluetooth::traits::SoundcoreDeviceConnectionDescriptor;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StubSoundcoreDeviceConnectionDescriptor {
    name: String,
    mac_address: String,
}

impl StubSoundcoreDeviceConnectionDescriptor {
    pub fn new(name: String, mac_address: String) -> Self {
        Self { name, mac_address }
    }
}

impl SoundcoreDeviceConnectionDescriptor for StubSoundcoreDeviceConnectionDescriptor {
    fn name(&self) -> &String {
        &self.name
    }

    fn mac_address(&self) -> &String {
        &self.mac_address
    }
}
