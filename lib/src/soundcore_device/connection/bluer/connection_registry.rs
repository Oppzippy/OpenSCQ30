use std::collections::HashSet;
use std::sync::{Arc, Weak};
use std::time::Duration;

use bluer::{Adapter, Address, DiscoveryFilter, DiscoveryTransport, Session};
use futures::StreamExt;
use macaddr::MacAddr6;
use tokio::sync::Mutex;
use weak_table::weak_value_hash_map::Entry;
use weak_table::WeakValueHashMap;

use crate::api::connection::{ConnectionRegistry, GenericConnectionDescriptor};
use crate::device_utils;

use super::connection::BluerConnection;
use super::RuntimeOrHandle;

pub struct BluerConnectionRegistry {
    runtime: RuntimeOrHandle,
    session: Session,
    connections: Mutex<WeakValueHashMap<MacAddr6, Weak<BluerConnection>>>,
}

impl BluerConnectionRegistry {
    pub async fn new(runtime: RuntimeOrHandle) -> crate::Result<Self> {
        Ok(Self {
            session: runtime
                .spawn(async move { Session::new().await })
                .await
                .unwrap()?,
            runtime,
            connections: Mutex::new(WeakValueHashMap::new()),
        })
    }

    #[tracing::instrument(skip(self))]
    async fn all_connected(&self) -> crate::Result<HashSet<GenericConnectionDescriptor>> {
        let session = self.session.to_owned();
        self.runtime
            .spawn(async move {
                let adapter = session.default_adapter().await?;
                tracing::debug!("starting scan on adapter {}", adapter.name());
                let device_addresses = Self::ble_scan(&adapter).await?;
                tracing::debug!("discovered {} devices", device_addresses.len());
                let mut descriptors = HashSet::new();
                for address in device_addresses {
                    if let Some(descriptor) = Self::address_to_descriptor(&adapter, address).await?
                    {
                        descriptors.insert(descriptor);
                    }
                }
                tracing::debug!("filtered down to {} descriptors", descriptors.len());
                Ok(descriptors)
            })
            .await
            .unwrap()
    }

    /// Scans for connected BLE devices and attempt to filter out devices without the service UUID.
    /// Not guaranteed to filter out all devices if another process is scanning at the same time.
    async fn ble_scan(adapter: &Adapter) -> crate::Result<HashSet<Address>> {
        adapter
            .set_discovery_filter(DiscoveryFilter {
                transport: DiscoveryTransport::Le,
                ..Default::default()
            })
            .await?;

        let discover = adapter.discover_devices().await?;

        let device_addresses = discover
            .take_until(tokio::time::sleep(Duration::from_secs(1)))
            .filter_map(|event| async move {
                match event {
                    bluer::AdapterEvent::DeviceAdded(address)
                        if device_utils::is_mac_address_soundcore_device(address.into()) =>
                    {
                        Some(address)
                    }
                    _ => None,
                }
            })
            .collect::<HashSet<_>>()
            .await;

        Ok(device_addresses)
    }

    /// Filters out devices that are not connected and returns descriptors
    async fn address_to_descriptor(
        adapter: &Adapter,
        address: Address,
    ) -> crate::Result<Option<GenericConnectionDescriptor>> {
        let device = adapter.device(address)?;
        if device.is_connected().await? {
            Ok(Some(GenericConnectionDescriptor::new(
                device.name().await?.unwrap_or_default(),
                address.into(),
            )))
        } else {
            Ok(None)
        }
    }

    async fn new_connection(
        &self,
        mac_address: MacAddr6,
    ) -> crate::Result<Option<BluerConnection>> {
        let handle = self.runtime.handle();
        let session = self.session.to_owned();
        self.runtime
            .spawn(async move {
                let adapter = session.default_adapter().await?;
                match adapter.device(mac_address.into_array().into()) {
                    Ok(device) => Ok(Some(BluerConnection::new(device, handle).await?)),
                    Err(err) => match err.kind {
                        bluer::ErrorKind::NotFound => Ok(None),
                        _ => Err(crate::Error::from(err)),
                    },
                }
            })
            .await
            .unwrap()
    }
}

impl ConnectionRegistry for BluerConnectionRegistry {
    type ConnectionType = BluerConnection;
    type DescriptorType = GenericConnectionDescriptor;

    async fn connection_descriptors(&self) -> crate::Result<HashSet<Self::DescriptorType>> {
        self.all_connected().await
    }

    async fn connection(
        &self,
        mac_address: MacAddr6,
    ) -> crate::Result<Option<Arc<Self::ConnectionType>>> {
        match self.connections.lock().await.entry(mac_address.to_owned()) {
            Entry::Occupied(entry) => Ok(Some(entry.get().to_owned())),
            Entry::Vacant(entry) => {
                if let Some(connection) = self.new_connection(mac_address).await? {
                    let connection = Arc::new(connection);
                    entry.insert(connection.to_owned());
                    Ok(Some(connection))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
