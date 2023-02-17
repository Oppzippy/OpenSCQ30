use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use async_trait::async_trait;

use crate::soundcore_bluetooth::traits::{
    SoundcoreDeviceConnectionDescriptor, SoundcoreDeviceConnectionError,
    SoundcoreDeviceConnectionRegistry,
};

use super::{StubSoundcoreDeviceConnection, StubSoundcoreDeviceConnectionDescriptor};

#[derive(Debug)]
pub struct StubSoundcoreDeviceConnectionRegistry {
    connections: HashMap<
        StubSoundcoreDeviceConnectionDescriptor,
        Arc<<Self as SoundcoreDeviceConnectionRegistry>::DeviceConnectionType>,
    >,
}

impl StubSoundcoreDeviceConnectionRegistry {
    pub fn new(
        connections: HashMap<
            StubSoundcoreDeviceConnectionDescriptor,
            Arc<<Self as SoundcoreDeviceConnectionRegistry>::DeviceConnectionType>,
        >,
    ) -> Self {
        Self { connections }
    }
}

#[async_trait]
impl SoundcoreDeviceConnectionRegistry for StubSoundcoreDeviceConnectionRegistry {
    type DeviceConnectionType = StubSoundcoreDeviceConnection;
    type DescriptorType = StubSoundcoreDeviceConnectionDescriptor;

    async fn connection_descriptors(
        &self,
    ) -> Result<HashSet<Self::DescriptorType>, SoundcoreDeviceConnectionError> {
        Ok(self.connections.keys().cloned().collect())
    }

    async fn connection(
        &self,
        mac_address: &str,
    ) -> Result<Option<Arc<Self::DeviceConnectionType>>, SoundcoreDeviceConnectionError> {
        Ok(self
            .connections
            .iter()
            .find(|(descriptor, _connection)| descriptor.mac_address() == mac_address)
            .map(|(_descriptor, connection)| connection)
            .cloned())
    }
}
