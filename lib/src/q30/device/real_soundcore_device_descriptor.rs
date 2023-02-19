use crate::api::{
    connection::SoundcoreDeviceConnectionDescriptor, device::SoundcoreDeviceDescriptor,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RealSoundcoreDeviceDescriptor<T>
where
    T: SoundcoreDeviceConnectionDescriptor + Send + Sync,
{
    inner: T,
}

impl<T> RealSoundcoreDeviceDescriptor<T>
where
    T: SoundcoreDeviceConnectionDescriptor + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> SoundcoreDeviceDescriptor for RealSoundcoreDeviceDescriptor<T>
where
    T: SoundcoreDeviceConnectionDescriptor + Send + Sync,
{
    fn name(&self) -> &String {
        self.inner.name()
    }

    fn mac_address(&self) -> &String {
        self.inner.mac_address()
    }
}
