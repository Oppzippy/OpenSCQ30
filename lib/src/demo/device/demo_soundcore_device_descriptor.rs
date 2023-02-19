use crate::api::device::SoundcoreDeviceDescriptor;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DemoSoundcoreDeviceDescriptor {
    name: String,
    mac_address: String,
}

impl DemoSoundcoreDeviceDescriptor {
    pub fn new(name: String, mac_address: String) -> Self {
        Self { name, mac_address }
    }
}

impl SoundcoreDeviceDescriptor for DemoSoundcoreDeviceDescriptor {
    fn name(&self) -> &String {
        &self.name
    }

    fn mac_address(&self) -> &String {
        &self.mac_address
    }
}
