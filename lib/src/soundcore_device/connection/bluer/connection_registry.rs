use std::collections::HashSet;
use std::sync::{Arc, Weak};
use std::time::Duration;

use bluer::{DiscoveryFilter, DiscoveryTransport, Session};
use futures::StreamExt;
use macaddr::MacAddr6;
use tokio::sync::Mutex;
use weak_table::weak_value_hash_map::Entry;
use weak_table::WeakValueHashMap;

use crate::api::connection::{ConnectionRegistry, GenericConnectionDescriptor};
use crate::device_utils;
use crate::soundcore_device::connection::btleplug::RuntimeOrHandle;

use super::connection::BluerConnection;

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
                adapter
                    .set_discovery_filter(DiscoveryFilter {
                        uuids: HashSet::from_iter(device_utils::service_uuids()),
                        transport: DiscoveryTransport::Le,
                        ..Default::default()
                    })
                    .await?;

                let discover = adapter.discover_devices().await?;

                let events = discover
                    .take_until(tokio::time::sleep(Duration::from_secs(1)))
                    .collect::<Vec<_>>()
                    .await;

                let mut device_addresses = HashSet::new();
                for event in events {
                    match event {
                        bluer::AdapterEvent::DeviceAdded(address) => {
                            device_addresses.insert(address);
                        }
                        bluer::AdapterEvent::DeviceRemoved(address) => {
                            device_addresses.remove(&address);
                        }
                        bluer::AdapterEvent::PropertyChanged(_) => (),
                    }
                }

                let mut descriptors = HashSet::new();
                for address in device_addresses {
                    let device = adapter.device(address)?;
                    if device.is_connected().await? {
                        let mut has_service = false;
                        for service in device.services().await? {
                            if device_utils::is_soundcore_service_uuid(&service.uuid().await?) {
                                has_service = true;
                                break;
                            }
                        }
                        if has_service {
                            descriptors.insert(GenericConnectionDescriptor::new(
                                device.name().await?.unwrap_or_default(),
                                device.address().0.into(),
                            ));
                        }
                    }
                }
                Ok(descriptors)
            })
            .await
            .unwrap()
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
