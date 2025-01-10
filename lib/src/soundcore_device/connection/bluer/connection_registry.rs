use std::collections::HashSet;
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

use bluer::rfcomm::{Profile, ReqError};
use bluer::{Adapter, Address, DiscoveryFilter, DiscoveryTransport, Session};
use futures::StreamExt;
use macaddr::MacAddr6;
use tokio::select;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::{debug, debug_span, trace, warn, warn_span, Instrument};
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
        let session = runtime
            .spawn(async move { Session::new().await })
            .await
            .unwrap()?;
        Ok(Self {
            session,
            runtime,
            connections: Mutex::new(WeakValueHashMap::new()),
        })
    }

    async fn all_connected(&self) -> crate::Result<HashSet<GenericConnectionDescriptor>> {
        let session = self.session.to_owned();
        self.runtime
            .spawn(
                async move {
                    let adapter = session.default_adapter().await.map_err(|err| {
                        if err.kind == bluer::ErrorKind::NotFound {
                            crate::Error::BluetoothAdapterNotAvailable {
                                source: Some(Box::new(err)),
                            }
                        } else {
                            err.into()
                        }
                    })?;

                    // Needed to ensure GATT services are discovered
                    Self::ble_scan(&adapter).await?;

                    let device_addresses = adapter.device_addresses().await?;
                    let mut descriptors = HashSet::new();
                    for address in device_addresses {
                        if device_utils::is_mac_address_soundcore_device(address.into()) {
                            if let Some(descriptor) =
                                Self::address_to_descriptor(&adapter, address).await?
                            {
                                descriptors.insert(descriptor);
                            }
                        }
                    }
                    tracing::debug!("filtered down to {} descriptors", descriptors.len());
                    Ok(descriptors)
                }
                .instrument(debug_span!("BluerConnectionRegistry::all_connected")),
            )
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
            .spawn(
                async move {
                    let adapter = session.default_adapter().await.map_err(|err| {
                        if err.kind == bluer::ErrorKind::NotFound {
                            crate::Error::BluetoothAdapterNotAvailable {
                                source: Some(Box::new(err)),
                            }
                        } else {
                            err.into()
                        }
                    })?;

                    let device = match adapter.device(mac_address.into_array().into()) {
                        Ok(device) => device,
                        Err(err) => {
                            return match err.kind {
                                bluer::ErrorKind::NotFound => Ok(None),
                                _ => Err(crate::Error::from(err)),
                            }
                        }
                    };
                    device.connect().await?;
                    let uuids = device.uuids().await?.unwrap();
                    let uuid = uuids
                        .into_iter()
                        .find(device_utils::is_soundcore_vendor_rfcomm_uuid)
                        .unwrap_or(device_utils::RFCOMM_UUID);
                    debug!("using uuid {uuid} for RFCOMM");

                    let mut rfcomm_handle = session
                        .register_profile(Profile {
                            uuid,
                            ..Default::default()
                        })
                        .await?;

                    let timeout = Instant::now().checked_add(Duration::from_secs(5)).unwrap();
                    debug!("connecting");
                    let stream = loop {
                        select! {
                            res = async {
                                trace!("connect_profile");
                                device.connect_profile(&uuid).await
                            } => {
                                if let Err(err)=res{
                                    warn!("connect profile failed: {err:?}")
                                }
                                sleep(Duration::from_secs(3)).await;
                            }
                            req = rfcomm_handle.next() => {
                                let req = req.unwrap();
                                if req.device() == device.address() {
                                    debug!("accepting request from {}", req.device());
                                    break req.accept()?;
                                } else {
                                    debug!("rejecting request from {}", req.device());
                                    req.reject(ReqError::Rejected);
                                }
                            }
                            _ = tokio::time::sleep_until(timeout.into()) => {
                                return Err(crate::Error::TimedOut { action: "connect" })
                            }
                        }
                    };
                    debug!("connected");
                    BluerConnection::new(device, stream, handle).await.map(Some)
                }
                .instrument(warn_span!("BluerConnectionRegistry::new_connection")),
            )
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
