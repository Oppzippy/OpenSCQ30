use std::{
    marker::PhantomData,
    rc::{Rc, Weak},
};

use macaddr::MacAddr6;
use tokio::sync::Mutex;
use weak_table::{weak_value_hash_map::Entry, WeakValueHashMap};

use crate::{
    api::{
        connection::{ConnectionDescriptor, ConnectionRegistry},
        device::{DeviceRegistry, GenericDeviceDescriptor},
    },
    device_utils,
    futures::Futures,
};

use super::soundcore_device::SoundcoreDevice;

pub struct SoundcoreDeviceRegistry<RegistryType, FuturesType>
where
    RegistryType: ConnectionRegistry,
    FuturesType: Futures,
{
    conneciton_registry: RegistryType,
    devices: Mutex<
        WeakValueHashMap<
            MacAddr6,
            Weak<SoundcoreDevice<RegistryType::ConnectionType, FuturesType>>,
        >,
    >,
    futures: PhantomData<FuturesType>,
}

impl<RegistryType, FuturesType> SoundcoreDeviceRegistry<RegistryType, FuturesType>
where
    RegistryType: ConnectionRegistry,
    FuturesType: Futures,
{
    pub async fn new(connection_registry: RegistryType) -> crate::Result<Self> {
        Ok(Self {
            conneciton_registry: connection_registry,
            devices: Mutex::new(WeakValueHashMap::new()),
            futures: PhantomData,
        })
    }

    async fn new_device(
        &self,
        mac_address: MacAddr6,
    ) -> crate::Result<Option<SoundcoreDevice<RegistryType::ConnectionType, FuturesType>>> {
        let connection = self.conneciton_registry.connection(mac_address).await?;

        if let Some(connection) = connection {
            SoundcoreDevice::new(connection).await.map(Option::Some)
        } else {
            Ok(None)
        }
    }
}

impl<RegistryType, FuturesType> DeviceRegistry
    for SoundcoreDeviceRegistry<RegistryType, FuturesType>
where
    RegistryType: ConnectionRegistry,
    FuturesType: Futures,
{
    type DeviceType = SoundcoreDevice<RegistryType::ConnectionType, FuturesType>;
    type DescriptorType = GenericDeviceDescriptor;

    async fn device_descriptors(&self) -> crate::Result<Vec<Self::DescriptorType>> {
        let inner_descriptors = self.conneciton_registry.connection_descriptors().await?;
        let descriptors = inner_descriptors
            .into_iter()
            .filter(|descriptor| {
                device_utils::is_mac_address_soundcore_device(descriptor.mac_address())
            })
            .map(GenericDeviceDescriptor::from)
            .collect::<Vec<_>>();
        Ok(descriptors)
    }

    async fn device(&self, mac_address: MacAddr6) -> crate::Result<Option<Rc<Self::DeviceType>>> {
        match self.devices.lock().await.entry(mac_address.to_owned()) {
            Entry::Occupied(entry) => {
                tracing::debug!("{mac_address} is cached");
                Ok(Some(entry.get().to_owned()))
            }
            Entry::Vacant(entry) => {
                tracing::debug!("{mac_address} is not cached");
                if let Some(device) = self.new_device(mac_address).await? {
                    let device = Rc::new(device);
                    entry.insert(device.to_owned());
                    Ok(Some(device))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use macaddr::MacAddr6;
    use tokio::sync::mpsc;

    use crate::{
        api::{
            connection::{ConnectionDescriptor, GenericConnectionDescriptor},
            device::{Device, DeviceDescriptor, DeviceRegistry},
        },
        futures::TokioFutures,
        stub::connection::{StubConnection, StubConnectionRegistry},
    };

    use super::SoundcoreDeviceRegistry;

    #[tokio::test]
    async fn test_device_descriptors() {
        let descriptor = GenericConnectionDescriptor::new(
            "Stub Device",
            // Must start with soundcore prefix
            MacAddr6::new(0xAC, 0x12, 0x2F, 0x01, 0x02, 0x03),
        );
        let device = Arc::new(StubConnection::new());
        let devices = HashMap::from([(descriptor, device)]);
        let connection_registry = StubConnectionRegistry::new(devices.to_owned());
        let device_registry = SoundcoreDeviceRegistry::<_, TokioFutures>::new(connection_registry)
            .await
            .unwrap();

        let descriptors = device_registry.device_descriptors().await.unwrap();
        let descriptor_values = descriptors
            .iter()
            .map(|descriptor| (descriptor.name(), descriptor.mac_address()))
            .collect::<Vec<_>>();

        assert_eq!(
            vec![(
                "Stub Device",
                MacAddr6::new(0xAC, 0x12, 0x2F, 0x01, 0x02, 0x03),
            )],
            descriptor_values,
        );
    }

    #[tokio::test]
    async fn test_get_device() {
        let descriptor = GenericConnectionDescriptor::new(
            "Stub Device",
            MacAddr6::new(0x00, 0x11, 0x22, 0x33, 0x44, 0x55),
        );
        let device = Arc::new(StubConnection::new());
        let (sender, receiver) = mpsc::channel(1);
        sender
            .send(vec![
                0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0x01, 0x00, 0x3c,
                0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
                0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x35,
            ])
            .await
            .unwrap();
        device.set_inbound_packets_channel(Ok(receiver)).await;
        device
            .set_mac_address_return(Ok(descriptor.mac_address()))
            .await;
        device
            .set_name_return(Ok(descriptor.name().to_owned()))
            .await;
        for _ in 0..10 {
            device.push_write_return(Ok(())).await;
        }

        let devices = HashMap::from([(descriptor, device)]);
        let connection_registry = StubConnectionRegistry::new(devices.to_owned());
        let device_registry = SoundcoreDeviceRegistry::<_, TokioFutures>::new(connection_registry)
            .await
            .unwrap();

        let device = device_registry
            .device(MacAddr6::new(0x00, 0x11, 0x22, 0x33, 0x44, 0x55))
            .await
            .expect("must not fail to create device")
            .expect("device must exist");

        assert_eq!(
            MacAddr6::new(0x00, 0x11, 0x22, 0x33, 0x44, 0x55),
            device.mac_address().await.unwrap()
        );
    }

    #[tokio::test]
    async fn test_get_device_but_it_doesnt_exist() {
        let descriptor = GenericConnectionDescriptor::new(
            "Stub Device",
            MacAddr6::new(0x00, 0x11, 0x22, 0x33, 0x44, 0x55),
        );
        let device = Arc::new(StubConnection::new());

        let devices = HashMap::from([(descriptor, device)]);
        let connection_registry = StubConnectionRegistry::new(devices.to_owned());
        let device_registry = SoundcoreDeviceRegistry::<_, TokioFutures>::new(connection_registry)
            .await
            .unwrap();

        let maybe_device = device_registry
            .device(MacAddr6::new(0x00, 0x00, 0x22, 0x33, 0x44, 0x55))
            .await
            .expect("must not fail to create device");

        assert_eq!(true, maybe_device.is_none());
    }
}
