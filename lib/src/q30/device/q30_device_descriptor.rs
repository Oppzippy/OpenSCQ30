use crate::api::{connection::ConnectionDescriptor, device::DeviceDescriptor};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Q30DeviceDescriptor<T>
where
    T: ConnectionDescriptor + Send + Sync,
{
    inner: T,
}

impl<T> Q30DeviceDescriptor<T>
where
    T: ConnectionDescriptor + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> DeviceDescriptor for Q30DeviceDescriptor<T>
where
    T: ConnectionDescriptor + Send + Sync,
{
    fn name(&self) -> &String {
        self.inner.name()
    }

    fn mac_address(&self) -> &String {
        self.inner.mac_address()
    }
}
