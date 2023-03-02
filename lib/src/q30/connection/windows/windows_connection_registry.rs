use std::{collections::HashSet, str::FromStr, sync::Arc, thread};

use async_trait::async_trait;
use macaddr::MacAddr6;
use tokio::sync::oneshot;
use windows::Devices::Bluetooth::{self, BluetoothConnectionStatus, BluetoothDevice};

use crate::api::connection::ConnectionRegistry;

use super::{WindowsConnection, WindowsConnectionDescriptor};

pub struct WindowsConnectionRegistry {}

impl WindowsConnectionRegistry {
    pub async fn new() -> crate::Result<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl ConnectionRegistry for WindowsConnectionRegistry {
    type ConnectionType = WindowsConnection;
    type DescriptorType = WindowsConnectionDescriptor;

    async fn connection_descriptors(&self) -> crate::Result<HashSet<Self::DescriptorType>> {
        let (result_sender, result_receiver) = oneshot::channel();
        thread::spawn(move || {
            let result = (|| {
                let devices =
                    windows::Devices::Enumeration::DeviceInformation::FindAllAsyncAqsFilter(
                        &Bluetooth::BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
                            BluetoothConnectionStatus::Connected,
                        )?,
                    )?
                    .get()?;
                let descriptors = devices
                    .into_iter()
                    .map(|device| {
                        let id = device.Id()?;
                        let bluetooth_device = BluetoothDevice::FromIdAsync(&id)?.get()?;
                        let addr = bluetooth_device.BluetoothAddress()?;
                        let bytes: [u8; 8] = addr.to_le_bytes();
                        let mac_address = MacAddr6::new(
                            bytes[5], bytes[4], bytes[3], bytes[2], bytes[1], bytes[0],
                        );

                        // TODO properly convert u64 to mac address
                        Ok(WindowsConnectionDescriptor::new(
                            bluetooth_device.Name()?.to_string(),
                            mac_address.to_string(),
                            addr,
                        )) as crate::Result<WindowsConnectionDescriptor>
                    })
                    .filter_map(|result| match result {
                        Ok(descriptor) => Some(descriptor),
                        Err(err) => {
                            tracing::warn!("error creating device descriptor: {}", err);
                            None
                        }
                    })
                    .collect();
                Ok(descriptors) as crate::Result<HashSet<Self::DescriptorType>>
            })();
            result_sender.send(result).unwrap();
        });
        result_receiver.await.map_err(|err| crate::Error::Other {
            source: Box::new(err),
        })?
    }

    async fn connection(
        &self,
        mac_address: &str,
    ) -> crate::Result<Option<Arc<Self::ConnectionType>>> {
        let mac_address = MacAddr6::from_str(mac_address).unwrap();
        // TODO temporary until we swap out mac address parameter for descriptor?
        let id = mac_address
            .into_array()
            .into_iter()
            .enumerate()
            .fold(0 as u64, |acc, (i, value)| {
                acc | ((value as u64) << (5 - i) * 8)
            });

        Ok(WindowsConnection::new(id).await?.map(Arc::new))
    }
}
